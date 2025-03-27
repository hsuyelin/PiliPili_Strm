use std::{
    sync::Arc,
    ops::Deref
};

use notify::EventKind;

#[derive(Clone)]
pub struct FileWatcherCallback(pub(crate) Arc<dyn Fn(EventKind) + Send + Sync>);

impl FileWatcherCallback {
    pub fn new<F: Fn(EventKind) + Send + Sync + 'static>(f: F) -> Self {
        Self(Arc::new(f))
    }
}

impl Deref for FileWatcherCallback {
    type Target = Arc<dyn Fn(EventKind) + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}