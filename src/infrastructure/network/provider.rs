//! Provides the main network request handling functionality.
//! 
//! This module implements the core network provider that handles HTTP requests,
//! including request building, sending, and plugin integration.

use reqwest::{
    Client, 
    Method
};
use once_cell::sync::Lazy;

use super::{
    http_method::HttpMethod,
    plugin::NetworkPlugin,
    task::NetworkTask,
    target::NetworkTarget,
    extension::RequestFormExt
};

/// A static HTTP client instance configured with default settings.
/// 
/// The client is configured to:
/// - Use rustls for TLS
/// - Accept invalid certificates (for development)
/// - Accept invalid hostnames (for development)
/// - Use a standard browser user agent
static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .use_rustls_tls()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36")
        .build()
        .expect("Failed to build HTTP client")
});

/// The main network request provider.
/// 
/// This struct handles the execution of network requests with plugin support.
/// It manages:
/// - Request building and sending
/// - Plugin integration
/// - Response handling
pub struct NetworkProvider {

    /// List of plugins to be executed during request lifecycle
    plugins: Vec<Box<dyn NetworkPlugin>>,
}

impl NetworkProvider {

    /// Creates a new provider with the specified plugins.
    /// 
    /// # Arguments
    /// 
    /// * `plugins` - Vector of plugins to be used for request processing
    pub fn new(plugins: Vec<Box<dyn NetworkPlugin>>) -> Self {
        Self { plugins }
    }

    /// Sends a network request to the specified target.
    /// 
    /// This method handles the complete request lifecycle:
    /// 1. Builds the request with the target's configuration
    /// 2. Executes request plugins
    /// 3. Sends the request
    /// 4. Executes response/error plugins
    /// 
    /// # Arguments
    /// 
    /// * `target` - The target to send the request to
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either the response or an error
    pub async fn send_request<T: NetworkTarget>(
        &self, 
        target: &T
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "{}/{}",
            target.base_url().trim_end_matches('/'),
            target.path().trim_start_matches('/')
        );

        let mut request = CLIENT.request(match target.method() {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Delete => Method::DELETE,
        }, &url);

        if let Some(headers) = target.headers() {
            let mut header_map = reqwest::header::HeaderMap::new();
            for (key, value) in headers {
                header_map.insert(key, value.parse().unwrap());
            }
            request = request.headers(header_map);
        }

        match target.task() {
            NetworkTask::RequestPlain => {
                // For simple requests with just URL/path, no additional configuration is needed
                // The request is already configured with the URL and method
            }
            NetworkTask::RequestJson(json_body) => {
                request = request.json(&json_body);
            }
            NetworkTask::RequestParameters(params) => {
                request = request.query(&params);
            }
            NetworkTask::RequestForm(params) => {
                request = request.with_multipart(params).await;
            }
        }

        for plugin in &self.plugins {
            if let Some(cloned_request) = request.try_clone() {
                if let Ok(built_request) = cloned_request.build() {
                    plugin.on_request(&built_request);
                }
            }
        }

        let response = request.send().await;
        match &response {
            Ok(res) => {
                for plugin in &self.plugins {
                    plugin.on_response(res);
                }
            }
            Err(err) => {
                for plugin in &self.plugins {
                    plugin.on_error(err);
                }
            }
        }

        response
    }
}