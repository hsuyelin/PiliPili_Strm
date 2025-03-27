use std::{
    process::{Command, Stdio},
    io::{BufReader, BufRead},
    path::Path
};

use anyhow::anyhow;
use regex::Regex;

use crate::infrastructure::fs::dir_sync_config::DirSyncConfig;

type ProgressCallback = Box<dyn Fn(&str) + Send + 'static>;
type FileSyncCallback = Box<dyn Fn(&str) + Send + 'static>;

pub struct DirSyncHelper {
    config: DirSyncConfig,
    progress_callback: Option<ProgressCallback>,
    file_sync_callback: Option<FileSyncCallback>,
}

impl DirSyncHelper {

    pub fn new(config: DirSyncConfig) -> Self {
        DirSyncHelper { 
            config,
            progress_callback: None,
            file_sync_callback: None,
        }
    }

    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        self.progress_callback = Some(callback);
    }

    pub fn set_file_sync_callback(&mut self, callback: FileSyncCallback) {
        self.file_sync_callback = Some(callback);
    }

    pub fn sync(&self) -> Result<(), anyhow::Error> {
        self.check_guard_file()?;
        self.check_source_dir()?;

        let mut cmd = self.build_rsync_command();
        let source = self.config.get_source_dir();
        let destination = self.get_destination_path();

        cmd.arg(&source).arg(&destination);
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

    fn check_guard_file(&self) -> Result<(), anyhow::Error> {
        if let Some(guard) = &self.config.get_guard_file() {
            if !Path::new(guard).exists() {
                return Err(anyhow!("Guard file '{}' does not exist, sync aborted.", guard));
            }
        }
        Ok(())
    }

    fn check_source_dir(&self) -> Result<(), anyhow::Error> {
        let source = self.config.get_source_dir();
        if !Path::new(&source).exists() {
            return Err(anyhow!("Source path '{}' does not exist, sync aborted.", source));
        }
        Ok(())
    }

    fn build_rsync_command(&self) -> Command {
        let mut cmd = Command::new("rsync");
        cmd.arg("-a")
            .arg("--progress")
            .arg("-v");

        if let Some(ssh_config) = &self.config.get_ssh_config() {
            cmd.arg("-e").arg(ssh_config.to_rsync_arg());
        }

        if self.config.get_strict_mode() {
            cmd.arg("--delete");
        }

        for suffix in &self.config.get_suffixes() {
            cmd.arg(format!("--exclude=*.{}", suffix));
        }

        if let Some(regex) = &self.config.get_ignore_regex() {
            if Regex::new(regex.as_str()).is_ok() {
                cmd.arg(format!("--exclude={}", regex.as_str()));
            } else {
                println!(
                    "Warning: Invalid regex pattern '{}', skipping exclude rule.",
                    regex.as_str()
                );
            }
        }

        cmd
    }

    fn get_destination_path(&self) -> String {
        if self.config.get_destination_dir().contains('@') {
            self.config.get_destination_dir()
        } else if let Some(ssh_config) = &self.config.get_ssh_config() {
            format!(
                "{}@{}:{}",
                ssh_config.username(),
                ssh_config.ip(),
                self.config.get_destination_dir()
            )
        } else {
            self.config.get_destination_dir()
        }
    }

    fn process_output(
        &self,
        stdout: std::process::ChildStdout,
        stderr: std::process::ChildStderr,
    ) -> Result<(), anyhow::Error> {
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        let mut stderr_output = String::new();

        for line in stdout_reader.lines() {
            let line = line?;
            if line.contains("to-chk") || line.contains("bytes/sec") {
                if let Some(ref cb) = self.progress_callback {
                    cb(&line);
                }
            } else if !line.starts_with(" ") && !line.is_empty() {
                if let Some(ref cb) = self.file_sync_callback {
                    cb(&line);
                }
            }
        }

        for line in stderr_reader.lines() {
            stderr_output.push_str(&line?);
            stderr_output.push('\n');
        }

        if !stderr_output.is_empty() {
            println!("rsync stderr: {}", stderr_output.trim());
        }

        Ok(())
    }
}