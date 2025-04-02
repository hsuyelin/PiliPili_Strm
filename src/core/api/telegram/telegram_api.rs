use crate::core::config::Config;
use crate::infrastructure::network::{HttpMethod, NetworkTarget, NetworkTask};

use super::{
    PhotoMessage,
    TextMessage
};

pub enum TelegramAPI {

    SendMessage(TextMessage),

    SendPhoto(PhotoMessage),
}

impl NetworkTarget for TelegramAPI {

    fn base_url(&self) -> String {
        let token = Config::get().telegram.bot_token.clone();
        format!("https://api.telegram.org/bot{}", token)
    }

    fn path(&self) -> String {
        match self {
            TelegramAPI::SendMessage(_) => "sendMessage".to_string(),
            TelegramAPI::SendPhoto(_) => "sendPhoto".to_string(),
        }
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn task(&self) -> NetworkTask {
        match self {
            TelegramAPI::SendMessage(params) => params
                .clone()
                .into_task(self.get_chat_id()),

            TelegramAPI::SendPhoto(params) => params
                .clone()
                .into_task(self.get_chat_id())
        }
    }

    fn headers(&self) -> Option<Vec<(&'static str, String)>> {
        Some(vec![
            ("Content-Type", "application/json".to_string()),
            ("Accept", "application/json".to_string()),
            ("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36".to_string()),
        ])
    }
}

impl TelegramAPI {

    fn get_chat_id(&self) -> String {
        Config::get().telegram.chat_id.clone()
    }
}