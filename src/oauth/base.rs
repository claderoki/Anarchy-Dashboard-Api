use std::collections::HashMap;

use serde::de::DeserializeOwned;
use reqwest;

pub enum HttpMethod {
    Get,
}

pub trait Endpoint<D: DeserializeOwned> {
    fn get_method(&self) -> HttpMethod;
}

trait Callable {
    const BASE_URI: &'static str;

    fn call<T: Endpoint<D>, D: DeserializeOwned>(endpoint: T) {
        // let a = Self::BASE_URI;
        match endpoint.get_method() {
            HttpMethod::Get => todo!(),
        };
    }

    fn get_default_headers(&self) -> Option<HashMap<String, String>> {
        None
    }

}


#[derive(serde::Deserialize, Debug)]
pub struct MeResponse {

}

struct GetMe;
impl Endpoint<MeResponse> for GetMe {
    fn get_method(&self) -> HttpMethod {
        HttpMethod::Get
    }
}

struct DiscordCall;
impl Callable for DiscordCall {
    const BASE_URI: &'static str = "https://discord.com/api";

    fn get_default_headers(&self) -> Option<HashMap<String, String>> {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("Authentication".into(), format!("Bearer {}", ""));
        Some(params)
    }
}