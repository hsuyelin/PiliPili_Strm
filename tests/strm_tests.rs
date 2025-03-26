use std::path::Path;
use tempfile::tempdir;
use tokio::fs;

use pilipili_strm::infrastructure::strm::{
    file_sync::FileSync,
    sync_config::SyncConfig,
};

#[tokio::test]
async fn test_strm_generation() {
    let temp_dir = tempdir().unwrap();
    let test_dir = temp_dir.path();

    let video_file = test_dir.join("test_video.mp4");
    fs::write(&video_file, "fake video content").await.unwrap();

    let audio_file = test_dir.join("test_audio.mp3");
    fs::write(&audio_file, "fake audio content").await.unwrap();

    let ignore_file = test_dir.join("ignore.txt");
    fs::write(&ignore_file, "should be ignored").await.unwrap();

    let config = SyncConfig {
        video_extensions: vec!["mp4".to_string(), "mkv".to_string()],
        audio_extensions: vec!["mp3".to_string(), "flac".to_string()],
        ignore_extensions: vec!["txt".to_string()],
        ..Default::default()
    };

    let file_sync = FileSync::new(config).unwrap();

    let strm_path = file_sync.get_generator().generate_strm(&video_file).await.unwrap();
    assert!(strm_path.exists());
    assert_eq!(strm_path.extension().unwrap(), "strm");

    let content = fs::read_to_string(&strm_path).await.unwrap();
    assert_eq!(content, video_file.to_str().unwrap());

    let results = file_sync.get_generator().generate_strm_for_dir(test_dir).await.unwrap();
    assert_eq!(results.len(), 2);

    let ignore_strm = test_dir.join("ignore.strm");
    assert!(!ignore_strm.exists());
}

#[tokio::test]
async fn test_strm_generation_in_subdirs() {
    let temp_dir = tempdir().unwrap();
    let test_dir = temp_dir.path();

    let sub_dir = test_dir.join("subdir");
    fs::create_dir(&sub_dir).await.unwrap();

    let video_file = sub_dir.join("sub_video.mp4");
    fs::write(&video_file, "sub video").await.unwrap();

    let config = SyncConfig::default();
    let file_sync = FileSync::new(config).unwrap();

    let results = file_sync.get_generator().generate_strm_for_dir(test_dir).await.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].to_str().unwrap().contains("sub_video.strm"));
}

#[tokio::test]
async fn test_existing_strm_file() {
    let temp_dir = tempdir().unwrap();
    let video_file = temp_dir.path().join("existing.mp4");
    fs::write(&video_file, "content").await.unwrap();

    let existing_strm = temp_dir.path().join("existing.strm");
    fs::write(&existing_strm, "old content").await.unwrap();

    let file_sync = FileSync::new(SyncConfig::default()).unwrap();
    let result = file_sync.get_generator().generate_strm(&video_file).await.unwrap();

    let content = fs::read_to_string(&result).await.unwrap();
    assert_eq!(content, "old content");
}

#[tokio::test]
async fn test_file_sync_with_strm() {
    let src_dir = tempdir().unwrap();
    let dest_dir = tempdir().unwrap();

    create_media_library_structure(src_dir.path()).await;
    let video_file = src_dir.path().join("video.mp4");
    fs::write(&video_file, "test").await.unwrap();

    let config = SyncConfig::default();
    let file_sync = FileSync::new(config).unwrap();

    file_sync.sync_directory(src_dir.path(), dest_dir.path(), "copy")
        .await
        .unwrap();

    let strm_file = dest_dir.path().join("video.strm");
    assert!(strm_file.exists());

    let content = fs::read_to_string(&strm_file).await.unwrap();
    assert_eq!(content, video_file.to_str().unwrap());
}

async fn create_media_library_structure(base_dir: &Path) {
    let show_dir = base_dir.join("TV Shows/The Simpsons");
    fs::create_dir_all(&show_dir).await.unwrap();

    fs::write(show_dir.join("tvshow.nfo"), "<tvshow><title>The Simpsons</title></tvshow>").await.unwrap();
    fs::write(show_dir.join("poster.jpg"), "fake_poster_data").await.unwrap();

    let season_dir = show_dir.join("Season 1");
    fs::create_dir_all(&season_dir).await.unwrap();
    fs::write(season_dir.join("season.nfo"), "<season><title>Season 1</title></season>").await.unwrap();

    fs::write(season_dir.join("S01E01.mp4"), "fake_video_data").await.unwrap();
    fs::write(season_dir.join("S01E01.nfo"), "<episode><title>Simpsons Roasting</title></episode>").await.unwrap();

    let movie_dir = base_dir.join("Movies/Inception (2010)");
    fs::create_dir_all(&movie_dir).await.unwrap();
    fs::write(movie_dir.join("movie.nfo"), "<movie><title>Inception</title></movie>").await.unwrap();
    fs::write(movie_dir.join("fanart.jpg"), "fake_fanart_data").await.unwrap();
    fs::write(movie_dir.join("Inception (2010).mp4"), "fake_movie_data").await.unwrap();
}