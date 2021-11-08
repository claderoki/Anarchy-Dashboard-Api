use super::base_api::Callable;
use actix_web::http::HeaderValue;
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::header::AUTHORIZATION;
use strum_macros::Display;

pub struct DiscordCall {
    pub access_token: AccessToken,
}

#[derive(Display, Debug)]
pub enum AccessTokenKind {
    Bot,
    Bearer,
}

pub struct AccessToken {
    pub kind: AccessTokenKind,
    pub value: String,
}

impl AccessToken {
    pub fn bearer(access_token: &str) -> Self {
        Self {
            value: access_token.into(),
            kind: AccessTokenKind::Bearer,
        }
    }

    pub fn bot(access_token: &str) -> Self {
        Self {
            value: access_token.into(),
            kind: AccessTokenKind::Bot,
        }
    }
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
        if let Ok(value) = HeaderValue::from_str(&format!(
            "{} {}",
            self.access_token.kind.to_string(),
            self.access_token.value
        )) {
            params.insert(AUTHORIZATION, value);
        }
        Some(params)
    }
}
