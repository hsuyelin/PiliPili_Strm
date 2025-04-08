//! Telegram Bot API integration and message handling.
//!
//! This module provides a complete interface for building Telegram bots with:
//! - Full API client implementation
//! - Support for different message types
//! - Response handling and error management
//! - Markdown formatting utilities
//! 
pub mod telegram_api;
pub mod photo_message;
pub mod telegram_response;
pub mod text_message;

pub use telegram_api::*;
pub use photo_message::*;
pub use telegram_response::*;
pub use text_message::*;