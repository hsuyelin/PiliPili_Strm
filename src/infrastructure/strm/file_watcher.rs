use std::path::{Path, PathBuf};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event, EventKind};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use std::future::Future;

use super::{
    sync_error::{SyncError, SyncResult}, 
    sync_config::SyncConfig, 
    media_detector::MediaDetector
};

pub struct FileWatcher {
    config: SyncConfig,
    media_detector: Option<MediaDetector>,
}

impl FileWatcher {
    pub fn new(config: SyncConfig) -> Self {
        let media_detector = MediaDetector::new(config.clone()).ok();
        Self {
            config,
            media_detector,
        }
    }

    pub fn get_config(&self) -> &SyncConfig {
        &self.config
    }

    pub async fn watch<F, Fut>(&self, path: &Path, callback: F) -> SyncResult<()>
    where
        F: FnMut(PathBuf, EventKind) -> Fut + Send + 'static,
        Fut: Future<Output = SyncResult<()>> + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel(32);

        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            notify::Config::default()
        )?;

        watcher.watch(path, RecursiveMode::Recursive)?;

        let mut callback = callback;
        while let Some(event) = rx.recv().await {
            self.handle_event(event, &mut callback).await?;
        }

        Ok(())
    }

    async fn handle_event<F, Fut>(&self, event: Event, callback: &mut F) -> SyncResult<()>
    where
        F: FnMut(PathBuf, EventKind) -> Fut,
        Fut: Future<Output = SyncResult<()>>,
    {
        sleep(Duration::from_millis(100)).await;

        let Some(media_detector) = self.media_detector.as_ref() else {
            return Err(SyncError::ConfigError("Media detector is not initialized".into()));
        };
        
        for path in event.paths {
            if media_detector.should_ignore(&path) {
                continue;
            }

            callback(path, event.kind).await?;
        }

        Ok(())
    }
}