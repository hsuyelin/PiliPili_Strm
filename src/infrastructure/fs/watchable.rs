use notify::EventKind;
use crate::infrastructure::fs::WatcherState;

pub trait FileWatchable {
    fn get_state(&self) -> WatcherState;
    fn resume(&mut self) -> Result<(), String>;
    fn pause(&mut self);
    fn stop(&mut self);
    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(EventKind) + Send + Sync + 'static;
}