use std::env;

use super::base_api::Callable;
use super::calls::GetGuilds;
use super::calls::GetChannels;
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Guild {
    id: String,
    name: String,
}

pub async fn get_mutual_guilds(req: HttpRequest) -> HttpResponse {
    let access_token = parse_access_token(&req);

    if access_token.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let user_call = DiscordCall::new(AccessToken::bearer(&access_token.unwrap()));
    let bot_call = DiscordCall::new(AccessToken::bot(&env::var("DISCORD_CLIENT_TOKEN").unwrap()));

    // println!("{:?}", bot_call.call(GetChannels{guild_id: 906652198899965983}).await);

    let mutual_guilds_response: Result<Vec<Guild>, String> = {
        match user_call.call(GetGuilds {}).await {
            Ok(user_guilds) => {
                match bot_call.call(GetGuilds {}).await {
                    Ok(bot_guilds) => {
                        let mut guilds: Vec<Guild> = Vec::new();
                        for guild in user_guilds.guilds.iter() {
                            for other_guild in bot_guilds.guilds.iter() {
                                if guild.id == other_guild.id {
                                    guilds.push(Guild{id: guild.id.clone(), name: guild.name.clone()});
                                }
                            }
                        }
                        Ok(guilds)
                    },
                    Err(e) => {Err(e)},
                }
            },
            Err(e) => {Err(e)},
        }
    };
    match mutual_guilds_response {
        Ok(guilds) => {
            HttpResponse::Ok().json(guilds)
        },
        Err(err) => {
            HttpResponse::BadRequest().finish()
        },
    }
}
