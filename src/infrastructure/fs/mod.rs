//! File system watching and synchronization infrastructure.
//!
//! This module provides a comprehensive solution for monitoring filesystem changes
//! and performing directory synchronization with the following features:
//!
//! - **Cross-platform** monitoring of filesystem events
//! - **Configurable** watching behaviors and filters
//! - **Thread-safe** operation with proper state management
//! - **SSH support** for remote file operations
//! - **Synchronization** utilities for directory mirroring
//!
//! # Architecture Overview
//!
//! The system is organized into several core components:
//!
//! ## Watching Infrastructure
//! - [`Watcher`] - Main filesystem watcher implementation
//! - [`WatcherState`] - Tracks operational state (Running/Paused/Stopped)
//! - [`FileWatcherCallback`] - Event notification handler trait
//! - [`FileWatchable`] - Common interface for watcher implementations
//!
//! ## Utilities
//! - [`PathHelper`] - Cross-platform path manipulation utilities
//! - [`FileHelper`] - Filesystem operation utilities
//!
//! ## Synchronization
//! - [`DirSyncConfig`] - Configuration for directory synchronization
//! - [`DirSyncHelper`] - Main synchronization executor
//! - [`DirLocation`] - Local/remote path representation
//! - [`SshConfig`] - SSH connection configuration
//! 
pub mod watcher;
pub mod watcher_state;
pub mod watcher_callback;
pub mod watchable;
pub mod path_helper;
pub mod file_helper;
pub mod ssh_config;
pub mod dir_sync_config;
pub mod dir_sync_helper;
pub mod dir_location;

pub use watcher::*;
pub use watcher_callback::*;
pub use watcher_state::*;
pub use watchable::*;
pub use path_helper::*;
pub use file_helper::*;
pub use ssh_config::*;
pub use dir_sync_config::*;
pub use dir_sync_helper::*;
pub use dir_location::*;