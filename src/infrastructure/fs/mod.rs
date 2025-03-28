//! File system watching infrastructure
//!
//! This module provides a complete solution for monitoring filesystem changes with:
//! - Cross-platform support
//! - Configurable watching behavior
//! - Thread-safe operation
//! - State management
//!
//! # Core Components
//! - [`Watcher`] - The main filesystem watcher implementation
//! - [`WatcherState`] - Tracks the operational state (Running/Paused/Stopped)
//! - [`FileWatcherCallback`] - Handles event notifications
//! - [`FileWatchable`] - Trait defining the watcher interface
//! - [`PathHelper`] - Utilities for path manipulation
//! - [`FileHelper`] - Utilities for file manipulation
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