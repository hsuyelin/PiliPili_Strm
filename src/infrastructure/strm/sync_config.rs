use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub video_extensions: Vec<String>,
    pub audio_extensions: Vec<String>,
    pub ignore_extensions: Vec<String>,
    pub ignore_keywords: Vec<String>,
    pub ignore_regex: Vec<String>,
    pub name_replacements: Vec<(String, String)>,
    pub soft_delete_dir: Option<PathBuf>,
    pub rclone_remote: Option<String>,
    pub rsync_args: Option<Vec<String>>,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            video_extensions: vec![
                "mp4".into(),
                "mkv".into(),
                "avi".into(),
                "mov".into(),
                "wmv".into(),
                "flv".into(),
                "m2ts".into(),
                "mts".into(),
                "ts".into(),
                "rmvb".into(),
                "rm".into(),
                "vob".into(),
            ],
            audio_extensions: vec![
                "mp3".into(),
                "wav".into(),
                "flac".into(),
                "aac".into(),
                "ogg".into(),
                "ape".into(),
                "opus".into(),
            ],
            ignore_extensions: vec![],
            ignore_keywords: vec![],
            ignore_regex: vec![],
            name_replacements: vec![],
            soft_delete_dir: None,
            rclone_remote: None,
            rsync_args: None,
        }
    }
}