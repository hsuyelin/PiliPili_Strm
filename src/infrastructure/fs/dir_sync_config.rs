use regex::Regex;
use anyhow::Result;

use crate::infrastructure::fs::ssh_config::SshConfig;

#[derive(Debug)]
pub struct DirSyncConfig {

    source_dir: String, 
    destination_dir: String,
    strict_mode: bool,
    suffixes: Vec<String>,
    ignore_regex: Option<Regex>,
    guard_file: Option<String>,
    ssh_config: Option<SshConfig>,
}

impl DirSyncConfig {

    pub fn new(source: &str, destination: &str) -> Self {
        DirSyncConfig {
            source_dir: source.to_string(),
            destination_dir: destination.to_string(),
            strict_mode: false,
            suffixes: Vec::new(),
            ignore_regex: None,
            guard_file: None,
            ssh_config: None,
        }
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    pub fn with_suffixes(mut self, suffixes: Vec<&str>) -> Self {
        self.suffixes = suffixes.into_iter().map(String::from).collect();
        self
    }

    pub fn with_ignore_regex(mut self, regex: &str) -> Result<Self> {
        self.ignore_regex = Some(Regex::new(regex)?);
        Ok(self)
    }

    pub fn with_guard_file(mut self, guard_file: &str) -> Self {
        self.guard_file = Some(guard_file.to_string());
        self
    }

    pub fn with_ssh_config(mut self, ssh_config: SshConfig) -> Self {
        self.ssh_config = Some(ssh_config);
        self
    }

    pub fn get_source_dir(&self) -> String {
        self.source_dir.to_string().clone()
    }
    
    pub fn get_destination_dir(&self) -> String {
        self.destination_dir.to_string().clone()
    }
    
    pub fn get_guard_file(&self) -> Option<String> {
        self.guard_file.clone()
    }
    
    pub fn set_guard_file(&mut self, guard_file: &str) {
        self.guard_file = Some(guard_file.to_string());
    }
    
    pub fn get_ssh_config(&self) -> Option<SshConfig> {
        self.ssh_config.clone()
    }
    
    pub fn get_strict_mode(&self) -> bool {
        self.strict_mode.clone()
    }
    
    pub fn get_suffixes(&self) -> Vec<String> {
        self.suffixes.clone()
    }
    
    pub fn get_ignore_regex(&self) -> Option<Regex> {
        self.ignore_regex.clone()
    }
}