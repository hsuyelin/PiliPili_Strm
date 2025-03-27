use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatcherState {
    Running,
    Paused,
    Stopped,
}

impl fmt::Display for WatcherState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state_str = match *self {
            WatcherState::Running => "Running",
            WatcherState::Paused => "Paused",
            WatcherState::Stopped => "Stopped",
        };
        write!(f, "{}", state_str)
    }
}