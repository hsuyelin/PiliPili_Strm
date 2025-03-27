use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf}
};

/// Provides utility methods for file operations
pub struct FileHelper;

impl FileHelper {

    /// Creates a new file with the specified extension while avoiding name conflicts
    ///
    /// # Arguments
    /// * `file_path` - Path to the original file (must exist)
    /// * `extension` - New extension to use (without leading dot)
    ///
    /// # Returns
    /// - `Some(PathBuf)` containing the path to the newly created file
    /// - `None` if:
    ///   - Original file doesn't exist
    ///   - Canonicalization fails
    ///   - File creation fails
    ///   - Writing fails
    ///
    /// # Behavior
    /// 1. Verifies original file exists
    /// 2. Creates new file with same name but different extension
    /// 3. If name exists, appends incrementing numbers (-1, -2, etc.)
    /// 4. Writes original file's absolute path into new file
    pub fn create_file_with_extension(
        file_path: &str, 
        extension: &str
    ) -> Option<PathBuf> {
        let path = Path::new(file_path);

        if !path.exists() {
            return None;
        }

        let absolute_path = fs::canonicalize(path).ok()?;
        let mut new_file_path = absolute_path.with_extension(extension);

        // Handle naming conflicts by appending incrementing numbers
        let mut count = 1;
        while new_file_path.exists() {
            let file_stem = new_file_path
                .file_stem()
                .unwrap()
                .to_string_lossy();
            let new_stem = format!("{}-{}", file_stem, count);
            new_file_path = absolute_path
                .with_file_name(new_stem)
                .with_extension(extension);
            count += 1;
        }

        // Create file and write original path
        match File::create(&new_file_path) {
            Ok(mut file) => {
                if let Err(_) = writeln!(file, "{}", absolute_path.display()) {
                    return None;
                }
                Some(new_file_path)
            }
            Err(_) => None,
        }
    }
}