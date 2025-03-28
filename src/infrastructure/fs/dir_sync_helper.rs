use std::{
    process::{Command, Stdio},
    io::{BufReader, BufRead},
    path::Path,
};
use anyhow::{Result, anyhow, Error};
use regex::Regex;

use crate::{info_log, debug_log, warn_log};
use super::dir_sync_config::DirSyncConfig;

/// Domain identifier for file sync logs
const DIR_SYNC_LOGGER_DOMAIN: &str = "[DIR-SYNC]";

/// Callback type for progress updates
type ProgressCallback = Box<dyn Fn(&str) + Send + 'static>;

/// Callback type for file sync notifications
type FileSyncCallback = Box<dyn Fn(&str) + Send + 'static>;

/// Helper for performing directory synchronization using rsync.
///
/// This struct manages the complete synchronization workflow including:
/// - Pre-sync validation checks
/// - Rsync command construction
/// - Process execution and output handling
/// - Progress and file sync callbacks
pub struct DirSyncHelper {

    /// Configuration for the sync operation
    config: DirSyncConfig,

    /// Optional callback for progress updates
    progress_callback: Option<ProgressCallback>,

    /// Optional callback for file sync notifications
    file_sync_callback: Option<FileSyncCallback>,
}

impl DirSyncHelper {

    /// Creates a new `DirSyncHelper` with the given configuration.
    pub fn new(config: DirSyncConfig) -> Self {
        DirSyncHelper {
            config,
            progress_callback: None,
            file_sync_callback: None,
        }
    }

    /// Sets a callback for receiving progress updates during sync.
    ///
    /// The callback will receive strings containing rsync's progress output.
    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        self.progress_callback = Some(callback);
    }

    /// Sets a callback for receiving file sync notifications.
    ///
    /// The callback will receive strings containing names of files being synced.
    pub fn set_file_sync_callback(&mut self, callback: FileSyncCallback) {
        self.file_sync_callback = Some(callback);
    }

    /// Performs the directory synchronization.
    ///
    /// # Steps
    /// 1. Validates guard file (if configured)
    /// 2. Checks source directory existence
    /// 3. Builds and executes rsync command
    /// 4. Processes output with callbacks
    ///
    /// # Errors
    /// Returns `anyhow::Error` if any step fails or rsync returns non-zero status.
    pub fn sync(&self) -> Result<(), Error> {
        self.check_guard_file()?;
        self.check_source_dir()?;

        let mut cmd = self.build_rsync_command()?;
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        let stdout = child.stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to capture stdout"))?;
        let stderr = child.stderr
            .take()
            .ok_or_else(|| anyhow!("Failed to capture stderr"))?;

        self.process_output(stdout, stderr)?;

        let exit_status = child.wait()?;
        if !exit_status.success() {
            return Err(anyhow!("rsync failed"));
        }

        Ok(())
    }

    /// Validates the guard file if configured.
    ///
    /// # Errors
    /// Returns error if guard file is required but doesn't exist.
    fn check_guard_file(&self) -> Result<(), Error> {
        if let Some(guard) = &self.config.get_guard_file() {
            if !Path::new(guard).exists() {
                return Err(anyhow!("Guard file '{}' does not exist, sync aborted.", guard));
            }
        }
        Ok(())
    }

    /// Validates the source directory exists (for local paths).
    ///
    /// # Errors
    /// Returns error if source path doesn't exist (only for local paths).
    fn check_source_dir(&self) -> Result<(), Error> {
        let source_path = self.config.get_source().get_path();
        if self.config.get_source().ssh_config().is_none() &&
            !Path::new(&source_path).exists() {
            return Err(anyhow!("Source path '{}' does not exist, sync aborted.", source_path));
        }
        Ok(())
    }

    /// Constructs the rsync command based on configuration.
    ///
    /// # Returns
    /// Configured `Command` ready for execution.
    ///
    /// # Notes
    /// - Handles both local and remote paths
    /// - Applies to include/exclude filters
    /// - Configures strict mode if enabled
    /// - Logs the final command for debugging
    fn build_rsync_command(&self) -> Result<Command, Error> {
        let mut cmd = Command::new("rsync");
        cmd.arg("-a")            // Archive mode (preserve attributes)
            .arg("--info=progress2")  // Show progress information
            .arg("-v");          // Verbose output

        // Configure SSH options if needed
        let source_ssh = self.config.get_source().to_rsync_arg();
        let dest_ssh = self.config.get_destination().to_rsync_arg();

        // Prefer destination SSH config if both exist
        if let Some(ssh_arg) = dest_ssh {
            cmd.arg("-e").arg(ssh_arg);
        } else if let Some(ssh_arg) = source_ssh {
            cmd.arg("-e").arg(ssh_arg);
        }

        // Configure strict mode (delete extraneous files)
        if self.config.get_strict_mode() {
            cmd.arg("--delete");
        }

        // Handle include/exclude filters
        if !self.config.get_include_suffixes().is_empty() {
            cmd.arg("--include=*/");  // Always include directories
            for suffix in &self.config.get_include_suffixes() {
                cmd.arg(format!("--include=*.{}", suffix));
            }
            cmd.arg("--exclude=*");  // Exclude everything else
        } else if !self.config.get_exclude_suffixes().is_empty() {
            for suffix in &self.config.get_exclude_suffixes() {
                cmd.arg(format!("--exclude=*.{}", suffix));
            }
        }

        // Handle regex excludes
        if let Some(regex) = &self.config.get_exclude_regex() {
            if let Ok(_re) = Regex::new(regex.as_str()) {
                cmd.arg(format!("--exclude={}", regex.as_str()));
            } else {
                warn_log!(
                    DIR_SYNC_LOGGER_DOMAIN, 
                    format!(
                        "Warning: Invalid regex pattern '{}', \
                        skipping exclude rule.",
                        regex.as_str()
                    )
                );
            }
        }

        // Set source and destination paths
        let source_path = self.config.get_source().get_path();
        let dest_path = self.config.get_destination().get_path();
        cmd.arg(&source_path).arg(&dest_path);

        self.print_sync_command(&mut cmd);

        Ok(cmd)
    }

    /// Formats and logs the rsync command being executed for debugging purposes.
    ///
    /// This function reconstructs the command string from the `Command` object,
    /// properly handling quoted arguments (especially the SSH `-e` option) to
    /// produce an executable-equivalent string for logging.
    ///
    /// # Arguments
    /// * `cmd` - The `Command` object representing the rsync operation
    ///
    /// # Example Output
    /// ```text
    /// rsync -a -v -e "ssh -i ~/.ssh/id_rsa -p 22" /source/path/ user@host:/dest/path
    /// ```
    ///
    /// # Notes
    /// - Special handling for SSH `-e` option to keep its argument quoted
    /// - Other arguments are joined with simple spaces
    /// - Output is logged at debug level with DIR_SYNC domain
    fn print_sync_command(&self, cmd: &mut Command) {
        // Format command for logging
        let mut cmd_parts = vec![cmd.get_program().to_string_lossy().into_owned()];
        let args: Vec<_> = cmd
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        let mut i = 0;
        while i < args.len() {
            if args[i] == "-e" && i + 1 < args.len() {
                cmd_parts.push(format!("-e \"{}\"", args[i + 1]));
                i += 2;
            } else {
                cmd_parts.push(args[i].clone());
                i += 1;
            }
        }
        let cmd_string = cmd_parts.join(" ");
        debug_log!(DIR_SYNC_LOGGER_DOMAIN, format!("Executing command: {}", cmd_string));
    }

    /// Processes rsync output streams and invokes callbacks.
    ///
    /// # Arguments
    /// * `stdout` - Child process stdout pipe
    /// * `stderr` - Child process stderr pipe
    ///
    /// # Behavior
    /// - Progress updates are sent to progress callback
    /// - File sync notifications are sent to file sync callback
    /// - Error output is logged
    fn process_output(
        &self,
        stdout: std::process::ChildStdout,
        stderr: std::process::ChildStderr,
    ) -> Result<(), Error> {
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        let mut stderr_output = String::new();

        // Process stdout line by line
        for line in stdout_reader.lines() {
            let line = line?;
            match () {
                _ if Self::check_file_sync_progress(&line) => {
                    // Progress information
                    if let Some(ref cb) = self.progress_callback {
                        cb(&line);
                    }
                }
                _ if Self::check_file_sync_line(&line) => {
                    // File being synced
                    if let Some(ref cb) = self.file_sync_callback {
                        cb(&line);
                    }
                }
                _ => {}
            }
        }

        // Collect stderr output
        for line in stderr_reader.lines() {
            stderr_output.push_str(&line?);
            stderr_output.push('\n');
        }

        // Log any stderr output
        if !stderr_output.is_empty() {
            info_log!(DIR_SYNC_LOGGER_DOMAIN, format!("Rsync stderr: {}", stderr_output.trim()));
        }

        Ok(())
    }

    /// Determines if a line from rsync output represents progress information.
    ///
    /// This checks for rsync's progress format that shows transfer statistics,
    /// typically containing either "to-chk" (remaining files) or "bytes/sec" (transfer speed).
    ///
    /// # Arguments
    /// * `line` - A line of output from the rsync command
    ///
    /// # Returns
    /// `true` if the line contains progress information, `false` otherwise
    fn check_file_sync_progress(line: &String) -> bool {
        (line.contains("to-chk") || line.contains("bytes/sec")) &&
            !(line.contains("sent") && line.contains("received"))
    }

    /// Determines if a line from rsync output represents a file being synced.
    ///
    /// Filters out summary lines, empty lines, and other non-file output from rsync.
    ///
    /// # Arguments
    /// * `line` - A line of output from the rsync command
    ///
    /// # Returns
    /// `true` if the line represents a file being transferred, `false` otherwise
    fn check_file_sync_line(line: &String) -> bool {
        !line.starts_with(" ") &&
            !line.is_empty() &&
            !line.starts_with("total size is") &&
            !(line.contains("sent") && line.contains("received")) &&
            !line.ends_with("sending incremental file list") &&
            !line.ends_with("./")
    }
}