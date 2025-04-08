use std::{
    collections::HashMap,
    future::Future,
    pin::Pin
};

use reqwest::{multipart, RequestBuilder};

use crate::error_log;

/// Domain identifier for reqwest extension logs
const REQWEST_EXT_LOGGER_DOMAIN: &str = "[REQWEST-EXT]";

pub trait RequestFormExt {

    fn with_multipart<'a>(
        self,
        fields: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = RequestBuilder> + Send + 'a>>
    where
        Self: 'a;

    fn with_multipart_files<'a>(
        self,
        fields: HashMap<String, String>,
        files: Vec<(String, String)>,
    ) -> Pin<Box<dyn Future<Output = RequestBuilder> + Send + 'a>>
    where
        Self: 'a;
}

impl RequestFormExt for RequestBuilder {

    fn with_multipart<'a>(
        self,
        fields: HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = RequestBuilder> + Send + 'a>>
    where
        Self: 'a,
    {
        Box::pin(async move {
            let mut form = multipart::Form::new();

            for (key, value) in fields {
                form = form.text(key, value);
            }

            self.multipart(form)
        })
    }

    fn with_multipart_files<'a>(
        self,
        fields: HashMap<String, String>,
        files: Vec<(String, String)>,
    ) -> Pin<Box<dyn Future<Output = RequestBuilder> + Send + 'a>>
    where
        Self: 'a,
    {
        Box::pin(async move {
            let mut form = multipart::Form::new();

            for (key, value) in fields {
                form = form.text(key, value);
            }

            for (path, name) in files {
                match multipart::Part::file(&path).await {
                    Ok(file_part) => {
                        form = form.part(name, file_part);
                    }
                    Err(e) => {
                        error_log!(
                            REQWEST_EXT_LOGGER_DOMAIN,
                            format!("Failed to get part from file {}: {}", path, e)
                        );
                    }
                }
            }

            self.multipart(form)
        })
    }
}