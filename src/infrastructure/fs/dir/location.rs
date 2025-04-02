use serde::Serialize;

use super::ssh_config::SshConfig;

/// Represents a filesystem location which could be either local or remote.
///
/// This struct encapsulates information about a directory or file path,
/// with optional SSH configuration for remote locations. It provides
/// convenience methods for path formatting and SSH-related operations.
#[derive(Clone, Debug, Serialize)]
pub struct DirLocation {

    /// The filesystem path (either local or remote)
    path: String,

    /// Flag indicating whether this location represents a directory
    is_dir: bool,

    /// Optional SSH configuration for remote locations
    ssh_config: Option<SshConfig>,
}

impl Default for DirLocation {

    /// Creates a default `DirLocation` instance.
    ///
    /// Returns a local directory location with:
    /// - Empty path string
    /// - `is_dir` set to `true`
    /// - No SSH configuration
    fn default() -> Self {
        DirLocation {
            path: "".to_string(),
            is_dir: true,
            ssh_config: None,
        }
    }
}

impl DirLocation {

    /// Creates a new `DirLocation` instance.
    ///
    /// # Arguments
    /// * `path` - Filesystem path (will be normalized by trimming trailing slashes)
    /// * `is_dir` - Whether the path represents a directory
    /// * `ssh_config` - Optional SSH configuration for remote paths
    pub fn new(
        path: &str,
        is_dir: bool,
        ssh_config: Option<SshConfig>
    ) -> Self {
        DirLocation {
            path: path.to_string(),
            is_dir,
            ssh_config,
        }
    }

    /// Gets the formatted path string for this location.
    ///
    /// For local paths, returns the normalized path (with trimmed trailing slashes).
    /// For remote paths, formats as `username@host:path`.
    ///
    /// Directories will have a single trailing slash added.
    pub fn get_path(&self) -> String {
        let base_path = if self.is_dir {
            format!("{}/", self.path.trim_end_matches('/'))
        } else {
            self.path.trim_end_matches('/').to_string()
        };

        if let Some(ssh_config) = &self.ssh_config {
            format!(
                "{}@{}:{}",
                ssh_config.get_username(),
                ssh_config.get_ip(),
                base_path
            )
        } else {
            base_path
        }
    }

    /// Returns a reference to the SSH configuration, if any.
    pub fn ssh_config(&self) -> Option<&SshConfig> {
        self.ssh_config.as_ref()
    }

    /// Converts the SSH configuration to rsync-compatible arguments.
    ///
    /// Returns `None` if no SSH configuration is present or if the
    /// configuration cannot be converted to rsync arguments.
    pub fn to_rsync_arg(&self) -> Option<String> {
        self.ssh_config
            .as_ref()
            .and_then(|ssh| ssh.to_rsync_arg())
    }
}