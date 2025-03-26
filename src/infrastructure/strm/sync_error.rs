use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Sync operation error: {0}")]
    SyncOperationError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("File watcher error: {0}")]
    WatcherError(String),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Notification error: {0}")]
    NotifyError(#[from] notify::Error),

    #[error("Rclone error: {0}")]
    RcloneError(String),

    #[error("Rsync error: {0}")]
    RsyncError(String),

    #[error("File already exists: {0}")]
    FileExists(PathBuf),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

pub type SyncResult<T> = Result<T, SyncError>;