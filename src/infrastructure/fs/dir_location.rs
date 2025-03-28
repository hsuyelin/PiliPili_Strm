use serde::Serialize;

use super::ssh_config::SshConfig;

#[derive(Clone, Debug, Serialize)]
pub struct DirLocation {

    path: String,
    is_dir: bool,
    ssh_config: Option<SshConfig>,
}

impl Default for DirLocation {
    
    fn default() -> Self {
        DirLocation {
            path: "".to_string(),
            is_dir: true,
            ssh_config: None,
        }
    }
}

impl DirLocation {
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

    pub fn get_path(&self) -> String {
        let base_path = if self.is_dir {
            format!("{}/", self.path.trim_end_matches('/'))
        } else {
            self.path.trim_end_matches('/').to_string()
        };

        if let Some(ssh_config) = &self.ssh_config {
            format!("{}@{}:{}", ssh_config.get_username(), ssh_config.get_ip(), base_path)
        } else {
            base_path
        }
    }

    pub fn ssh_config(&self) -> Option<&SshConfig> {
        self.ssh_config.as_ref()
    }

    pub fn to_rsync_arg(&self) -> Option<String> {
        self.ssh_config
            .as_ref()
            .and_then(|ssh| ssh.to_rsync_arg())
    }
}