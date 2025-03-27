use std::path::{
    Path, 
    PathBuf
};

use dirs;

/// A helper struct for common path operations with cross-platform support
pub struct PathHelper;

impl PathHelper {

    /// Expands the tilde (`~`) in a path to the user's home directory.
    ///
    /// # Arguments
    /// * `path` - The path to expand
    ///
    /// # Returns
    /// The expanded path if it starts with `~`, otherwise the original path
    ///
    /// # Platform Notes
    /// - Linux/macOS: Expands to `$HOME` environment variable value
    /// - Windows: Expands to `%USERPROFILE%` environment variable value
    pub fn expand_tilde(path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();
        if path.starts_with("~") {
            if path == Path::new("~") {
                return Self::home_dir().unwrap_or_else(|| path.to_path_buf());
            }
            if let Ok(stripped) = path.strip_prefix("~/") {
                return Self::home_dir()
                    .map(|home| home.join(stripped))
                    .unwrap_or_else(|| path.to_path_buf());
            }
        }
        path.to_path_buf()
    }

    /// Returns the user's home directory
    ///
    /// # Platform-specific Paths
    /// - Linux: `$HOME` (typically `/home/username`)
    /// - macOS: `/Users/username`
    /// - Windows: `C:\Users\username` or `C:\Documents and Settings\username` (older versions)
    ///
    /// # Returns
    /// Some(PathBuf) if the home directory could be determined, None otherwise
    pub fn home_dir() -> Option<PathBuf> {
        dirs::home_dir()
    }

    /// Returns the user's desktop directory
    ///
    /// # Platform-specific Paths
    /// - Linux: `$HOME/Desktop` (varies by distribution)
    /// - macOS: `/Users/username/Desktop`
    /// - Windows: `C:\Users\username\Desktop`
    ///
    /// # Returns
    /// Some(PathBuf) if the desktop directory could be determined, None otherwise
    pub fn desktop_dir() -> Option<PathBuf> {
        dirs::desktop_dir()
    }

    /// Returns the user's documents directory
    ///
    /// # Platform-specific Paths
    /// - Linux: `$HOME/Documents` (common but not standardized)
    /// - macOS: `/Users/username/Documents`
    /// - Windows: `C:\Users\username\Documents`
    ///
    /// # Returns
    /// Some(PathBuf) if the documents directory could be determined, None otherwise
    pub fn documents_dir() -> Option<PathBuf> {
        dirs::document_dir()
    }

    /// Returns the user's configuration directory
    ///
    /// # Platform-specific Paths
    /// - Linux: `$XDG_CONFIG_HOME` (default: `$HOME/.config`)
    /// - macOS: `/Users/username/Library/Application Support`
    /// - Windows: `C:\Users\username\AppData\Roaming`
    ///
    /// # Returns
    /// Some(PathBuf) if the config directory could be determined, None otherwise
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir()
    }

    /// Joins two path components with platform-specific separator
    ///
    /// # Arguments
    /// * `base` - The base path
    /// * `part` - The component to join to the base path
    ///
    /// # Platform Notes
    /// - Linux/macOS: Uses forward slash (`/`)
    /// - Windows: Uses backslash (`\`) but accepts forward slashes too
    ///
    /// # Returns
    /// The joined path
    pub fn join<P: AsRef<Path>>(base: impl AsRef<Path>, part: P) -> PathBuf {
        base.as_ref().join(part)
    }

    /// Extracts the file stem (name without extension) from a path
    ///
    /// # Arguments
    /// * `path` - The path to examine
    ///
    /// # Platform Notes
    /// - Handles both Unix-style and Windows-style paths
    /// - Case sensitivity depends on filesystem (Linux: usually sensitive, Windows/macOS: usually insensitive)
    ///
    /// # Returns
    /// Some(String) containing the file stem if present, None otherwise
    pub fn file_stem(path: impl AsRef<Path>) -> Option<String> {
        path.as_ref()
            .file_stem()?
            .to_str()
            .map(|s| s.to_string())
    }

    /// Extracts the file extension from a path
    ///
    /// # Arguments
    /// * `path` - The path to examine
    ///
    /// # Platform Notes
    /// - On Windows, compares extensions case-insensitively
    /// - On Linux, compares extensions case-sensitively
    /// - macOS typically uses case-insensitive comparison but preserves case
    ///
    /// # Returns
    /// Some(String) containing the extension if present, None otherwise
    pub fn extension(path: impl AsRef<Path>) -> Option<String> {
        path.as_ref()
            .extension()?
            .to_str()
            .map(|s| s.to_string())
    }

    /// Normalizes a path by removing redundant components
    ///
    /// # Arguments
    /// * `path` - The path to normalize
    ///
    /// # Platform-specific Behavior
    /// - On Windows, preserves the verbatim prefix if present
    /// - On all platforms, resolves `.` and `..` components
    /// - Handles both forward and backward slashes on Windows
    ///
    /// # Returns
    /// The normalized path
    pub fn normalize(path: impl AsRef<Path>) -> PathBuf {
        let mut result = PathBuf::new();
        for component in path.as_ref().components() {
            match component {
                std::path::Component::Prefix(p) => result.push(p.as_os_str()),
                std::path::Component::RootDir => result.push("/"),
                std::path::Component::CurDir => {},
                std::path::Component::ParentDir => { result.pop(); },
                std::path::Component::Normal(p) => result.push(p),
            }
        }
        result
    }
}