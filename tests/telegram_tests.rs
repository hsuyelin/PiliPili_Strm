#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use tokio;

    use pilipili_strm::{
        core::{ 
            api::*,
            client::*
        },
        infrastructure::{ 
            logger::{builder::LoggerBuilder, LogLevel},
            network::{curl_plugin::CurlPlugin}
        },
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

        let client = TelegramClient::builder()
            .with_plugin(CurlPlugin)
            .build();
        let text_msg = TextMessage {
            text: "Test message".to_string(),
            reply_markup: None,
        };
        let response = client.send_message(text_msg).await;
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
    async fn test_photo_message_with_url() {
        setup();

        let client = TelegramClient::builder()
            .with_plugin(CurlPlugin)
            .build();
        let photo_msg = PhotoMessage {
            photo: PhotoInput::Url("https://cdn.pixabay.com/photo/2023/12/07/11/11/girl-8435340_1280.png".to_string()),
            caption: Some("description of photo".to_string())
        };
        let response = client.send_photo(photo_msg).await;
        match response {
            Ok(response) => {
                info_log!(format!("Send photo message success: {:?}", response))
            }
            Err(error) => {
                error_log!(format!("Send photo message failed: {:?}", error));
            }
        }
    }

    #[tokio::test]
    async fn test_photo_message_with_file() {
        setup();

        let client = TelegramClient::builder()
            .with_plugin(CurlPlugin)
            .build();
        let photo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/telegram_photo.png");
        let photo_msg = PhotoMessage {
            photo: PhotoInput::FilePath(photo_path),
            caption: Some("description of photo".to_string())
        };
        let response = client.send_photo(photo_msg).await;
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