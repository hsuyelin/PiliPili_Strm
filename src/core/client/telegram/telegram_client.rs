use crate::infrastructure::network::{NetworkProvider, NetworkPlugin};
use crate::core::api::telegram::{
    TextMessage, PhotoMessage, TelegramAPI, TelegramResponse, MessageResult
};

/// Telegram API client with configured network provider.
///
/// Maintains a reusable network provider instance to make authenticated requests
/// to Telegram's servers. Construct using [`TelegramClientBuilder`] for customization.
pub struct TelegramClient {

    /// The network provider handling actual HTTP requests
    provider: NetworkProvider,
}

/// Builder for creating configured `TelegramClient` instances.
///
/// Allows customization of the network stack through plugins before constructing
/// the final client. By default creates a client with no plugins.
pub struct TelegramClientBuilder {
    plugins: Vec<Box<dyn NetworkPlugin>>,
}

impl TelegramClientBuilder {

    /// Creates a new builder with default configuration.
    ///
    /// Starts with an empty set of network plugins. You'll typically want to add
    /// at least one network implementation like `CurlPlugin`.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Adds a network plugin to the client's configuration.
    ///
    /// # Arguments
    /// * `plugin` - Network plugin implementing the transport layer
    ///
    /// # Note
    /// Plugins are used in the order they're added. The first compatible plugin
    /// will handle each request.
    pub fn with_plugin(mut self, plugin: impl NetworkPlugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Constructs the `TelegramClient` with the configured plugins.
    ///
    /// Consumes the builder and returns the finalized client instance.
    pub fn build(self) -> TelegramClient {
        let provider = NetworkProvider::new(self.plugins);
        TelegramClient { provider }
    }
}

impl TelegramClient {

    /// Creates a new `TelegramClientBuilder` for configuring a client instance.
    ///
    /// This is the preferred way to construct a `TelegramClient` as it allows
    /// for network backend customization.
    pub fn builder() -> TelegramClientBuilder {
        TelegramClientBuilder::new()
    }

    /// Sends a text message to a Telegram chat.
    ///
    /// # Arguments
    /// * `params` - Message configuration including chat ID and text content
    ///
    /// # Errors
    /// Returns `Err` if:
    /// - Network request fails
    /// - Telegram API returns error
    /// - Response parsing fails
    pub async fn send_message(
        &self,
        params: TextMessage,
    ) -> Result<TelegramResponse<MessageResult>, anyhow::Error> {
        let response = self.provider
            .send_request(&TelegramAPI::SendMessage(params))
            .await?;
        let result: TelegramResponse<MessageResult> = response.json().await?;
        Ok(result)
    }

    /// Sends a photo to a Telegram chat.
    ///
    /// # Arguments
    /// * `params` - Photo message configuration including chat ID and image data
    ///
    /// # Errors
    /// Returns `Err` if:
    /// - Network request fails
    /// - File upload fails
    /// - Telegram API returns error
    /// - Response parsing fails
    pub async fn send_photo(
        &self,
        params: PhotoMessage,
    ) -> Result<TelegramResponse<MessageResult>, anyhow::Error> {
        let response = self.provider
            .send_request(&TelegramAPI::SendPhoto(params))
            .await?;
        let result: TelegramResponse<MessageResult> = response.json().await?;
        Ok(result)
    }
}