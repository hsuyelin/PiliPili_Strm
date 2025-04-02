//! File system monitoring and synchronization utilities.
//!
//! This module provides a hierarchical approach to filesystem operations with:
//! - Granular file-level operations
//! - Directory-level monitoring and synchronization
//! - Comprehensive filesystem watching capabilities
//!
//! # Core Components
//!
//! ## Directory Operations (`dir` module)
//! - [`Dir`] - Directory synchronization and batch operations
//! ## File Operations (`file` module)
//! - [`File`] - Enhanced file operations
//! ## Filesystem Monitoring (`watcher` module)
//! - [`Watcher`] - Real-time filesystem monitoring

pub mod dir;
pub mod file;
pub mod watcher;

pub use dir::*;
pub use file::*;
pub use watcher::*;