use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};

use serde::Serialize;
use crate::infrastructure::network::NetworkTask;

#[derive(Debug, Clone, Serialize)]
pub struct TextMessage {

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<String>,
}

impl Display for TextMessage {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "text={}", self.text)
    }
}

impl TextMessage {

    pub fn to_json_value(&self, chat_id: String) -> serde_json::Value {
        let mut value = serde_json::to_value(self).unwrap();
        if let Some(obj) = value.as_object_mut() {
            obj.entry("parse_mode")
                .or_insert_with(|| "MarkdownV2".into());
            obj.entry("chat_id")
                .or_insert_with(|| chat_id.into());
        }
        value
    }

    pub fn into_task(self, chat_id: String) -> NetworkTask {
        NetworkTask::RequestJson(self.to_json_value(chat_id))
    }
}