extern crate reqwest;

use std::collections::HashMap;

use super::base_api::ApiCall;
use super::base_api::Api;

pub struct GetMe {
}

impl GetMe {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct MeResponse {

}

impl ApiCall<MeResponse> for GetMe {
    fn get_uri(&self) -> String {
        format!("{}/users/@me", DiscordApi::get_base_uri())
    }

}

pub struct DiscordApi;
impl Api for DiscordApi {
    fn get_base_uri() -> String {
        "https://discord.com/api".into()
    }
}
