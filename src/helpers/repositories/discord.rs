use std::env;

use crate::discord::calls::ChannelKind;
use crate::discord::calls::GetChannels;
use crate::discord::calls::GetGuilds;
use crate::discord::calls::GetMembers;
use crate::discord::calls::GetRoles;
use crate::discord::discord_base::AccessToken;
use crate::discord::discord_base::DiscordCall;

use crate::discord::base_api::Callable;
use crate::discord::models::Guild;
use crate::helpers::caching::base::Cache;

use crate::helpers::caching::base::CacheKey;
use crate::helpers::caching::discord::ChannelsCache;
use crate::helpers::caching::discord::GuildId;

use crate::discord::models::Channel;
use crate::discord::models::Member;
use crate::discord::models::Role;
use crate::helpers::caching::discord::GuildsCache;
use crate::helpers::caching::discord::MembersCache;
use crate::helpers::caching::discord::RolesCache;
use crate::helpers::caching::discord::UserId;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Repository<D: DeserializeOwned + Send + Sized, F: Send + Sync> {
    async fn get(f: &F) -> Result<Vec<D>, String> {
        match Self::get_cached(f).await {
            Ok(channels) => Ok(channels),
            Err(_) => {
                let uncached = Self::get_uncached(f).await?;
                let _ = Self::cache(f, &uncached);
                Ok(uncached)
            }
        }
    }

    fn cache(f: &F, values: &Vec<D>) -> bool;
    async fn get_cached(f: &F) -> Result<Vec<D>, String>;
    async fn get_uncached(f: &F) -> Result<Vec<D>, String>;
}

pub trait RepositoryOptions<D: CacheKey> {
    fn get_cache_key(&self) -> D;
}

pub struct ChannelRepositoryOptions(pub u64, pub ChannelKind);
impl RepositoryOptions<GuildId> for ChannelRepositoryOptions {
    fn get_cache_key(&self) -> GuildId {
        GuildId(self.0)
    }
}

pub struct ChannelRepository;
#[async_trait]
impl Repository<Channel, ChannelRepositoryOptions> for ChannelRepository {
    fn cache(options: &ChannelRepositoryOptions, values: &Vec<Channel>) -> bool {
        ChannelsCache::set(options.get_cache_key(), &values)
    }

    async fn get_cached(options: &ChannelRepositoryOptions) -> Result<Vec<Channel>, String> {
        let channels =
            ChannelsCache::get(options.get_cache_key()).ok_or(String::from("Not found."))?;
        Ok(channels
            .into_iter()
            .filter(|c| &c.kind == &options.1)
            .collect::<Vec<Channel>>())
    }

    async fn get_uncached(f: &ChannelRepositoryOptions) -> Result<Vec<Channel>, String> {
        let call = DiscordCall::new(AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()));
        let result = call.call(GetChannels { guild_id: f.0 }).await?;

        let channels: Vec<Channel> = result
            .channels
            .into_iter()
            .filter(|c| c.kind == f.1)
            .map(|c| Channel {
                id: c.id,
                name: c.name.clone(),
                kind: c.kind.clone(),
            })
            .collect();

        Ok(channels)
    }
}

pub struct SharedRepositoryOptions(pub u64);
impl RepositoryOptions<GuildId> for SharedRepositoryOptions {
    fn get_cache_key(&self) -> GuildId {
        GuildId(self.0)
    }
}

pub struct RoleRepository;
#[async_trait]
impl Repository<Role, SharedRepositoryOptions> for RoleRepository {
    fn cache(options: &SharedRepositoryOptions, values: &Vec<Role>) -> bool {
        RolesCache::set(options.get_cache_key(), &values)
    }

    async fn get_cached(options: &SharedRepositoryOptions) -> Result<Vec<Role>, String> {
        RolesCache::get(options.get_cache_key()).ok_or(String::from("Not found."))
    }

    async fn get_uncached(f: &SharedRepositoryOptions) -> Result<Vec<Role>, String> {
        let call = DiscordCall::new(AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()));
        let result = call.call(GetRoles { guild_id: f.0 }).await?;

        let roles: Vec<Role> = result
            .roles
            .into_iter()
            .map(|c| Role {
                id: c.id,
                name: c.name.clone(),
            })
            .collect();

        Ok(roles)
    }
}

pub struct MemberRepository;

#[async_trait]
impl Repository<Member, SharedRepositoryOptions> for MemberRepository {
    fn cache(options: &SharedRepositoryOptions, values: &Vec<Member>) -> bool {
        MembersCache::set(options.get_cache_key(), &values)
    }

    async fn get_cached(options: &SharedRepositoryOptions) -> Result<Vec<Member>, String> {
        MembersCache::get(options.get_cache_key()).ok_or(String::from("Not found."))
    }

    async fn get_uncached(f: &SharedRepositoryOptions) -> Result<Vec<Member>, String> {
        let call = DiscordCall::new(AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()));
        let result = call.call(GetMembers(f.0)).await?;

        let members: Vec<Member> = result
            .members
            .into_iter()
            .map(|c| Member {
                id: c.user.id,
                username: c.user.username.clone(),
                discriminator: c.user.discriminator.parse::<u16>().unwrap(),
            })
            .collect();

        Ok(members)
    }
}

pub struct GuildRepositoryOptions(pub u64, pub AccessToken);
impl RepositoryOptions<UserId> for GuildRepositoryOptions {
    fn get_cache_key(&self) -> UserId {
        UserId(self.0)
    }
}

pub struct MutualGuildRepository;
#[async_trait]
impl Repository<Guild, GuildRepositoryOptions> for MutualGuildRepository {
    fn cache(options: &GuildRepositoryOptions, values: &Vec<Guild>) -> bool {
        GuildsCache::set(options.get_cache_key(), &values)
    }

    async fn get_cached(options: &GuildRepositoryOptions) -> Result<Vec<Guild>, String> {
        GuildsCache::get(options.get_cache_key()).ok_or(String::from("Not found."))
    }

    async fn get_uncached(options: &GuildRepositoryOptions) -> Result<Vec<Guild>, String> {
        let bot_call =
            DiscordCall::new(AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()));
        let user_call = DiscordCall::new(options.1.clone());

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
}
