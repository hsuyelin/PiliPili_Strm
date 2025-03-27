use notify::EventKind;
use crate::infrastructure::fs::WatcherState;

/// A trait defining the interface for file system watchers
/// 
/// This provides common operations for controlling file system monitoring
/// with the ability to pause, resume, and stop watching, as well as
/// handling notification callbacks.
pub trait FileWatchable {

    /// Gets the current state of the watcher
    ///
    /// # Returns
    /// The current [`WatcherState`] indicating whether the watcher is
    /// active, paused, or stopped
    fn get_state(&self) -> WatcherState;

    /// Resumes watching after being paused
    ///
    /// # Returns
    /// - `Ok(())` if successfully resumed
    /// - `Err(String)` with error message if resuming failed
    ///
    /// # Notes
    /// - Only valid when watcher is in paused state
    /// - May fail if underlying watcher cannot be restarted
    fn resume(&mut self) -> Result<(), String>;

    /// Temporarily pauses watching without shutting down
    ///
    /// # Notes
    /// - Watcher can be later resumed with [`resume()`]
    /// - Maintains existing watch configuration while paused
    fn pause(&mut self);

    /// Permanently stops watching and releases resources
    ///
    /// # Notes
    /// - Cannot be restarted after stopping
    /// - Different from pausing as it cleans up resources
    fn stop(&mut self);

    /// Sets the callback function for handling filesystem events
    ///
    /// # Arguments
    /// * `callback` - Closure that will be called when filesystem events occur
    ///
    /// # Generic Parameters
    /// * `F` - Callback type implementing `Fn(EventKind)` and thread safety traits
    ///
    /// # Notes
    /// - Callback must be thread-safe (`Send + Sync`)
    /// - Callback will receive [`EventKind`] notifications
    /// - Only one callback can be active at a time (replaces previous)
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(EventKind) + Send + Sync + 'static;
}