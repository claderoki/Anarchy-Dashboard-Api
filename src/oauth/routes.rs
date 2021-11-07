use std::collections::HashMap;
use std::env;

use actix_web::HttpResponse;
use actix_web::http::HeaderValue;
use actix_web::HttpRequest;
use actix_web::Responder;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::Display;

#[derive(Display, Serialize, Deserialize, Debug)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OauthScope {
    Identify,
}

#[derive(Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
enum ResponseType {
    Code,
}

struct OauthUrlSettings {
    pub scopes: Vec<OauthScope>,
    pub client_id: u64,
    pub redirect_uri: String,
    pub response_type: ResponseType,
}

#[derive(Display, Serialize, Deserialize, Debug)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
enum GrantType {
    AuthorizationCode(String),
    RefreshToken(String),
}

struct OauthController;
impl OauthController {
    pub fn create_url(settings: OauthUrlSettings) -> String {
        format!("https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type={}&scope={}",
            settings.client_id,
            settings.redirect_uri,
            settings.response_type,
            settings.scopes.iter().map(|s|format!("{}", s)).collect::<Vec<String>>().join(",")
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: OauthScope,
}

async fn token_call(grant_type: GrantType) -> Result<AccessTokenResponse, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("client_id", env::var("DISCORD_CLIENT_ID").unwrap());
    params.insert("client_secret", env::var("DISCORD_CLIENT_SECRET").unwrap());
    params.insert(
        "redirect_uri",
        format!("{}/authenticate", env::var("CLIENT_URI").unwrap()),
    );
    params.insert("grant_type", grant_type.to_string());

    match grant_type {
        GrantType::AuthorizationCode(code) => {
            params.insert("code", code);
        }
        GrantType::RefreshToken(refresh_token) => {
            params.insert("refresh_token", refresh_token);
        }
    };

    let response = reqwest::Client::new()
        .post("https://discord.com/api/oauth2/token")
        .form(&params)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .send()
        .await
        .map_err(|e| format!("{}", e))?;

    match response.error_for_status_ref() {
        Ok(_) => {
            let json = response
                .json::<AccessTokenResponse>()
                .await
                .map_err(|e| format!("{}", e))?;
            return Ok(json);
        }
        Err(_) => {
            return Err(response.text().await.map_err(|e| format!("{}", e))?);
        }
    }
}

pub async fn authenticate(req: HttpRequest) -> HttpResponse {
    match req.match_info().get("code") {
        Some(code) => {
            match token_call(GrantType::AuthorizationCode(code.into())).await {
                Ok(response) => HttpResponse::Ok().json(response),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

pub async fn oauth_url() -> impl Responder {
    OauthController::create_url(OauthUrlSettings {
        scopes: vec![OauthScope::Identify],
        client_id: 906651425961046046,
        redirect_uri: format!("{}/authenticate", env::var("CLIENT_URI").unwrap()),
        response_type: ResponseType::Code,
    })
}
