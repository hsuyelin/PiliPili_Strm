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

/// Configuration for directory synchronization operations.
///
/// This struct encapsulates all parameters needed to perform directory
/// synchronization between source and destination locations, with various
/// filtering options and safety checks.
#[derive(Clone, Debug, Serialize)]
pub struct DirSyncConfig {

    /// Source directory location (local or remote)
    source: DirLocation,

    /// Destination directory location (local or remote)
    destination: DirLocation,

    /// When true, enables additional validation and safety checks
    strict_mode: bool,

    /// List of file suffixes to explicitly include (without leading dots)
    include_suffixes: Vec<String>,

    /// List of file suffixes to explicitly exclude (without leading dots)
    exclude_suffixes: Vec<String>,

    /// Optional regex pattern for excluding matching paths
    #[serde(with = "serde_regex")]
    exclude_regex: Option<Regex>,

    /// Optional guard file that must be present to proceed with sync
    guard_file: Option<String>,
}

impl Display for DirSyncConfig {

    /// Formats the configuration as a JSON string.
    ///
    /// # Errors
    /// Returns `std::fmt::Error` if JSON serialization fails.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let json_str = serde_json::to_string(self).map_err(|_| Error)?;
        write!(f, "{}", json_str)
    }
}

impl Default for DirSyncConfig {

    /// Creates a default `DirSyncConfig` with empty values and disabled strict mode.
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

    /// Creates a new `DirSyncConfig` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts a builder pattern chain for creating a configuration.
    pub fn builder() -> Self {
        Self::new()
    }

    /// Sets the source directory location (builder pattern).
    pub fn with_source(mut self, source: DirLocation) -> Self {
        self.source = source;
        self
    }

    /// Sets the destination directory location (builder pattern).
    pub fn with_destination(mut self, destination: DirLocation) -> Self {
        self.destination = destination;
        self
    }

    /// Enables or disables strict mode (builder pattern).
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Sets included file suffixes, automatically trimming leading dots (builder pattern).
    pub fn with_include_suffixes(mut self, suffixes: Vec<&str>) -> Self {
        self.include_suffixes = suffixes.into_iter()
            .map(|s| String::from(s.trim_start_matches('.')))
            .collect();
        self
    }

    /// Sets excluded file suffixes, automatically trimming leading dots (builder pattern).
    pub fn with_exclude_suffixes(mut self, suffixes: Vec<&str>) -> Self {
        self.exclude_suffixes = suffixes.into_iter()
            .map(|s| String::from(s.trim_start_matches('.')))
            .collect();
        self
    }

    /// Sets an exclusion regex pattern (builder pattern).
    ///
    /// # Errors
    /// Returns `anyhow::Error` if the regex pattern is invalid.
    pub fn with_exclude_regex(mut self, regex: &str) -> Result<Self> {
        self.exclude_regex = Some(Regex::new(regex)?);
        Ok(self)
    }

    /// Sets a guard file requirement (builder pattern).
    pub fn with_guard_file(mut self, guard_file: &str) -> Self {
        self.guard_file = Some(guard_file.to_string());
        self
    }

    /// Gets a clone of the source directory location.
    pub fn get_source(&self) -> DirLocation {
        self.source.clone()
    }

    /// Gets a clone of the destination directory location.
    pub fn get_destination(&self) -> DirLocation {
        self.destination.clone()
    }

    /// Gets a clone of the guard file path, if set.
    pub fn get_guard_file(&self) -> Option<String> {
        self.guard_file.clone()
    }

    /// Returns whether strict mode is enabled.
    pub fn get_strict_mode(&self) -> bool {
        self.strict_mode
    }

    /// Gets a clone of the included suffixes list.
    pub fn get_include_suffixes(&self) -> Vec<String> {
        self.include_suffixes.clone()
    }

    /// Gets a clone of the excluded suffixes list.
    pub fn get_exclude_suffixes(&self) -> Vec<String> {
        self.exclude_suffixes.clone()
    }

    /// Gets a clone of the exclusion regex, if set.
    pub fn get_exclude_regex(&self) -> Option<Regex> {
        self.exclude_regex.clone()
    }
}