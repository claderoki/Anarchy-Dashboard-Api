use redis::ErrorKind;
use redis::FromRedisValue;
use redis::RedisError;
use redis::ToRedisArgs;

use crate::discord::models::Channel;
use crate::discord::models::Guild;
use crate::discord::models::Member;
use crate::discord::models::Role;
use crate::redis_struct;

use super::base::Cache;
use super::base::CacheKey;

#[derive(Debug)]
pub struct AccessTokenHash {
    pub hash: String,
    pub expires_in: Option<usize>,
}

impl AccessTokenHash {
    pub fn new(hash: &str) -> Self {
        Self {
            hash: hash.into(),
            expires_in: None,
        }
    }

    pub fn new_with_expires_in(hash: &str, expires_in: usize) -> Self {
        Self {
            hash: hash.into(),
            expires_in: Some(expires_in),
        }
    }
}

impl CacheKey for AccessTokenHash {
    const KEY: &'static str = "access_token_hash";

    fn get_key(&self) -> String {
        format!("{}:{}", Self::KEY, self.hash)
    }
}

pub struct UserIdCache;
impl Cache<AccessTokenHash, u64> for UserIdCache {
    fn get_expire(key: &AccessTokenHash) -> Option<usize> {
        key.expires_in
    }
}

#[derive(Debug)]
pub struct UserId(pub(crate) u64);
impl CacheKey for UserId {
    const KEY: &'static str = "users";

    fn get_key(&self) -> String {
        format!("{}:{}", Self::KEY, self.0)
    }
}

pub struct GuildsCache;
impl Cache<UserId, Vec<Guild>> for GuildsCache {
    fn get(key: UserId) -> Option<Vec<Guild>> {
        Self::get_vec::<Guild>(key)
    }

    fn set(key: UserId, value: &Vec<Guild>) -> bool {
        Self::set_vec::<Guild>(key, value)
    }

    fn get_expire(_key: &UserId) -> Option<usize> {
        Some(1800)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("guilds".into())
    }
}

#[derive(Debug)]
pub struct GuildId(pub(crate) u64);
impl CacheKey for GuildId {
    const KEY: &'static str = "guilds";

    fn get_key(&self) -> String {
        format!("{}:{}", Self::KEY, self.0)
    }
}

redis_struct! {
    Channel;
    Role;
    Member;
    Guild;
}

pub struct ChannelsCache;
impl Cache<GuildId, Vec<Channel>> for ChannelsCache {
    fn get(key: GuildId) -> Option<Vec<Channel>> {
        Self::get_vec::<Channel>(key)
    }

    fn set(key: GuildId, value: &Vec<Channel>) -> bool {
        Self::set_vec::<Channel>(key, value)
    }

    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("channels".into())
    }
}

pub struct RolesCache;
impl Cache<GuildId, Vec<Role>> for RolesCache {
    fn get(key: GuildId) -> Option<Vec<Role>> {
        Self::get_vec::<Role>(key)
    }

    fn set(key: GuildId, value: &Vec<Role>) -> bool {
        Self::set_vec::<Role>(key, value)
    }

    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("roles".into())
    }
}

pub struct MembersCache;
impl Cache<GuildId, Vec<Member>> for MembersCache {
    fn get(key: GuildId) -> Option<Vec<Member>> {
        Self::get_vec::<Member>(key)
    }

    fn set(key: GuildId, value: &Vec<Member>) -> bool {
        Self::set_vec::<Member>(key, value)
    }

    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("members".into())
    }
}
