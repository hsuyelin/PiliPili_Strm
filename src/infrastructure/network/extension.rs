use std::{
    collections::HashMap,
    future::Future,
    pin::Pin
};

use reqwest::{multipart, RequestBuilder};

pub trait RequestFormExt {

    fn with_multipart<'a>(
        self,
        fields: HashMap<String, String>,
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
}