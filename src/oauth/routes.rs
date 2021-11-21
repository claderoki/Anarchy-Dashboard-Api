use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::Hash;
use std::hash::Hasher;

use actix_web::get;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::discord::base_api::Callable;
use crate::discord::calls::GetMe;
use crate::discord::discord_base::AccessToken;
use crate::discord::discord_base::DiscordCall;
use crate::helpers::caching::base::Cache;
use crate::helpers::caching::discord::AccessTokenHash;
use crate::helpers::caching::discord::UserIdCache;
use crate::oauth::models::OauthScope;
use crate::oauth::models::ResponseType;

use super::calls::token_call;
use super::models::AccessTokenResponse;
use super::models::GrantType;
use super::models::OauthUrlSettings;

struct OauthController;
impl OauthController {
    pub fn create_url(settings: OauthUrlSettings) -> String {
        format!("https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type={}&scope={}",
            settings.client_id,
            settings.redirect_uri,
            settings.response_type,
            settings.scopes.iter().map(|s|format!("{}", s)).collect::<Vec<String>>().join("%20")
        )
    }
}

async fn store_oauth(response: &AccessTokenResponse) {
    let mut hasher = DefaultHasher::new();
    response.access_token.hash(&mut hasher);
    let key = AccessTokenHash::new_with_expires_in(
        &hasher.finish().to_string(),
        response.expires_in.try_into().unwrap(),
    );

    let call = DiscordCall::new(AccessToken::Bearer(response.access_token.clone()));
    let result = call.call(GetMe).await;

    if let Ok(me) = result {
        if let Ok(user_id) = me.id.parse::<u64>() {
            UserIdCache::set(key, &user_id);
        }
    }
}

#[get("/authenticate")]
pub async fn authenticate(req: HttpRequest) -> HttpResponse {
    match req.match_info().get("code") {
        Some(code) => match token_call(GrantType::AuthorizationCode(code.into())).await {
            Ok(response) => {
                store_oauth(&response).await;
                HttpResponse::Ok().json(response)
            }
            Err(err) => HttpResponse::BadRequest().body(err),
        },
        None => HttpResponse::BadRequest().finish(),
    }
}

#[get("/url")]
pub async fn oauth_url() -> impl Responder {
    OauthController::create_url(OauthUrlSettings {
        scopes: vec![OauthScope::Identify, OauthScope::Guilds],
        client_id: env::var("DISCORD_CLIENT_ID")
            .unwrap()
            .parse::<u64>()
            .unwrap(),
        redirect_uri: format!("{}/authenticate", env::var("CLIENT_URI").unwrap()),
        response_type: ResponseType::Code,
    })
}
