use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum SyncMethod {
    Rsync,
    RcloneCopy,
    RcloneSync,
}

impl Display for SyncMethod {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            SyncMethod::Rsync => "rsync",
            SyncMethod::RcloneCopy => "rclone copy",
            SyncMethod::RcloneSync => "rclone sync",
        };
        write!(f, "{}", str)
    }
}