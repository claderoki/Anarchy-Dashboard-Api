use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use actix_web::HttpRequest;

use crate::discord::base_api::Callable;
use crate::discord::calls::GetMe;
use crate::discord::discord_base::AccessToken;
use crate::discord::discord_base::DiscordCall;
use crate::discord::models::Guild;
use crate::helpers::caching::base::Cache;
use crate::helpers::caching::discord::AccessTokenHash;
use crate::helpers::caching::discord::UserIdCache;

use super::repositories::discord::GuildRepositoryOptions;
use super::repositories::discord::MutualGuildRepository;
use super::repositories::discord::Repository;

pub fn parse_access_token(req: &HttpRequest) -> Option<String> {
    let value = req.headers().get("Authorization")?;
    Some(value.to_str().ok()?.split(" ").last()?.into())
}

async fn get_user_id(access_token: &str) -> Result<u64, String> {
    let mut hasher = DefaultHasher::new();
    access_token.hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let key = AccessTokenHash::new(&hash);

    match UserIdCache::get(key) {
        Some(user_id) => Ok(user_id),
        None => {
            let call = DiscordCall::new(AccessToken::Bearer(access_token.into()));
            let result = call.call(GetMe).await;
            if let Ok(me) = result {
                if let Ok(user_id) = me.id.parse::<u64>() {
                    UserIdCache::set(AccessTokenHash::new(&hash), &user_id);
                    return Ok(user_id);
                }
            }

            Err("Couldn't find user id".into())
        }
    }
}

pub async fn get_allowed_guilds(access_token: &str) -> Result<Vec<Guild>, String> {
    let user_id = get_user_id(access_token).await?;

    let options = GuildRepositoryOptions(user_id, AccessToken::Bearer(access_token.into()));
    let guilds = MutualGuildRepository::get(&options).await?;

    Ok(guilds)
}

type ValidationResult = Result<ValidationInfo, String>;
pub struct ValidationInfo {
    pub guild_id: u64,
    pub access_token: String,
}

pub struct Validator;
impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub async fn validate(&self, req: &HttpRequest) -> ValidationResult {
        let access_token = parse_access_token(&req).ok_or("No access token found.")?;
        let guild_id = req
            .match_info()
            .get("guild_id")
            .ok_or("No guild id passed.")?;

        match get_allowed_guilds(&access_token).await {
            Ok(guilds) => {
                if guilds.iter().any(|g| g.id == guild_id) {
                    return Ok(ValidationInfo {
                        guild_id: guild_id.parse::<u64>().unwrap(),
                        access_token,
                    });
                }
            }
            Err(err) => println!("{}", err),
        }

        return Err("Guild not permitted.".into());
    }
}
