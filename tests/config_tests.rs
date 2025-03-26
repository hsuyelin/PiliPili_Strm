#[cfg(test)]
mod tests {
 
    use std::sync::RwLockReadGuard;
    use pilipili_strm::infrastructure::config::Config;

    #[test]
    fn test_load_existing_config() {
        let config: RwLockReadGuard<'static, Config> = Config::get();

        assert!(!config.emby.base_url.is_empty(), "Base URL should not be empty");
        assert!(!config.emby.api_key.is_empty(), "API key should not be empty");
    }
}