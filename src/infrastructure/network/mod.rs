//! A flexible and extensible network request handling system.
//! 
//! This module provides a plugin-based architecture for making HTTP requests with the following features:
//! - Support for different HTTP methods
//! - Plugin system for request/response processing
//! - Curl-based implementation
//! - Task-based request handling

pub mod http_method;
pub mod task;
pub mod target;
pub mod provider;
pub mod plugin;
pub mod curl_plugin;

pub use http_method::HttpMethod;
pub use task::NetworkTask;
pub use target::NetworkTarget;
pub use provider::NetworkProvider;
pub use plugin::NetworkPlugin;
pub use curl_plugin::CurlPlugin;