use serde::Serialize;

/// Configuration for SSH connection parameters.
///
/// This struct encapsulates all necessary parameters to establish an SSH connection,
/// supporting both key-based and password authentication. It provides a builder pattern
/// for convenient configuration and methods to generate appropriate connection strings.
#[derive(Clone, Debug, Serialize)]
pub struct SshConfig {

    /// SSH username (defaults to "root" if not specified)
    username: Option<String>,

    /// Path to private key file for authentication
    key_path: Option<String>,

    /// Password for authentication (use with caution)
    password: Option<String>,

    /// IP address or hostname of the remote server
    ip: String,

    /// SSH port number (defaults to 22 if not specified)
    port: Option<u16>,
}

impl Default for SshConfig {

    /// Creates a default `SshConfig` with:
    /// - No username (defaults to "root" when used)
    /// - No authentication method
    /// - IP set to localhost (127.0.0.1)
    /// - Default SSH port (22)
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

    /// Creates a new `SshConfig` with default values.
    pub fn new() -> Self {
        SshConfig::default()
    }

    /// Starts a builder pattern chain for configuration.
    pub fn builder() -> Self {
        SshConfig::new()
    }

    /// Sets the SSH username (builder pattern).
    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Sets the path to SSH private key (builder pattern).
    pub fn with_key_path(mut self, key_path: String) -> Self {
        self.key_path = Some(key_path);
        self
    }

    /// Sets the SSH password (builder pattern).
    ///
    /// # Security Note
    /// Storing passwords in memory should be done with caution. Consider using
    /// key-based authentication when possible.
    pub fn with_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    /// Sets the remote server IP or hostname (builder pattern).
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }

    /// Sets the SSH port number (builder pattern).
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Gets the SSH username, defaults to "root" if not specified.
    pub fn get_username(&self) -> &str {
        self.username.as_deref().unwrap_or("root")
    }

    /// Gets the remote server IP or hostname.
    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    /// Gets the SSH port number, defaults to 22 if not specified.
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(22)
    }

    /// Gets the SSH password if set.
    ///
    /// # Security Note
    /// Be cautious when handling or displaying password values.
    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    /// Checks if password authentication is configured.
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    /// Generates rsync-compatible SSH arguments based on configuration.
    ///
    /// Returns `None` if neither key nor password authentication is configured.
    /// When both key and password are configured, the key takes precedence.
    pub fn to_rsync_arg(&self) -> Option<String> {
        match (&self.key_path, &self.password) {
            (Some(key), None) => {
                Some(format!(
                    "ssh -i {} -p {}",
                    key,
                    self.port.unwrap_or(22)
                ))
            }
            (None, Some(_)) => {
                Some(format!(
                    "ssh -p {}",
                    self.port.unwrap_or(22)
                ))
            }
            (Some(key), Some(_)) => {
                // Key takes precedence when both are present
                Some(format!(
                    "ssh -i {} -p {}",
                    key,
                    self.port.unwrap_or(22)
                ))
            }
            (None, None) => None,
        }
    }
}