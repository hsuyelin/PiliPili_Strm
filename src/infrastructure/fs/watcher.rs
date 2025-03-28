use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }
};

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::{sleep, Duration},
};
use tokio_stream::{
    StreamExt,
    wrappers::ReceiverStream,
};
use ctrlc;

use crate::{error_log, info_log, warn_log};
use super::{
    watcher_state::WatcherState,
    watcher_callback::FileWatcherCallback,
    watchable::FileWatchable,
    path_helper::PathHelper,
};

/// Domain identifier for file watcher logs
const WATCHER_LOGGER_DOMAIN: &str = "[WATCHER]";

/// A robust filesystem watcher with debounce support and graceful shutdown
///
/// This watcher provides:
/// - Configurable debounce period for event processing
/// - Graceful handling of Ctrl+C signals
/// - State management (Running/Paused/Stopped)
/// - Automatic directory creation
/// - Thread-safe operation
pub struct FileWatcher {

    /// The path being watched (expanded with tilde if needed)
    path: PathBuf,

    /// Underlying notify watcher instance
    watcher: Option<RecommendedWatcher>,

    /// Current operational state
    state: WatcherState,

    /// Callback for processing filesystem events
    callback: Option<FileWatcherCallback>,

    /// Debounce period for event processing
    debounce_time: Duration,

    /// Channel sender for raw filesystem events
    event_tx: Sender<Event>,

    /// Channel receiver for event processing
    event_rx: Option<Receiver<Event>>,

    /// Handle to the async event processing task
    worker_handle: Option<tokio::task::JoinHandle<()>>,

    /// Atomic flag for graceful shutdown
    should_exit: Arc<AtomicBool>,
}

impl FileWatcher {

    /// Creates a new FileWatcher instance
    ///
    /// # Arguments
    /// * `path` - Path to watch (supports tilde expansion)
    /// * `debounce_time` - Minimum delay between processing events 
    /// (will be clamped to at least 2 seconds if lower value provided)
    ///
    /// # Notes
    /// - Watcher starts in Stopped state (call `resume()` to begin watching)
    /// - Path will be created if it doesn't exist when watching starts
    pub fn new<P: AsRef<Path>>(
        path: P,
        debounce_time: Duration
    ) -> Self {
        let path = PathHelper::expand_tilde(path.as_ref());
        let debounce_time = if debounce_time < Duration::from_secs(2) {
            warn_log!(
                WATCHER_LOGGER_DOMAIN, 
                "Debounce time can't be less than 2s. Adjusted to 2s."
            );
            Duration::from_secs(2)
        } else {
            debounce_time
        };
        let (event_tx, event_rx) = channel(100);

        Self {
            path,
            watcher: None,
            state: WatcherState::Stopped,
            callback: None,
            debounce_time,
            event_tx,
            event_rx: Some(event_rx),
            worker_handle: None,
            should_exit: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Sets up Ctrl+C handler for graceful shutdown
    ///
    /// # Returns
    /// - `Ok(())` if handler was registered successfully
    /// - `Err(ctrlc::Error)` if handler registration failed
    ///
    /// # Notes
    /// - Sets the `should_exit` flag when triggered
    /// - Should be called before starting the watcher
    pub fn setup_ctrlc_handler(&self) -> Result<(), ctrlc::Error> {
        let should_exit = self.should_exit.clone();
        ctrlc::set_handler(move || {
            should_exit.store(true, Ordering::Relaxed);
            info_log!(WATCHER_LOGGER_DOMAIN,"Received Ctrl+C, shutting down gracefully...");
        })
    }

    /// Checks if shutdown was requested
    ///
    /// # Returns
    /// `true` if graceful shutdown was requested (via Ctrl+C)
    pub fn get_should_exit(&self) -> bool {
        self.should_exit.load(Ordering::Relaxed)
    }

    /// Initializes the filesystem watcher
    ///
    /// # Returns
    /// - `Ok(())` if watcher was initialized successfully
    /// - `Err(String)` with error message if initialization failed
    ///
    /// # Notes
    /// - Creates directory if it doesn't exist
    /// - Starts event processing task
    /// - Only effective when in Stopped state
    fn init_watcher(&mut self) -> Result<(), String> {
        if self.state != WatcherState::Stopped {
            return Ok(());
        }

        if !self.path.exists() {
            std::fs::create_dir_all(&self.path).map_err(|e| {
                format!(
                    "Failed to create directory {}: {}",
                    self.path.display(),
                    e
                )
            })?;
            let msg = format!("Created directory: {}", self.path.display());
            info_log!(WATCHER_LOGGER_DOMAIN, msg);
        }

        let event_tx = self.event_tx.clone();
        let mut watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    if let Err(e) = event_tx.blocking_send(event) {
                        let msg = format!("Failed to send event: {}", e);
                        error_log!(WATCHER_LOGGER_DOMAIN, msg);
                    }
                }
                Err(e) => {
                    let msg = format!("Watch error: {}", e);
                    error_log!(WATCHER_LOGGER_DOMAIN, msg);
                }
            }
        })
            .map_err(|e| format!("Failed to create watcher: {}", e))?;

        watcher
            .watch(&self.path, RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch path {}: {}", self.path.display(), e))?;

        self.watcher = Some(watcher);
        self.state = WatcherState::Running;

        info_log!(
            WATCHER_LOGGER_DOMAIN,
            format!("Started watching directory: {}", self.path.display())
        );

        self.start_event_processor();

        Ok(())
    }

    /// Starts the async event processing task
    ///
    /// # Notes
    /// - Implements debounce logic
    /// - Only processes the last event in each debounce window
    /// - Checks for shutdown signal periodically
    fn start_event_processor(&mut self) {
        if self.worker_handle.is_some() {
            return;
        }

        let debounce_time = self.debounce_time;
        let callback = self.callback.clone();
        let event_rx = self.event_rx.take()
            .expect("Event receiver already taken");
        let should_exit = self.should_exit.clone();

        let handle = tokio::spawn(async move {
            let mut last_event = None;
            let mut stream = ReceiverStream::new(event_rx);

            loop {
                tokio::select! {
                    Some(event) = stream.next() => {
                        last_event = Some(event);
                    }

                    _ = sleep(debounce_time) => {
                        if let Some(event) = &last_event {
                            if let Some(cb) = &callback {
                                cb.0(event.kind);
                            }
                            last_event = None;
                        }
                    }

                    _ = sleep(Duration::from_secs(1)), if should_exit.load(Ordering::Relaxed) => {
                        break;
                    }
                }
            }
        });

        self.worker_handle = Some(handle);
    }
}

impl FileWatchable for FileWatcher {

    /// Gets the current watcher state
    fn get_state(&self) -> WatcherState {
        self.state.clone()
    }
    
    /// Resumes or starts watching
    ///
    /// # Returns
    /// - `Ok(())` if operation succeeded
    /// - `Err(String)` with error message if failed
    ///
    /// # Notes
    /// - If Stopped, initializes a new watcher
    /// - If Paused, resumes watching
    /// - If Running, no effect
    fn resume(&mut self) -> Result<(), String> {
        if self.state == WatcherState::Paused {
            self.state = WatcherState::Running;
            info_log!(WATCHER_LOGGER_DOMAIN, "Resumed watching.");
            Ok(())
        } else if self.state == WatcherState::Stopped {
            self.init_watcher()
        } else {
            Ok(())
        }
    }

    /// Pauses watching
    ///
    /// # Notes
    /// - Only effective when in Running state
    /// - Maintains watch configuration while paused
    fn pause(&mut self) {
        if self.state == WatcherState::Running {
            self.state = WatcherState::Paused;
            info_log!(WATCHER_LOGGER_DOMAIN, "Paused watching.");
        }
    }

    /// Stops watching and releases resources
    ///
    /// # Notes
    /// - Aborts the event processing task
    /// - Drops the underlying watcher
    /// - Cannot be resumed after stopping
    fn stop(&mut self) {
        if self.state != WatcherState::Stopped {
            self.state = WatcherState::Stopped;
            info_log!(WATCHER_LOGGER_DOMAIN, "Stopped watching.");
            self.watcher.take();
            if let Some(handle) = self.worker_handle.take() {
                tokio::spawn(async move {
                    handle.abort();
                    let _ = handle.await;
                });
            }
        }
    }

    /// Sets the event callback
    ///
    /// # Arguments
    /// * `callback` - Function to call when events occur
    ///
    /// # Notes
    /// - Replaces any existing callback
    /// - Callback must be thread-safe
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(EventKind) + Send + Sync + 'static,
    {
        self.callback = Some(FileWatcherCallback::new(callback));
    }
}

impl Drop for FileWatcher {

    /// Ensures clean shutdown when watcher is dropped
    fn drop(&mut self) {
        self.stop();
    }
}