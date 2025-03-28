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

    fn check_guard_file(&self) -> Result<(), Error> {
        if let Some(guard) = &self.config.get_guard_file() {
            if !Path::new(guard).exists() {
                return Err(anyhow!("Guard file '{}' does not exist, sync aborted.", guard));
            }
        }
        Ok(())
    }

    fn check_source_dir(&self) -> Result<(), Error> {
        let source_path = self.config.get_source().get_path();
        if self.config.get_source().ssh_config().is_none() && 
            !Path::new(&source_path).exists() {
            return Err(anyhow!("Source path '{}' does not exist, sync aborted.", source_path));
        }
        Ok(())
    }

    fn build_rsync_command(&self) -> Result<Command, Error> {
        let mut cmd = Command::new("rsync");
        cmd.arg("-a")
            .arg("--info=progress2")
            .arg("-v");

        let source_ssh = self.config.get_source().to_rsync_arg();
        let dest_ssh = self.config.get_destination().to_rsync_arg();

        if let Some(ssh_arg) = dest_ssh {
            cmd.arg("-e").arg(ssh_arg);
        } else if let Some(ssh_arg) = source_ssh {
            cmd.arg("-e").arg(ssh_arg);
        }

        if self.config.get_strict_mode() {
            cmd.arg("--delete");
        }

        if !self.config.get_include_suffixes().is_empty() {
            cmd.arg("--include=*/");
            for suffix in &self.config.get_include_suffixes() {
                cmd.arg(format!("--include=*.{}", suffix));
            }
            cmd.arg("--exclude=*");
        } else if !self.config.get_exclude_suffixes().is_empty() {
            for suffix in &self.config.get_exclude_suffixes() {
                cmd.arg(format!("--exclude=*.{}", suffix));
            }
        }

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

        let source_path = self.config.get_source().get_path();
        let dest_path = self.config.get_destination().get_path();
        cmd.arg(&source_path).arg(&dest_path);

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

        Ok(cmd)
    }

    fn process_output(
        &self,
        stdout: std::process::ChildStdout,
        stderr: std::process::ChildStderr,
    ) -> Result<(), Error> {
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        let mut stderr_output = String::new();
        
        for line in stdout_reader.lines() {
            let line = line?;
            match () {
                _ if line.contains("to-chk") || line.contains("bytes/sec") => {
                    if let Some(ref cb) = self.progress_callback {
                        cb(&line);
                    }
                }
                _ if !line.starts_with(" ") && 
                    !line.is_empty() && 
                    !line.starts_with("sent") &&
                    !line.starts_with("total size is") &&
                    !line.ends_with("sending incremental file list") &&
                    !line.ends_with("./") => {
                    if let Some(ref cb) = self.file_sync_callback {
                        cb(&line);
                    }
                }
                _ => {}
            }
        }

        for line in stderr_reader.lines() {
            stderr_output.push_str(&line?);
            stderr_output.push('\n');
        }

        if !stderr_output.is_empty() {
            info_log!(DIR_SYNC_LOGGER_DOMAIN, format!("Rsync stderr: {}", stderr_output.trim()));
        }

        Ok(())
    }
}