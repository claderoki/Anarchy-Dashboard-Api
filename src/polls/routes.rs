use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use crate::discord::base_api::Callable;
use crate::discord::caching::AccessTokenHash;
use crate::discord::caching::Cache;
use crate::discord::caching::GuildsCache;
use crate::discord::caching::UserId;
use crate::discord::caching::UserIdCache;
use crate::discord::calls::GetMe;
use crate::discord::discord_base::AccessToken;
use crate::discord::discord_base::DiscordCall;
use crate::discord::routes::get_shared_guilds;
use crate::discord::routes::parse_access_token;

use super::models::Poll;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

async fn get_user_id(access_token: &str) -> Result<u64, String> {
    let mut hasher = DefaultHasher::new();
    access_token.hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let key = AccessTokenHash::new(&hash);

    match UserIdCache::get(key) {
        Some(user_id) => Ok(user_id),
        None => {
            let call = DiscordCall {
                access_token: AccessToken::Bearer(access_token.into()),
            };
            let result = call.call(GetMe {}).await;
            if let Ok(me) = result {
                if let Ok(user_id) = me.id.parse::<u64>() {
                    UserIdCache::set(AccessTokenHash::new(&hash), user_id);
                    return Ok(user_id);
                }
            }

            Err("Couldn't find user id".into())
        }
    }
}

async fn get_allowed_guilds(access_token: &str) -> Result<Vec<u64>, String> {
    let user_id = get_user_id(access_token).await?;

    let key = UserId { 0: user_id };
    match GuildsCache::get(key) {
        Some(guild_ids) => Ok(guild_ids),
        None => {
            let guilds = get_shared_guilds(access_token).await?;

            GuildsCache::set(
                UserId { 0: user_id },
                guilds
                    .iter()
                    .map(|i| i.id.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            );

            Ok(guilds
                .iter()
                .map(|i| i.id.parse::<u64>().unwrap())
                .collect::<Vec<u64>>())
        }
    }
}

async fn get_allowed_channels(_guild_id: u64) -> Vec<Channel> {
    vec![
        Channel {
            id: 906898585302499369,
            name: "polls".into(),
        },
        Channel {
            id: 906898600162906162,
            name: "data".into(),
        },
    ]
}

async fn _get_allowed_roles(_guild_id: u64) -> Vec<Role> {
    vec![
        Role {
            id: 907341985365512323,
            name: "experimentation".into(),
        },
        Role {
            id: 907342012364226600,
            name: "Earthling".into(),
        },
    ]
}

pub async fn save_poll(poll: web::Json<Poll>) -> impl Responder {
    println!("{:?}", poll);
    format!("OK")
}

type ValidationResult = Result<ValidationInfo, String>;
pub struct ValidationInfo {
    pub guild_id: u64,
}

struct Validator;
impl Validator {
    pub async fn validate(&self, req: &HttpRequest) -> ValidationResult {
        let access_token = parse_access_token(&req).ok_or("No access token found.")?;
        let guild_id = req
            .match_info()
            .get("guild_id")
            .ok_or("No guild id passed.")?
            .parse::<u64>()
            .map_err(|_| "Guild id invalid.")?;

        if let Ok(guilds) = get_allowed_guilds(&access_token).await {
            if guilds.contains(&guild_id) {
                return Ok(ValidationInfo { guild_id: guild_id });
            }
        }

        return Err("Guild not permitted.".into());
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Channel {
    pub id: u64,
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Role {
    pub id: u64,
    pub name: String,
}

async fn get_poll_channels_result(req: &HttpRequest) -> Result<Vec<Channel>, String> {
    let validator = Validator {};
    match validator.validate(&req).await {
        Ok(info) => Ok(get_allowed_channels(info.guild_id).await),
        Err(_) => Err("Validate failed.".into()),
    }
}

pub async fn get_poll_channels(req: HttpRequest) -> HttpResponse {
    match get_poll_channels_result(&req).await {
        Ok(allowed_channels) => {
            return HttpResponse::Ok().json(allowed_channels);
        }
        Err(_err) => {
            println!("{}", _err);
            return HttpResponse::Unauthorized().finish();
        }
    }
}
