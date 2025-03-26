#[cfg(test)]
mod tests {

    use pilipili_strm::{debug_log, error_log, info_log, trace_log, warn_log};
    use pilipili_strm::infrastructure::logger::builder::LoggerBuilder;
    use pilipili_strm::infrastructure::logger::LogLevel;

    #[test]
    fn test_log() {
        LoggerBuilder::default()
            .with_level(LogLevel::Trace)
            .init();

        debug_log!("[DOMAIN1]", "This is a debug log.");
        info_log!("[DOMAIN2]", "This is a info log.");
        warn_log!("[DOMAIN3]", "This is a warn log.");
        error_log!("[DOMAIN4]", "This is a error log.");
        trace_log!("This is a trace log.");
    }
}