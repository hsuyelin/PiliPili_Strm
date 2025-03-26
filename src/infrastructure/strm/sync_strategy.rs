use std::path::Path;
use tokio::process::Command;
use async_trait::async_trait;

use super::{
    sync_error::{SyncError, SyncResult},
    sync_config::SyncConfig
};

#[async_trait]
pub trait SyncStrategy: Send + Sync {
    async fn copy(&self, src: &Path, dest: &Path) -> SyncResult<()>;
    async fn sync(&self, src: &Path, dest: &Path) -> SyncResult<()>;
    async fn delete(&self, path: &Path) -> SyncResult<()>;
    async fn ensure_directory(&self, path: &Path) -> SyncResult<()>;
}

pub struct LocalSyncStrategy {
    config: SyncConfig,
}

impl LocalSyncStrategy {
    pub fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    fn build_rsync_command(&self, src: &Path, dest: &Path, delete: bool) -> Command {
        let mut cmd = Command::new("rsync");

        cmd.arg("-avz")
            .arg("--progress")
            .arg(format!("{}/", src.display()));

        for ext in &self.config.video_extensions {
            cmd.arg("--exclude").arg(format!("*.{}", ext));
        }

        for ext in &self.config.audio_extensions {
            cmd.arg("--exclude").arg(format!("*.{}", ext));
        }

        if delete {
            cmd.arg("--delete");
        }

        cmd.arg(dest);

        if let Some(args) = &self.config.rsync_args {
            cmd.args(args);
        }

        cmd
    }
}

#[async_trait]
impl SyncStrategy for LocalSyncStrategy {
    async fn copy(&self, src: &Path, dest: &Path) -> SyncResult<()> {
        let output = self.build_rsync_command(src, dest, false)
            .output()
            .await?;

        if !output.status.success() {
            return Err(SyncError::RsyncError(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ));
        }
        Ok(())
    }

    async fn sync(&self, src: &Path, dest: &Path) -> SyncResult<()> {
        let output = self.build_rsync_command(src, dest, true)
            .output()
            .await?;

        if !output.status.success() {
            return Err(SyncError::RsyncError(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ));
        }
        Ok(())
    }

    async fn delete(&self, path: &Path) -> SyncResult<()> {
        if let Some(soft_delete_dir) = &self.config.soft_delete_dir {
            let dest = soft_delete_dir.join(
                path.file_name()
                    .ok_or_else(|| SyncError::PathError("Invalid file name".into()))?
            );
            tokio::fs::rename(path, dest).await?;
        } else {
            tokio::fs::remove_file(path).await?;
        }
        Ok(())
    }

    async fn ensure_directory(&self, path: &Path) -> SyncResult<()> {
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
            tracing::info!("Created local directory: {}", path.display());
        }
        Ok(())
    }
}

pub struct RcloneSyncStrategy {
    config: SyncConfig,
}

impl RcloneSyncStrategy {
    pub fn new(config: SyncConfig) -> Self {
        Self { config }
    }

    fn build_rclone_command(&self, operation: &str, src: &Path, dest: &str) -> Command {
        let mut cmd = Command::new("rclone");

        cmd.arg(operation)
            .arg("--progress")
            .arg(format!("{}/", src.display()));

        for ext in &self.config.video_extensions {
            cmd.arg("--exclude").arg(format!("*.{}", ext));
        }

        for ext in &self.config.audio_extensions {
            cmd.arg("--exclude").arg(format!("*.{}", ext));
        }

        cmd.arg(dest);

        cmd
    }
}

#[async_trait]
impl SyncStrategy for RcloneSyncStrategy {
    async fn copy(&self, src: &Path, dest: &Path) -> SyncResult<()> {
        let remote = self.config.rclone_remote.as_ref()
            .ok_or_else(|| SyncError::ConfigError("Rclone remote not configured".into()))?;

        let dest_str = format!("{}:{}", remote, dest.to_str().unwrap());

        let output = self.build_rclone_command("copy", src, &dest_str)
            .output()
            .await?;

        if !output.status.success() {
            return Err(SyncError::RcloneError(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ));
        }
        Ok(())
    }

    async fn sync(&self, src: &Path, dest: &Path) -> SyncResult<()> {
        let remote = self.config.rclone_remote.as_ref()
            .ok_or_else(|| SyncError::ConfigError("Rclone remote not configured".into()))?;

        let dest_str = format!("{}:{}", remote, dest.to_str().unwrap());

        let output = self.build_rclone_command("sync", src, &dest_str)
            .output()
            .await?;

        if !output.status.success() {
            return Err(SyncError::RcloneError(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ));
        }
        Ok(())
    }

    async fn delete(&self, path: &Path) -> SyncResult<()> {
        let remote = self.config.rclone_remote.as_ref()
            .ok_or_else(|| SyncError::ConfigError("Rclone remote not configured".into()))?;

        let path_str = format!("{}:{}", remote, path.to_str().unwrap());

        let output = Command::new("rclone")
            .arg("delete")
            .arg(&path_str)
            .output()
            .await?;

        if !output.status.success() {
            return Err(SyncError::RcloneError(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ));
        }
        Ok(())
    }

    async fn ensure_directory(&self, path: &Path) -> SyncResult<()> {
        let remote = self.config.rclone_remote.as_ref()
            .ok_or_else(|| SyncError::ConfigError("Rclone remote not configured".into()))?;

        let path_str = format!("{}:{}", remote, path.to_str()
            .ok_or_else(|| SyncError::PathError("Invalid path".into()))?);

        let output = Command::new("rclone")
            .arg("mkdir")
            .arg("--parents")
            .arg(&path_str)
            .output()
            .await?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            tracing::error!("Failed to create remote directory: {}", error_msg);
            return Err(SyncError::RcloneError(error_msg.into_owned()));
        }

        tracing::info!("Created remote directory: {}", path_str);
        Ok(())
    }
}