use std::env;

use actix_web::HttpResponse;
use actix_web::HttpRequest;
use actix_web::Responder;

use crate::oauth::models::OauthScope;
use crate::oauth::models::ResponseType;

use super::calls::token_call;
use super::models::GrantType;
use super::models::OauthUrlSettings;

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
        client_id: env::var("DISCORD_CLIENT_ID").unwrap().parse::<u64>().unwrap(),
        redirect_uri: format!("{}/authenticate", env::var("CLIENT_URI").unwrap()),
        response_type: ResponseType::Code,
    })
}
