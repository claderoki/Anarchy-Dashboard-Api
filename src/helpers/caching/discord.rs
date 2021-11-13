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
impl Cache<UserId, Vec<u64>> for GuildsCache {
    fn get(key: UserId) -> Option<Vec<u64>> {
        Self::get_vec::<u64>(key)
    }

    fn set(key: UserId, value: Vec<u64>) -> bool {
        Self::set_vec::<u64>(key, value)
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

pub struct ChannelsCache;
impl Cache<GuildId, Vec<u64>> for ChannelsCache {
    fn get(key: GuildId) -> Option<Vec<u64>> {
        Self::get_vec::<u64>(key)
    }

    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }

    fn set(key: GuildId, value: Vec<u64>) -> bool {
        Self::set_vec::<u64>(key, value)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("channels".into())
    }
}

pub struct RolesCache;
impl Cache<GuildId, Vec<u64>> for RolesCache {
    fn get(key: GuildId) -> Option<Vec<u64>> {
        Self::get_vec::<u64>(key)
    }

    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }

    fn set(key: GuildId, value: Vec<u64>) -> bool {
        Self::set_vec::<u64>(key, value)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("roles".into())
    }
}
