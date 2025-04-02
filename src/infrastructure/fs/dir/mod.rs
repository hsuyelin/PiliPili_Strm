//! Directory synchronization and file transfer utilities.
//!
//! This module provides a complete solution for local and remote file synchronization with:
//! - Cross-platform path handling
//! - SSH configuration and authentication
//! - Flexible sync configuration
//! - Progress tracking and reporting
//!
//! # Core Components
//!
//! - [`Location`] - Unified local/remote path representation (`location` module)
//! - [`SshConfig`] - SSH connection configuration (`ssh_config` module)  
//! - [`SyncConfig`] - Synchronization rules and parameters (`sync_config` module)
//! - [`SyncHelper`] - Main synchronization executor (`sync_helper` module)

pub mod location;
pub mod ssh_config;
pub mod sync_config;
pub mod sync_helper;

pub use location::*;
pub use ssh_config::*;
pub use sync_config::*;
pub use sync_helper::*;