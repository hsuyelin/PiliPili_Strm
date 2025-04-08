use crate::{
    core::config::Config,
    infrastructure::network::{HttpMethod, NetworkTarget, NetworkTask}
};

use super::{PhotoMessage, TextMessage};

/// The base URL for the Telegram API, used to construct requests to the Telegram Bot API.
/// This constant provides the root address, to be concatenated with a bot token and specific endpoints.
const TELEGRAM_API_BASE: &str = "https://api.telegram.org/bot";

/// Represents Telegram Bot API endpoints with their respective parameters.
///
/// This enum encapsulates all supported Telegram API operations,
/// providing a type-safe way to construct API requests.
#[derive(Debug, Clone)]
pub enum TelegramAPI {

    /// Send a text message to a chat
    SendMessage(TextMessage),

    /// Send a photo to a chat
    SendPhoto(PhotoMessage),
}

impl NetworkTarget for TelegramAPI {

    /// Gets the base URL for Telegram API requests.
    ///
    /// Constructs the URL using the bot token from configuration.
    fn base_url(&self) -> String {
        let token = Config::get().telegram.bot_token.clone();
        format!("{}{}", TELEGRAM_API_BASE, token)
    }

    /// Gets the API endpoint path for the specific operation.
    fn path(&self) -> String {
        match self {
            TelegramAPI::SendMessage(_) => "sendMessage".to_string(),
            TelegramAPI::SendPhoto(_) => "sendPhoto".to_string(),
        }
    }

    /// Gets the HTTP method for the request (always POST for Telegram API).
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    /// Converts the API operation into a network task ready for execution.
    ///
    /// # Returns
    /// A `NetworkTask` containing all necessary request parameters.
    fn task(&self) -> NetworkTask {
        match self {
            TelegramAPI::SendMessage(params) => params
                .clone()
                .into_task(self.get_chat_id()),
            TelegramAPI::SendPhoto(params) => params
                .clone()
                .into_task(self.get_chat_id()),
        }
    }

    /// Gets the default headers for Telegram API requests.
    ///
    /// Includes:
    /// - Standard JSON content type headers
    /// - User agent string
    fn headers(&self) -> Option<Vec<(&'static str, String)>> {
        Some(vec![
            ("Content-Type", "application/json".to_string()),
            ("Accept", "application/json".to_string()),
            ("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36".to_string()),
        ])
    }
}

impl TelegramAPI {

    /// Gets the target chat ID from configuration.
    ///
    /// This is used as the default destination for all messages.
    fn get_chat_id(&self) -> String {
        Config::get().telegram.chat_id.clone()
    }
}