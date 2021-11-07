use async_trait::async_trait;
use reqwest;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

pub enum HttpMethod {
    Get,
}

pub trait Endpoint<D: DeserializeOwned> {
    const METHOD: HttpMethod = HttpMethod::Get;

    fn get_endpoint(&self) -> &str;
}

#[async_trait]
pub trait Callable {
    const BASE_URI: &'static str;

    async fn call<T: Endpoint<D> + Send, D: DeserializeOwned>(
        &self,
        endpoint: T,
    ) -> Result<D, String> {
        let uri = format!("{}/{}", Self::BASE_URI, endpoint.get_endpoint());
        let client = reqwest::Client::new();
        let response = match T::METHOD {
            HttpMethod::Get => client.get(&uri),
        }
        .headers(self.get_default_headers().unwrap_or(HeaderMap::new()))
        .send()
        .await
        .map_err(|e| format!("{}", e))?;

        let json = response.json::<D>().await.map_err(|e| format!("{}", e))?;
        Ok(json)
    }

    fn get_default_headers(&self) -> Option<HeaderMap> {
        None
    }
}
