#[cfg(test)]
mod tests {

    use tokio;

    use pilipili_strm::{
        core::{ 
            api::*,
            client::*
        },
        infrastructure::{ logger::{builder::LoggerBuilder, LogLevel } },
        info_log,
        error_log
    };

    fn setup() {
        LoggerBuilder::default()
            .with_level(LogLevel::Debug)
            .init();
    }

    #[tokio::test]
    async fn test_send_text_message() {
        setup();

        let params = TextMessage {
            text: "Test message".to_string(),
            reply_markup: None,
        };

        let response = TelegramClient::send_message(params).await;
        match response {
            Ok(response) => {
                info_log!(format!("Sending text message success: {:?}", response));
            }
            Err(error) => {
                error_log!(format!("Sending text message failed: {:?}", error));
            }
        }
    }

    #[tokio::test]
    async fn test_photo_message() {
        setup();
        
        let photo_msg = PhotoMessage {
            photo: PhotoInput::Url("https://cdn.pixabay.com/photo/2023/12/07/11/11/girl-8435340_1280.png".to_string()),
            caption: Some("图片的描述文案".to_string())
        };

        let response = TelegramClient::send_photo(photo_msg).await;
        match response {
            Ok(response) => {
                info_log!(format!("Send photo message success: {:?}", response))
            }
            Err(error) => {
                error_log!(format!("Send photo message failed: {:?}", error));
            }
        }
    }
}