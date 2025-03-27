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

use crate::{error_log, info_log};
use super::{
    watcher_state::WatcherState,
    watcher_callback::FileWatcherCallback,
    watchable::FileWatchable,
    path_helper::PathHelper,
};

const WATCHER_LOGGER_DOMAIN: &str = "[WATCHER]";

pub struct FileWatcher {
    path: PathBuf,
    watcher: Option<RecommendedWatcher>,
    state: WatcherState,
    callback: Option<FileWatcherCallback>,
    debounce_time: Duration,
    event_tx: Sender<Event>,
    event_rx: Option<Receiver<Event>>,
    worker_handle: Option<tokio::task::JoinHandle<()>>,
    should_exit: Arc<AtomicBool>,
}

impl FileWatcher {
    pub fn new<P: AsRef<Path>>(
        path: P,
        debounce_time: Duration
    ) -> Self {
        let path = PathHelper::expand_tilde(path.as_ref());
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

    pub fn setup_ctrlc_handler(&self) -> Result<(), ctrlc::Error> {
        let should_exit = self.should_exit.clone();
        ctrlc::set_handler(move || {
            should_exit.store(true, Ordering::Relaxed);
            info_log!(WATCHER_LOGGER_DOMAIN,"Received Ctrl+C, shutting down gracefully...");
        })
    }

    pub fn get_should_exit(&self) -> bool {
        self.should_exit.load(Ordering::Relaxed)
    }

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

    fn start_event_processor(&mut self) {
        if self.worker_handle.is_some() {
            return;
        }

        let debounce_time = self.debounce_time;
        let callback = self.callback.clone();
        let event_rx = self.event_rx.take().expect("Event receiver already taken");
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
    
    fn get_state(&self) -> WatcherState {
        self.state.clone()
    }
    
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

    fn pause(&mut self) {
        if self.state == WatcherState::Running {
            self.state = WatcherState::Paused;
            info_log!(WATCHER_LOGGER_DOMAIN, "Paused watching.");
        }
    }

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

    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(EventKind) + Send + Sync + 'static,
    {
        self.callback = Some(FileWatcherCallback::new(callback));
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}