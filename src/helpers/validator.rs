use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use actix_web::HttpRequest;

use crate::discord::base_api::Callable;
use crate::discord::calls::GetMe;
use crate::discord::discord_base::AccessToken;
use crate::discord::discord_base::DiscordCall;
use crate::discord::routes::get_shared_guilds;
use crate::discord::routes::parse_access_token;
use crate::helpers::caching::base::Cache;
use crate::helpers::caching::discord::AccessTokenHash;
use crate::helpers::caching::discord::GuildsCache;
use crate::helpers::caching::discord::UserId;
use crate::helpers::caching::discord::UserIdCache;

use super::caching::discord::GuildId;
use super::caching::discord::GuildNameCache;

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

pub async fn get_allowed_guilds(access_token: &str) -> Result<Vec<u64>, String> {
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

            for guild in guilds.iter() {
                GuildNameCache::set(
                    GuildId(guild.id.parse::<u64>().unwrap()),
                    guild.name.clone(),
                );
            }

            Ok(guilds
                .iter()
                .map(|i| i.id.parse::<u64>().unwrap())
                .collect::<Vec<u64>>())
        }
    }
}

type ValidationResult = Result<ValidationInfo, String>;
pub struct ValidationInfo {
    pub guild_id: u64,
    pub access_token: String,
}

pub struct Validator;
impl Validator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn validate(&self, req: &HttpRequest) -> ValidationResult {
        let access_token = parse_access_token(&req).ok_or("No access token found.")?;
        let guild_id = req
            .match_info()
            .get("guild_id")
            .ok_or("No guild id passed.")?
            .parse::<u64>()
            .map_err(|_| "Guild id invalid.")?;

        match get_allowed_guilds(&access_token).await {
            Ok(guilds) => {
                if guilds.contains(&guild_id) {
                    return Ok(ValidationInfo {
                        guild_id,
                        access_token,
                    });
                }
            }
            Err(err) => println!("{}", err),
        }

        return Err("Guild not permitted.".into());
    }
}
