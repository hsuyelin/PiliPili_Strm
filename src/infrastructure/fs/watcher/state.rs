use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

/// Represents the operational state of a file system watcher
///
/// This enum defines the possible states a file watcher can be in,
/// allowing for explicit state management and monitoring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatcherState {

    /// The watcher is actively monitoring for filesystem changes
    ///
    /// In this state:
    /// - Events are being processed
    /// - Callbacks are being triggered
    /// - Resources are actively being used
    Running,

    /// The watcher is temporarily inactive but can be resumed
    ///
    /// In this state:
    /// - No events are being processed
    /// - Callbacks are not triggered
    /// - Watch configurations are maintained
    Paused,

    /// The watcher is permanently stopped and cannot be restarted
    ///
    /// In this state:
    /// - All resources have been released
    /// - No further events will be processed
    /// - A new watcher must be created to resume monitoring
    Stopped,
}

impl Display for WatcherState {

    /// Formats the watcher state for display purposes
    ///
    /// # Arguments
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    /// `fmt::Result` indicating success or failure of the operation
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let state_str = match *self {
            WatcherState::Running => "Running",
            WatcherState::Paused => "Paused",
            WatcherState::Stopped => "Stopped",
        };
        write!(f, "{}", state_str)
    }
}