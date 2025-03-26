use std::path::{Path, PathBuf};
use tokio::fs;
use std::sync::Arc;

use super::{
    sync_error::{SyncError, SyncResult},
    sync_config::SyncConfig
};

#[derive(Debug, Clone)]
pub struct StrmGenerator {
    config: Arc<SyncConfig>,
}

impl StrmGenerator {
    pub fn new(config: SyncConfig) -> Self {
        Self {
            config: Arc::new(config)
        }
    }

    pub async fn generate_strm(&self, media_path: &Path) -> SyncResult<PathBuf> {
        let strm_path = media_path.with_extension("strm");

        if strm_path.exists() {
            return Ok(strm_path);
        }

        let content = media_path.to_str()
            .ok_or_else(|| SyncError::PathError(format!("Invalid path: {:?}", media_path)))?;

        fs::write(&strm_path, content).await?;
        Ok(strm_path)
    }

    pub async fn generate_strm_for_dir(&self, dir_path: &Path) -> SyncResult<Vec<PathBuf>> {
        let mut result = Vec::new();
        let mut dir_stack = vec![dir_path.to_path_buf()];

        while let Some(current_dir) = dir_stack.pop() {
            let mut entries = fs::read_dir(&current_dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();

                if path.is_dir() {
                    dir_stack.push(path);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if self.config.video_extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) ||
                        self.config.audio_extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                        result.push(self.generate_strm(&path).await?);
                    }
                }
            }
        }

        Ok(result)
    }
}