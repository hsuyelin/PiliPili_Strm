use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};

use serde::Serialize;

use crate::infrastructure::network::NetworkTask;

#[derive(Debug, Clone)]
pub enum PhotoInput {
    
    Url(String),

    FilePath(PathBuf),
}

#[derive(Debug, Clone, Serialize)]
pub struct PhotoMessage {

    #[serde(skip_serializing)]
    pub photo: PhotoInput,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

impl Display for PhotoInput {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PhotoInput::Url(url) => write!(f, "[URL] {}", url),
            PhotoInput::FilePath(path) => write!(f, "[File] {}", path.display()),
        }
    }
}

impl PhotoMessage {

    pub fn into_task(self, chat_id: String) -> NetworkTask {
        let mut fields = HashMap::new();
        fields.insert("chat_id".to_string(), chat_id);
        fields.insert("parse_mode".to_string(), "MarkdownV2".to_string());

        if let Some(caption) = self.caption {
            fields.insert("caption".to_string(), caption);
        }

        match self.photo {
            PhotoInput::FilePath(path) => {
                Some(vec![(path.to_string_lossy().into_owned(), "photo".to_string())])
            }
            PhotoInput::Url(url) => {
                fields.insert("photo".to_string(), url);
                None
            }
        };

        NetworkTask::RequestForm(fields)
    }
}

impl Display for PhotoMessage {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "PhotoMessage(photo: {}", self.photo)?;

        if let Some(caption) = &self.caption {
            write!(f, ", caption: {}", caption)?;
        }

        write!(f, ")")
    }
}