use crate::infrastructure::network::{NetworkProvider, CurlPlugin};

use crate::core::api::{
    telegram::{TextMessage, PhotoMessage, TelegramAPI, TelegramResponse, MessageResult},
};

pub struct TelegramClient;

impl TelegramClient {

    pub async fn send_message(
        params: TextMessage,
    ) -> Result<TelegramResponse<MessageResult>, anyhow::Error> {
        let provider = NetworkProvider::new(vec![Box::new(CurlPlugin)]);
        let response = provider.send_request(&TelegramAPI::SendMessage(params)).await?;
        let result: TelegramResponse<MessageResult> = response.json().await?;
        Ok(result)
    }

    pub async fn send_photo(
        params: PhotoMessage,
    ) -> Result<TelegramResponse<MessageResult>, anyhow::Error> {
        let provider = NetworkProvider::new(vec![Box::new(CurlPlugin)]);
        let response = provider.send_request(&TelegramAPI::SendPhoto(params)).await?;
        let result: TelegramResponse<MessageResult> = response.json().await?;
        Ok(result)
    }
}