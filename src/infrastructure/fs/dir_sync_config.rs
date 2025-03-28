use std::fmt::{
    Display, 
    Formatter, 
    Result as FmtResult,
    Error
};

use serde::Serialize;
use serde_regex;
use regex::Regex;
use anyhow::Result;

use super::DirLocation;

#[derive(Clone, Debug, Serialize)]
pub struct DirSyncConfig {

    source: DirLocation,
    destination: DirLocation,
    strict_mode: bool,
    include_suffixes: Vec<String>,
    exclude_suffixes: Vec<String>,
    #[serde(with = "serde_regex")]
    exclude_regex: Option<Regex>,
    guard_file: Option<String>,
}

impl Display for DirSyncConfig {
    
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let json_str = serde_json::to_string(self).map_err(|_| Error)?;
        write!(f, "{}", json_str)
    }
}

impl Default for DirSyncConfig {

    fn default() -> Self {
        DirSyncConfig {
            source: DirLocation::default(),
            destination: DirLocation::default(),
            strict_mode: false,
            include_suffixes: Vec::new(),
            exclude_suffixes: Vec::new(),
            exclude_regex: None,
            guard_file: None,
        }
    }
}

impl DirSyncConfig {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> Self {
        Self::new()
    }

    pub fn with_source(mut self, source: DirLocation) -> Self {
        self.source = source;
        self
    }

    pub fn with_destination(mut self, destination: DirLocation) -> Self {
        self.destination = destination;
        self
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    pub fn with_include_suffixes(mut self, suffixes: Vec<&str>) -> Self {
        self.include_suffixes = suffixes.into_iter()
            .map(|s| String::from(s.trim_start_matches('.')))
            .collect();
        self
    }

    pub fn with_exclude_suffixes(mut self, suffixes: Vec<&str>) -> Self {
        self.exclude_suffixes = suffixes.into_iter()
            .map(|s| String::from(s.trim_start_matches('.')))
            .collect();
        self
    }

    pub fn with_exclude_regex(mut self, regex: &str) -> Result<Self> {
        self.exclude_regex = Some(Regex::new(regex)?);
        Ok(self)
    }

    pub fn with_guard_file(mut self, guard_file: &str) -> Self {
        self.guard_file = Some(guard_file.to_string());
        self
    }

    pub fn get_source(&self) -> DirLocation {
        self.source.clone()
    }

    pub fn get_destination(&self) -> DirLocation {
        self.destination.clone()
    }

    pub fn get_guard_file(&self) -> Option<String> {
        self.guard_file.clone()
    }

    pub fn get_strict_mode(&self) -> bool {
        self.strict_mode
    }
    
    pub fn get_include_suffixes(&self) -> Vec<String> {
        self.include_suffixes.clone()
    }

    pub fn get_exclude_suffixes(&self) -> Vec<String> {
        self.exclude_suffixes.clone()
    }

    pub fn get_exclude_regex(&self) -> Option<Regex> {
        self.exclude_regex.clone()
    }
}