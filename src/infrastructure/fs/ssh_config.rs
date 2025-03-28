use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct SshConfig {

    username: Option<String>,
    key_path: Option<String>,
    password: Option<String>,
    ip: String,
    port: Option<u16>,
}

impl Default for SshConfig {

    fn default() -> Self {
        SshConfig {
            username: None,
            key_path: None,
            password: None,
            ip: "127.0.0.1".to_string(),
            port: None
        }
    }
}

impl SshConfig {
    
    pub fn new() -> Self {
        SshConfig::default()
    }
    
    pub fn builder() -> Self {
        SshConfig::new()
    }
    
    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    
    pub fn with_key_path(mut self, key_path: String) -> Self {
        self.key_path = Some(key_path);
        self
    }
    
    pub fn with_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }
    
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn get_username(&self) -> &str {
        self.username.as_deref().unwrap_or("root")
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(22)
    }
    
    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    pub fn to_rsync_arg(&self) -> Option<String> {
        match (&self.key_path, &self.password) {
            (Some(key), None) => {
                if let Some(port) = self.port {
                    Some(format!("ssh -i {} -p {}", key, port))
                } else {
                    Some(format!("ssh -i {} -p 22", key))
                }
            }
            (None, Some(_)) => {
                if let Some(port) = self.port {
                    Some(format!("ssh -p {}", port))
                } else {
                    Some("ssh -p 22".to_string())
                }
            }
            (Some(key), Some(_)) => {
                if let Some(port) = self.port {
                    Some(format!("ssh -i {} -p {}", key, port))
                } else {
                    Some(format!("ssh -i {} -p 22", key))
                }
            }
            (None, None) => {
                None
            }
        }
    }
}