use std::env;

use super::base_api::Callable;
use super::calls::GetGuilds;
use super::discord_base::AccessToken;
use super::discord_base::DiscordCall;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

pub fn parse_access_token(req: &HttpRequest) -> Option<String> {
    match req.headers().get("Authorization") {
        Some(value) => Some(
            value
                .to_str()
                .unwrap_or("")
                .split(" ")
                .last()
                .unwrap_or("")
                .into(),
        ),
        None => None,
    }
}

pub async fn get_mutual_guilds(req: HttpRequest) -> HttpResponse {
    let access_token = parse_access_token(&req);

    let user_call = DiscordCall::new(AccessToken::bearer(&access_token.unwrap()));
    let bot_call = DiscordCall::new(AccessToken::bot(&env::var("DISCORD_CLIENT_TOKEN").unwrap()));

    let user_response = user_call.call(GetGuilds {}).await;
    let bot_response = bot_call.call(GetGuilds {}).await;

    println!("{:?}", user_response);
    println!("{:?}", bot_response);

    HttpResponse::BadRequest().finish()
}
