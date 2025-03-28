use std::{
    path::PathBuf,
    time::Duration,
};

use pilipili_strm::info_log;
use pilipili_strm::infrastructure::logger::*;
use pilipili_strm::infrastructure::fs::*;

fn init_logger() {
    LoggerBuilder::default()
        .with_level(LogLevel::Debug)
        .init();
}

fn ensure_test_directory(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        info_log!(format!("Test directory created: {}", path.display()));
    }
    Ok(())
}

fn configure_watcher(
    watch_path: &PathBuf,
    debounce_duration: Duration,
) -> FileWatcher {
    let watcher = FileWatcher::new(watch_path, debounce_duration);
    watcher
}

fn setup_sync_callback(
    watcher: &mut FileWatcher,
    watch_path: PathBuf,
    sync_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let sync_path_clone = sync_path.clone();
    watcher.set_callback(move |_| {
        if let Err(e) = sync_directories(&watch_path, &sync_path_clone) {
            info_log!(format!("Sync failed: {}", e));
        } else {
            info_log!(format!(
                "Synced {} => {} complete!",
                watch_path.display(),
                sync_path_clone.display()
            ));
        }
    });
    Ok(())
}

fn sync_directories(
    source: &PathBuf,
    destination: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_owned = source.clone();
    let dest_owned = destination.clone();
    let config = DirSyncConfig::builder()
        .with_source(DirLocation::new(&source.to_string_lossy(), true, None))
        .with_destination(DirLocation::new(&destination.to_string_lossy(), true, None))
        .with_strict_mode(false)
        .with_include_suffixes(vec!["strm"])
        .with_exclude_suffixes(vec!["txt"]);

    info_log!(format!("Dir sync config: {}", config));

    let mut sync_helper = DirSyncHelper::new(config);

    sync_helper.set_progress_callback(Box::new(move |progress| {
        info_log!(format!("Sync progress: {}", progress));
    }));

    sync_helper.set_file_sync_callback(Box::new(move |file| {
        let message = format!(
            "{} => {}",
            source_owned.join(file).display(),
            dest_owned.join(file).display()
        );
        info_log!(message);
    }));

    sync_helper.sync()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let watch_path = PathHelper::expand_tilde(
        PathBuf::from("~/Downloads/Tests")
    );
    let sync_path = PathHelper::expand_tilde(
        PathBuf::from("~/Downloads/Sync_Tests")
    );

    ensure_test_directory(&watch_path)?;
    
    let mut watcher = configure_watcher(
        &watch_path,
        Duration::from_secs(5)
    );

    setup_sync_callback(&mut watcher, watch_path.clone(), sync_path.clone())?;
    watcher.resume()?;
    info_log!(format!("Syncing path: {}", sync_path.display()));

    watcher.setup_ctrlc_handler()?;
    info_log!("Press Ctrl+C to stop watching...");

    while !watcher.get_should_exit() {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    watcher.stop();
    info_log!("Watcher stopped gracefully");

    Ok(())
}