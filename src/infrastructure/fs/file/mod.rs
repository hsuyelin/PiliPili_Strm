//! Filesystem utility helpers for cross-platform operations.
//!
//! This module provides essential utilities for working with files and paths in a
//! platform-independent manner, including:
//! - Path manipulation and normalization
//! - File operations with consistent error handling
//! - Cross-platform path separator handling
//! 
pub mod file_helper;
pub mod path_helper;

pub use file_helper::*;
pub use path_helper::*;