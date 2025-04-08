//! A flexible and configurable logging system for Rust applications.
//! 
//! This module provides a comprehensive logging solution with the following features:
//! - Configurable log levels
//! - Log rotation support
//! - Builder pattern for easy configuration
//! - Convenient macros for logging
//! 
pub mod builder;
pub mod rotation;
pub mod level;
pub mod macros;

pub use builder::*;
pub use rotation::*;
pub use level::*;