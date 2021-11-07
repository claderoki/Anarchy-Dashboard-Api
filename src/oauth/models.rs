use serde::Deserialize;
use serde::Serialize;
use strum_macros::Display;

#[derive(Display, Serialize, Deserialize, Debug)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OauthScope {
    Identify,
    Guilds,
}

#[derive(Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
}

pub struct OauthUrlSettings {
    pub scopes: Vec<OauthScope>,
    pub client_id: u64,
    pub redirect_uri: String,
    pub response_type: ResponseType,
}

#[derive(Display, Serialize, Deserialize, Debug)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode(String),
    RefreshToken(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
    // #[serde(with = "serde_with::rust::StringWithSeparator::<SpaceSeparator>")]
    // pub scope: Vec<OauthScope>,
}
