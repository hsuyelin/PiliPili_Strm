use std::{
    sync::Arc,
    ops::Deref
};
use notify::EventKind;

/// A thread-safe, cloneable callback wrapper for filesystem events
///
/// This type encapsulates a callback function that handles filesystem notifications,
/// providing both thread safety through `Send + Sync` bounds and cheap cloning
/// through `Arc` reference counting.
#[derive(Clone)]
pub struct FileWatcherCallback(pub(crate) Arc<dyn Fn(EventKind) + Send + Sync>);

impl FileWatcherCallback {

    /// Creates a new `FileWatcherCallback` from a closure or function
    ///
    /// # Arguments
    /// * `f` - The callback function that will handle filesystem events
    ///
    /// # Generic Parameters
    /// * `F` - The callback type, must satisfy:
    ///   - `Fn(EventKind)` to handle events
    ///   - `Send + Sync` for thread safety
    ///   - `'static` lifetime
    ///
    /// # Notes
    /// - The callback will be wrapped in an `Arc` for shared ownership
    /// - The resulting callback can be cloned cheaply
    pub fn new<F: Fn(EventKind) + Send + Sync + 'static>(f: F) -> Self {
        Self(Arc::new(f))
    }
}

impl Deref for FileWatcherCallback {

    type Target = Arc<dyn Fn(EventKind) + Send + Sync>;

    /// Provides dereferencing access to the underlying callback
    ///
    /// This allows treating `FileWatcherCallback` instances as if they were
    /// direct references to the contained `Arc`-wrapped callback.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}