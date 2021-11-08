use redis::Commands;
use redis::Connection;

pub fn get_connection_redis() -> Result<Connection, &'static str> {
    let client =
        redis::Client::open("redis://127.0.0.1/").map_err(|_| "Failed to get redis client")?;
    client
        .get_connection()
        .map_err(|_| "Failed to get Redis connection")
}

pub trait CacheKey {
    const KEY: &'static str;
    fn get_full_key(&self) -> String;
}

pub trait Cache<D: CacheKey, T> {
    fn get(key: D) -> Option<T>;
    fn set(key: D, value: T) -> bool;
}

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

    fn get_full_key(&self) -> String {
        format!("{}:{}", Self::KEY, self.hash)
    }
}

pub struct UserIdCache;
impl Cache<AccessTokenHash, u64> for UserIdCache {
    fn get(key: AccessTokenHash) -> Option<u64> {
        let mut connection = get_connection_redis().ok()?;
        let value: u64 = connection.get(key.get_full_key()).ok()?;
        Some(value)
    }

    fn set(key: AccessTokenHash, value: u64) -> bool {
        if let Ok(mut connection) = get_connection_redis() {
            let full_key = key.get_full_key();
            let result: Result<(), _> = connection.set(&full_key, value);
            if let Some(expire) = key.expires_in {
                let _: Result<(), _> = connection.expire(&full_key, expire);
            }
            result.is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct UserId(pub(crate) u64);

impl CacheKey for UserId {
    const KEY: &'static str = "guilds";

    fn get_full_key(&self) -> String {
        format!("{}:{}", Self::KEY, self.0)
    }
}

pub struct GuildsCache;
impl Cache<UserId, Vec<u64>> for GuildsCache {
    fn get(key: UserId) -> Option<Vec<u64>> {
        let mut connection = get_connection_redis().ok()?;
        let full_key = key.get_full_key();
        if connection.exists(&full_key).ok()? {
            let value: Vec<u64> = connection.lrange(&full_key, 0, 100).ok()?;
            Some(value)
        } else {
            None
        }
    }

    fn set(key: UserId, value: Vec<u64>) -> bool {
        if let Ok(mut connection) = get_connection_redis() {
            let full_key = key.get_full_key();
            let result: Result<(), _> = connection.lpush(&full_key, value);
            result.is_ok()
        } else {
            false
        }
    }
}
