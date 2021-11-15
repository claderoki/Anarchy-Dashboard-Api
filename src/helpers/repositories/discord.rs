use std::env;

use crate::discord::calls::ChannelKind;
use crate::discord::calls::GetChannels;
use crate::discord::calls::GetMembers;
use crate::discord::calls::GetRoles;
use crate::discord::discord_base::AccessToken;
use crate::discord::discord_base::DiscordCall;

use crate::discord::base_api::Callable;
use crate::helpers::caching::base::Cache;

use crate::helpers::caching::discord::ChannelsCache;
use crate::helpers::caching::discord::GuildId;

use crate::helpers::caching::discord::MembersCache;
use crate::helpers::caching::discord::RolesCache;
use crate::polls::routes::Channel;
use crate::polls::routes::Member;
use crate::polls::routes::Role;

pub struct ChannelRepository;
impl ChannelRepository {
    pub async fn get(guild_id: u64, channel_kind: ChannelKind) -> Result<Vec<Channel>, String> {
        match Self::get_cached(guild_id, &channel_kind).await {
            Ok(channels) => Ok(channels),
            Err(_) => {
                let uncached = Self::get_uncached(guild_id, &channel_kind).await?;
                let mut clone: Vec<Channel> = Vec::new();
                for channel in uncached.iter() {
                    clone.push(Channel {
                        id: channel.id,
                        name: channel.name.clone(),
                        kind: channel.kind.clone(),
                    });
                }
                ChannelsCache::set(GuildId(guild_id), clone);

                Ok(uncached)
            }
        }
    }

    pub async fn get_cached(
        guild_id: u64,
        channel_kind: &ChannelKind,
    ) -> Result<Vec<Channel>, String> {
        let channels = ChannelsCache::get(GuildId(guild_id)).ok_or(String::from("Not found."))?;
        Ok(channels
            .into_iter()
            .filter(|c| &c.kind == channel_kind)
            .collect::<Vec<Channel>>())
    }

    pub async fn get_uncached(
        guild_id: u64,
        channel_kind: &ChannelKind,
    ) -> Result<Vec<Channel>, String> {
        let call = DiscordCall {
            access_token: AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()),
        };

        let mut channels: Vec<Channel> = Vec::new();

        let result = call.call(GetChannels { guild_id }).await?;

        for channel in result.channels.iter() {
            if &channel.kind == channel_kind {
                if let Ok(id) = channel.id.parse::<u64>() {
                    channels.push(Channel {
                        id,
                        name: channel.name.clone(),
                        kind: channel.kind.clone(),
                    });
                }
            }
        }

        Ok(channels)
    }
}

pub struct RoleRepository;
impl RoleRepository {
    pub async fn get(guild_id: u64) -> Result<Vec<Role>, String> {
        match Self::get_cached(guild_id).await {
            Ok(roles) => Ok(roles),
            Err(_) => {
                let uncached = Self::get_uncached(guild_id).await?;
                let mut clone: Vec<Role> = Vec::new();
                for channel in uncached.iter() {
                    clone.push(Role {
                        id: channel.id,
                        name: channel.name.clone(),
                    });
                }
                RolesCache::set(GuildId(guild_id), clone);

                Ok(uncached)
            }
        }
    }

    pub async fn get_cached(guild_id: u64) -> Result<Vec<Role>, String> {
        RolesCache::get(GuildId(guild_id)).ok_or(String::from("Not found."))
    }

    pub async fn get_uncached(guild_id: u64) -> Result<Vec<Role>, String> {
        let call = DiscordCall {
            access_token: AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()),
        };

        let mut roles: Vec<Role> = Vec::new();

        let result = call.call(GetRoles { guild_id }).await?;

        for channel in result.roles.iter() {
            if let Ok(id) = channel.id.parse::<u64>() {
                roles.push(Role {
                    id,
                    name: channel.name.clone(),
                });
            }
        }

        Ok(roles)
    }
}

pub struct MemberRepository;
impl MemberRepository {
    pub async fn get(guild_id: u64) -> Result<Vec<Member>, String> {
        match Self::get_cached(guild_id).await {
            Ok(members) => Ok(members),
            Err(_) => {
                let uncached = Self::get_uncached(guild_id).await?;
                let mut clone: Vec<Member> = Vec::new();
                for member in uncached.iter() {
                    clone.push(Member {
                        id: member.id,
                        username: member.username.clone(),
                        discriminator: member.discriminator,
                    });
                }
                MembersCache::set(GuildId(guild_id), clone);

                Ok(uncached)
            }
        }
    }

    pub async fn get_cached(guild_id: u64) -> Result<Vec<Member>, String> {
        MembersCache::get(GuildId(guild_id)).ok_or(String::from("Not found."))
    }

    pub async fn get_uncached(guild_id: u64) -> Result<Vec<Member>, String> {
        let call = DiscordCall {
            access_token: AccessToken::Bot(env::var("DISCORD_CLIENT_TOKEN").unwrap()),
        };

        let mut members: Vec<Member> = Vec::new();

        let result = call.call(GetMembers { guild_id }).await?;

        for member in result.members.iter() {
            if let Ok(id) = member.user.id.parse::<u64>() {
                members.push(Member {
                    id,
                    username: member.user.username.clone(),
                    discriminator: member.user.discriminator.parse::<u16>().unwrap(),
                });
            }
        }

        Ok(members)
    }
}
