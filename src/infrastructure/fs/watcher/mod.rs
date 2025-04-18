//! File system monitoring and change detection infrastructure.
//!
//! This module provides a comprehensive solution for real-time file system monitoring with:
//! - Cross-platform filesystem event notification
//! - Configurable event filtering
//! - State management for monitoring lifecycle
//! - Extensible callback system
//! 
pub mod callback;
pub mod state;
pub mod watchable;
pub mod watcher;

pub use callback::*;
pub use state::*;
pub use watchable::*;
pub use watcher::*;