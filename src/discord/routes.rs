use std::env;

use actix_web::HttpResponse;
use actix_web::HttpRequest;
use actix_web::Responder;
use super::calls::GetGuilds;
use super::discord_base::DiscordCall;

pub async fn get_mutual_guilds(req: HttpRequest) -> HttpResponse {
    let userCall = DiscordCall::new(AccessToken::bearer(access_token));
    let botCall = DiscordCall::new(AccessToken::bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()));
    let endpoint = GetGuilds::new();

    let userResponse = userCall.call(endpoint).await;
    let botResponse  = botCall.call(endpoint).await;

    println!("{:?}", userResponse);
    println!("{:?}", botResponse);

    HttpResponse::BadRequest().finish()
}
