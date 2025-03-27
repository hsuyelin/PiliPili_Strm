#[cfg(test)]
mod tests {

    use std::fs;
    use tempfile::tempdir;

    use pilipili_strm::infrastructure::fs::{
        file_helper::FileHelper, 
    };

    #[test]
    fn test_create_file_with_extension_existing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile.txt");
        let extension = "strm";

        fs::File::create(&file_path).unwrap();
        
        let new_file = FileHelper::create_file_with_extension(
            file_path.to_str().unwrap(),
            extension
        );

        assert!(new_file.is_some());
        let new_file_path = new_file.unwrap();
        assert!(new_file_path.exists());

        assert_eq!(new_file_path.extension().unwrap(), extension);
        fs::remove_file(new_file_path).unwrap();
    }

    #[test]
    fn test_create_file_with_extension_non_existent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("non_existent_file.txt");
        let extension = "strm";

        let new_file = FileHelper::create_file_with_extension(
            file_path.to_str().unwrap(), 
            extension
        );
        assert!(new_file.is_none());
    }

    #[test]
    fn test_create_file_with_extension_existing_file_with_suffix() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile.txt");
        let extension = "strm";

        fs::File::create(&file_path).unwrap();
        let new_file1 = FileHelper::create_file_with_extension(
            file_path.to_str().unwrap(), 
            extension
        );
        assert!(new_file1.is_some());

        let new_file2 = FileHelper::create_file_with_extension(
            file_path.to_str().unwrap(),
            extension
        );
        assert!(new_file2.is_some());
        let new_file2_path = new_file2.unwrap();
        assert!(new_file2_path.to_str().unwrap().contains("-1"));

        fs::remove_file(new_file1.unwrap()).unwrap();
        fs::remove_file(new_file2_path).unwrap();
    }
}