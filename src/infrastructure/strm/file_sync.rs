use std::path::Path;
use std::sync::Arc;
use notify::EventKind;

use super::{
    sync_error::{SyncError, SyncResult},
    sync_config::SyncConfig,
    media_detector::MediaDetector,
    strm_generator::StrmGenerator,
    sync_strategy::{SyncStrategy, LocalSyncStrategy, RcloneSyncStrategy},
    file_watcher::FileWatcher,
};

pub struct FileSync {
    config: SyncConfig,
    detector: MediaDetector,
    generator: StrmGenerator,
    strategy: Arc<dyn SyncStrategy>,
    watcher: Option<FileWatcher>,
}

impl FileSync {
    pub fn new(config: SyncConfig) -> SyncResult<Self> {
        let detector = MediaDetector::new(config.clone())?;
        let generator = StrmGenerator::new(config.clone());

        let strategy: Arc<dyn SyncStrategy> = if config.rclone_remote.is_some() {
            Arc::new(RcloneSyncStrategy::new(config.clone()))
        } else {
            Arc::new(LocalSyncStrategy::new(config.clone()))
        };

        let watcher = if config.soft_delete_dir.is_some() {
            Some(FileWatcher::new(config.clone()))
        } else {
            None
        };

        Ok(Self {
            config,
            detector,
            generator,
            strategy,
            watcher,
        })
    }

    pub fn get_config(&self) -> &SyncConfig {
        &self.config
    }

    pub fn get_generator(&self) -> &StrmGenerator {
        &self.generator
    }

    pub async fn sync_directory(&self, src: &Path, dest: &Path, operation: &str) -> SyncResult<()> {
        self.ensure_directory(dest).await?;

        self.generator.generate_strm_for_dir(src).await?;
        
        match operation {
            "copy" => self.strategy.copy(src, dest).await?,
            "sync" => self.strategy.sync(src, dest).await?,
            _ => return Err(SyncError::UnsupportedOperation(operation.to_string())),
        }

        Ok(())
    }

    pub async fn watch_directory(&self, src: &Path, dest: &Path) -> SyncResult<()> {
        let watcher = self.watcher.as_ref()
            .ok_or_else(|| SyncError::ConfigError("File watcher not configured".into()))?;

        let detector = Arc::new(self.detector.clone());
        let strategy = Arc::new(self.strategy.clone());
        let generator = Arc::new(self.generator.clone());
        let src_path = Arc::new(src.to_path_buf());
        let dest_path = Arc::new(dest.to_path_buf());

        watcher.watch(src, {
            let detector = detector.clone();
            let strategy = strategy.clone();
            let generator = generator.clone();
            let src_path = src_path.clone();
            let dest_path = dest_path.clone();

            move |path, kind| {
                let detector = detector.clone();
                let strategy = strategy.clone();
                let generator = generator.clone();
                let src_path = src_path.clone();
                let dest_path = dest_path.clone();

                Box::pin(async move {
                    match kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            if detector.is_media_file(&path) {
                                let rel_path = path.strip_prefix(&*src_path)
                                    .map_err(|e| SyncError::PathError(e.to_string()))?;
                                let full_dest = dest_path.join(rel_path);
                                generator.generate_strm(&full_dest).await?;
                            }
                        }
                        EventKind::Remove(_) => {
                            let rel_path = path.strip_prefix(&*src_path)
                                .map_err(|e| SyncError::PathError(e.to_string()))?;
                            let full_dest = dest_path.join(rel_path);
                            strategy.delete(&full_dest).await?;
                        }
                        _ => {}
                    }
                    Ok(())
                })
            }
        }).await?;

        Ok(())
    }

    async fn ensure_directory(&self, path: &Path) -> SyncResult<()> {
        self.strategy.ensure_directory(path).await
    }
}