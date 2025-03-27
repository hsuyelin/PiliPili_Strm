#[derive(Clone, Debug)]
pub struct SshConfig {

    username: Option<String>,
    key_path: String,
    ip: String,
    port: Option<u16>,
}

impl SshConfig {

    pub fn new(ip: &str, key_path: &str, username: Option<&str>, port: Option<u16>) -> Self {
        SshConfig {
            username: username.map(String::from).or(Some("root".to_string())),
            key_path: key_path.to_string(),
            ip: ip.to_string(),
            port,
        }
    }
    
    pub fn to_rsync_arg(&self) -> String {
        if let Some(port) = self.port {
            format!("ssh -i {} -p {}", self.key_path, port)
        } else {
            format!("ssh -i {}", self.key_path)
        }
    }

    pub fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("root")
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(22)
    }
}