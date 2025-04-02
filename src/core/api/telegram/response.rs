use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelegramResponse<T> {

    pub ok: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl<T: Display> Display for TelegramResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Status: {}", if self.ok { "OK" } else { "Failed" })?;

        if let Some(desc) = &self.description {
            write!(f, ", Description: {}", desc)?;
        }

        if let Some(res) = &self.result {
            write!(f, ", Result: {}", res)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageResult {

    pub message_id: i64,

    pub chat: Chat,

    pub text: Option<String>,
}

impl Display for MessageResult {
    
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "MessageID: {}, Chat: {}", self.message_id, self.chat)?;

        if let Some(text) = &self.text {
            write!(f, ", Text: {}", text)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Chat {

    pub id: i64,

    pub first_name: Option<String>,

    pub username: Option<String>,

    #[serde(rename = "type")]
    pub chat_type: String,
}

impl Display for Chat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ID: {}, Type: {}", self.id, self.chat_type)?;

        if let Some(name) = &self.first_name {
            write!(f, ", Name: {}", name)?;
        }

        if let Some(username) = &self.username {
            write!(f, ", Username: @{}", username)?;
        }

        Ok(())
    }
}