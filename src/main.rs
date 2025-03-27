use std::{
    path::PathBuf,
    time::Duration
};

use pilipili_strm::info_log;
use pilipili_strm::infrastructure::logger::*;
use pilipili_strm::infrastructure::fs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    LoggerBuilder::default()
        .init();

    let watch_path = PathBuf::from("~/Downloads/Tests");

    if !watch_path.exists() {
        std::fs::create_dir_all(&watch_path)?;
        info_log!(format!("Test directory created: {}", watch_path.display()));
    }

    let mut watcher = FileWatcher::new(&watch_path, Duration::from_secs(5));

    info_log!("======================================");
    info_log!("File monitoring test started");
    info_log!(format!("Monitoring path: {}", watch_path.display()));
    info_log!("Debounce time: 5 seconds");
    info_log!("--------------------------------------");
    info_log!("Please manually perform the following operations for testing:");
    info_log!("1. Create a new file in the directory");
    info_log!("2. Quickly modify the file content multiple times");
    info_log!("3. Delete the file");
    info_log!("======================================");

    watcher.setup_ctrlc_handler()?;
    info_log!("Press Ctrl+C to stop watching...");

    watcher.set_callback(move |_| {
        info_log!(format!("watch path changed: {}", watch_path.display()));
    });

    watcher.resume()?;

    while !watcher.get_should_exit() {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    watcher.stop();
    info_log!("Watcher stopped gracefully");
    Ok(())
}