use std::collections::HashMap;

use crate::infrastructure::network::{HttpMethod, NetworkTask, NetworkTarget};
use crate::infrastructure::config::Config;

pub enum EmbyAPI {
    GetUser { user_id: String },
}

impl NetworkTarget for EmbyAPI {

    fn base_url(&self) -> String {
        Config::get().emby.base_url.clone()
    }

    fn path(&self) -> String {
        match self {
            EmbyAPI::GetUser { user_id, .. } => {
                format!("emby/Users/{}", user_id)
            }
        }
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn task(&self) -> NetworkTask {
        match self {
            EmbyAPI::GetUser { user_id: _ } => {
                let api_key = Config::get().emby.api_key.clone();
                let mut params = HashMap::new();
                params.insert("api_key".to_string(), api_key);
                NetworkTask::RequestParameters(params)
            }
        }
    }

    fn headers(&self) -> Option<Vec<(&'static str, String)>> {
        let base_url = Config::get().emby.base_url.clone();
        Some(vec![
            ("accept", "application/json".to_string()),
            ("origin", base_url.clone()),
            ("referer", format!("{}/", base_url)),
            ("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36".to_string()),
        ])
    }
}
