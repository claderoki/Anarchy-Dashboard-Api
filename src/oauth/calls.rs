use std::collections::HashMap;
use std::env;

use actix_web::http::HeaderValue;
use reqwest::header::CONTENT_TYPE;

use super::models::AccessTokenResponse;
use super::models::GrantType;

pub async fn token_call(grant_type: GrantType) -> Result<AccessTokenResponse, String> {
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
