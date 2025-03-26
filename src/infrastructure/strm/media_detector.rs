use std::path::Path;
use regex::Regex;
use super::{
    sync_error::SyncError, 
    sync_config::SyncConfig
};

#[derive(Debug, Clone)]
pub struct MediaDetector {
    config: SyncConfig,
    ignore_regex: Vec<Regex>,
}

impl MediaDetector {
    pub fn new(config: SyncConfig) -> Result<Self, SyncError> {
        let ignore_regex = config.ignore_regex
            .iter()
            .map(|pattern| Regex::new(pattern))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { config, ignore_regex })
    }

    pub fn is_media_file(&self, path: &Path) -> bool {
        self.is_video_file(path) || self.is_audio_file(path)
    }

    pub fn is_video_file(&self, path: &Path) -> bool {
        self.check_extension(path, &self.config.video_extensions)
    }

    pub fn is_audio_file(&self, path: &Path) -> bool {
        self.check_extension(path, &self.config.audio_extensions)
    }

    pub fn should_ignore(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if self.config.ignore_extensions.iter()
                    .any(|e| e.eq_ignore_ascii_case(ext)) {
                    return true;
                }
            }

            if self.config.ignore_keywords.iter()
                .any(|k| file_name.contains(k)) {
                return true;
            }

            if self.ignore_regex.iter()
                .any(|r| r.is_match(file_name)) {
                return true;
            }
        }

        false
    }

    fn check_extension(&self, path: &Path, extensions: &[String]) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
        } else {
            false
        }
    }
}