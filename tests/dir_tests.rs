#[cfg(test)]
mod tests {

    use std::sync::mpsc::{channel, Receiver, Sender};

    use pilipili_strm::infrastructure::fs::*;

    fn mock_config(source: &str, destination: &str) -> DirSyncConfig {
        DirSyncConfig::builder()
            .with_source(DirLocation::new(source, true, None))
            .with_destination(DirLocation::new(destination, true, None))
            .with_strict_mode(false)
            .with_include_suffixes(vec!["strm"])
            .with_exclude_suffixes(vec!["aac", "ape", "flac"])
    }
    
    fn mock_server_config(source: &str, destination: &str) -> DirSyncConfig {
        let ssh_config = SshConfig::builder()
            .with_username("root".to_string())
            .with_password("123456".to_string())
            .with_ip("127.0.0.1".to_string());
        DirSyncConfig::builder()
            .with_source(DirLocation::new(source, true, None))
            .with_destination(DirLocation::new(destination, true, Some(ssh_config)))
            .with_strict_mode(false)
            .with_exclude_suffixes(vec!["aac", "ape", "flac"])
    }

    #[test]
    fn test_local_sync_with_callbacks() {
        let source_dir = tempfile::tempdir().unwrap();
        let dest_dir = tempfile::tempdir().unwrap();

        let config = mock_config(
            source_dir.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        );
        let mut sync_helper = DirSyncHelper::new(config);

        let (tx_progress, rx_progress): (Sender<String>, Receiver<String>) = channel();
        let (tx_file, rx_file): (Sender<String>, Receiver<String>) = channel();

        sync_helper.set_progress_callback(Box::new(move |progress| {
            println!("Progress: {}", progress);
            tx_progress.send(progress.to_string()).unwrap();
        }));
        sync_helper.set_file_sync_callback(Box::new(move |file| {
            println!("Sync file {}", file);
            tx_file.send(file.to_string()).unwrap();
        }));
        
        let result = sync_helper.sync();
        assert!(result.is_ok(), "Sync should succeed: {:?}", result.err());

        let progress_output = rx_progress.try_iter().collect::<Vec<_>>();
        let file_output = rx_file.try_iter().collect::<Vec<_>>();

        assert!(!progress_output.is_empty() || !file_output.is_empty(), "Callbacks should be triggered");
    }

    #[test]
    fn test_source_path_not_exist() {
        let config = mock_config("/nonexistent/source/", "/tmp/dest/");
        let sync_helper = DirSyncHelper::new(config);

        let result = sync_helper.sync();
        assert!(result.is_err(), "Sync should fail when source does not exist");
        if let Err(e) = result {
            assert!(e.to_string().contains("Source path '/nonexistent/source/' does not exist"));
        }
    }

    #[test]
    fn test_guard_file_not_exist() {
        let mut config = mock_config("/tmp/source/", "/tmp/dest/");
        let guard_file_option: Option<String> = Some("/nonexistent/guard.txt".to_string());
        if let Some(guard_file) = guard_file_option {
            config = config.with_guard_file(&guard_file);
        }

        let sync_helper = DirSyncHelper::new(config.clone());

        let result = sync_helper.sync();
        assert!(result.is_err(), "Sync should fail when guard file does not exist");
        if let Err(e) = result {
            assert!(e.to_string().contains("Guard file '/nonexistent/guard.txt' does not exist"));
        }
    }
    
    #[test]
    fn test_server_sync_with_callbacks() {
        let source_dir = tempfile::tempdir().unwrap();
        let dest_dir = tempfile::tempdir().unwrap();

        let config = mock_server_config(
            source_dir.path().to_str().unwrap(),
            dest_dir.path().to_str().unwrap(),
        );
        let sync_helper = DirSyncHelper::new(config);

        let _ = sync_helper.sync();
    }
}