use redis::ErrorKind;
use redis::FromRedisValue;
use redis::RedisError;
use redis::ToRedisArgs;

use crate::polls::routes::Channel;
use crate::polls::routes::Member;
use crate::polls::routes::Role;

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

#[macro_export]
macro_rules! redis_struct {
    ($($name:ident;)*) => {
        $(
            impl ToRedisArgs for $name {
                fn write_redis_args<W>(&self, out: &mut W)
                where
                    W: ?Sized + redis::RedisWrite,
                {
                    if let Ok(raw) = serde_json::to_string(self) {
                        out.write_arg(raw.as_bytes());
                    }
                }
            }

            impl FromRedisValue for $name {
                fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                    match v {
                        redis::Value::Nil => todo!("nil"),
                        redis::Value::Int(_) => todo!("int"),
                        redis::Value::Data(raw) => match std::str::from_utf8(raw) {
                            Ok(data) => {
                                let result: Result<Self, _> = serde_json::from_str(data);
                                match result {
                                    Ok(data) => Ok(data),
                                    Err(err) => Err(RedisError::from((
                                        ErrorKind::TypeError,
                                        "Response was of incompatible type",
                                        format!("(response was {:?})", err),
                                    ))),
                                }
                            }
                            Err(err) => Err(RedisError::from((
                                ErrorKind::TypeError,
                                "Response was of incompatible type",
                                format!("(response was {:?})", err),
                            ))),
                        },
                        redis::Value::Bulk(_) => todo!("bulk"),
                        redis::Value::Status(_) => todo!("status"),
                        redis::Value::Okay => todo!("okay"),
                    }
                }
            }
        )*
    }
}

redis_struct! {
    Channel;
    Role;
    Member;
}

pub struct ChannelsCache;
impl Cache<GuildId, Vec<Channel>> for ChannelsCache {
    fn get(key: GuildId) -> Option<Vec<Channel>> {
        Self::get_vec::<Channel>(key)
    }

    fn set(key: GuildId, value: Vec<Channel>) -> bool {
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

    fn set(key: GuildId, value: Vec<Role>) -> bool {
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

    fn set(key: GuildId, value: Vec<Member>) -> bool {
        Self::set_vec::<Member>(key, value)
    }

    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }

    fn get_additional_namespace() -> Option<String> {
        Some("members".into())
    }
}

pub struct GuildNameCache;
impl Cache<GuildId, String> for GuildNameCache {
    fn get_expire(_key: &GuildId) -> Option<usize> {
        Some(1800)
    }
}
