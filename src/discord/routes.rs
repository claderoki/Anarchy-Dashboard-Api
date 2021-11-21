use std::env;

use crate::helpers::caching::base::Cache;
use crate::helpers::caching::discord::GuildId;
use crate::helpers::caching::discord::GuildNameCache;

use crate::helpers::repositories::discord::ChannelRepository;
use crate::helpers::repositories::discord::ChannelRepositoryOptions;
use crate::helpers::repositories::discord::MemberRepository;
use crate::helpers::repositories::discord::Repository;
use crate::helpers::repositories::discord::RoleRepository;

use crate::helpers::repositories::discord::SharedRepositoryOptions;
use crate::helpers::validator::get_allowed_guilds;
use crate::helpers::validator::parse_access_token;
use crate::helpers::validator::Validator;

use super::base_api::Callable;
use super::calls::ChannelKind;
use super::calls::GetGuilds;
use super::discord_base::AccessToken;
use super::discord_base::DiscordCall;
use actix_web::get;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
}

pub async fn get_shared_guilds(access_token: &str) -> Result<Vec<Guild>, String> {
    let user_call = DiscordCall::new(AccessToken::Bearer(access_token.into()));
    let bot_call = DiscordCall::new(AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()));
    let user_guilds = user_call.call(GetGuilds).await?;
    let bot_guilds = bot_call.call(GetGuilds).await?;

    let mut guilds: Vec<Guild> = Vec::new();
    for guild in user_guilds.guilds.iter() {
        for other_guild in bot_guilds.guilds.iter() {
            if guild.id == other_guild.id {
                guilds.push(Guild {
                    id: guild.id.clone(),
                    name: guild.name.clone(),
                });
            }
        }
    }
    Ok(guilds)
}

#[get("/get_mutual_guilds")]
pub async fn get_mutual_guilds(req: HttpRequest) -> HttpResponse {
    let access_token = parse_access_token(&req);

    if access_token.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    match get_allowed_guilds(&access_token.as_ref().unwrap()).await {
        Ok(guild_ids) => {
            let mut guilds: Vec<Guild> = Vec::new();

            for guild_id in guild_ids.iter() {
                if let Some(guild_name) = GuildNameCache::get(GuildId(*guild_id)) {
                    guilds.push(Guild {
                        id: guild_id.to_string(),
                        name: guild_name,
                    });
                }
            }

            HttpResponse::Ok().json(guilds)
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/{guild_id}/get_all_text_channels")]
pub async fn get_all_text_channels(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(validation) => {
            match ChannelRepository::get(&ChannelRepositoryOptions(
                validation.guild_id,
                ChannelKind::GuildText,
            ))
            .await
            {
                Ok(channels) => HttpResponse::Ok().json(channels),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/{guild_id}/get_all_roles")]
pub async fn get_all_roles(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(validation) => {
            match RoleRepository::get(&SharedRepositoryOptions(validation.guild_id)).await {
                Ok(roles) => HttpResponse::Ok().json(roles),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[get("/{guild_id}/get_all_members")]
pub async fn get_all_members(req: HttpRequest) -> HttpResponse {
    match Validator::new().validate(&req).await {
        Ok(validation) => {
            match MemberRepository::get(&SharedRepositoryOptions(validation.guild_id)).await {
                Ok(members) => HttpResponse::Ok().json(members),
                Err(err) => HttpResponse::BadRequest().body(err),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
