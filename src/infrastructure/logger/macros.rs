//! Provides convenient macros for logging at different levels.
//! 
//! This module exports macros that make it easy to log messages with different severity levels.
//! Each macro supports both a simple form (with just a message) and a form that includes a domain.

/// Log a message at the trace level.
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! trace_log {
    ($msg:expr) => {
        trace_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::trace!("{} {}", $domain, $msg);
    };
}

/// Log a message at the debug level.
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! debug_log {
    ($msg:expr) => {
        debug_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::debug!("{} {}", $domain, $msg);
    };
}

/// Log a message at the info level.
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! info_log {
    ($msg:expr) => {
        info_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::info!("{} {}", $domain, $msg);
    };
}

/// Log a message at the warn level.
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! warn_log {
    ($msg:expr) => {
        warn_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::warn!("{} {}", $domain, $msg);
    };
}

/// Log a message at the error level.
/// If no domain is specified, "[APP]" will be used as the default domain.
#[macro_export]
macro_rules! error_log {
    ($msg:expr) => {
        error_log!("[APP]", $msg);
    };
    ($domain:expr, $msg:expr) => {
        tracing::error!("{} {}", $domain, $msg);
    };
}