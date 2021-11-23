use super::base_api::Callable;
use actix_web::http::HeaderValue;
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::header::AUTHORIZATION;

#[derive(Clone)]
pub enum AccessToken {
    Bot(String),
    Bearer(String),
}

pub struct DiscordCall {
    pub access_token: AccessToken,
}

impl DiscordCall {
    pub fn new(access_token: AccessToken) -> Self {
        Self { access_token }
    }
}

impl Callable for DiscordCall {
    const BASE_URI: &'static str = "https://discord.com/api";

    fn get_default_headers(&self) -> Option<HeaderMap> {
        let mut params = HeaderMap::new();
        if let Ok(value) = HeaderValue::from_str(&match &self.access_token {
            AccessToken::Bot(value) => format!("Bot {}", value),
            AccessToken::Bearer(value) => format!("Bearer {}", value),
        }) {
            params.insert(AUTHORIZATION, value);
        }
        Some(params)
    }
}
