use redis::Commands;
use redis::Connection;
use redis::FromRedisValue;
use redis::ToRedisArgs;

pub fn get_connection_redis() -> Result<Connection, &'static str> {
    let client =
        redis::Client::open("redis://127.0.0.1/").map_err(|_| "Failed to get redis client")?;
    client
        .get_connection()
        .map_err(|_| "Failed to get Redis connection")
}

pub trait CacheKey {
    const KEY: &'static str;
    fn get_key(&self) -> String;
}

pub trait Cache<D: CacheKey, T: ToRedisArgs + FromRedisValue> {
    fn get(key: D) -> Option<T> {
        let mut connection = get_connection_redis().ok()?;
        let value: T = connection.get(key.get_key()).ok()?;
        Some(value)
    }

    fn set(key: D, value: T) -> bool {
        if let Ok(mut connection) = get_connection_redis() {
            let full_key = key.get_key();
            let result: Result<(), _> = connection.set(&full_key, value);
            if let Some(expire) = Self::get_expire(&key) {
                let _: Result<(), _> = connection.expire(&full_key, expire);
            }
            result.is_ok()
        } else {
            false
        }
    }

    fn get_expire(_: &D) -> Option<usize> {
        None
    }

    fn get_vec<F: FromRedisValue>(key: D) -> Option<Vec<F>> {
        let mut connection = get_connection_redis().ok()?;
        let full_key = Self::get_full_key(&key);
        if connection.exists(&full_key).ok()? {
            let value: Vec<F> = connection.lrange(&full_key, 0, 100).ok()?;
            Some(value)
        } else {
            None
        }
    }

    fn set_vec<F: ToRedisArgs>(key: D, value: Vec<F>) -> bool {
        if let Ok(mut connection) = get_connection_redis() {
            let full_key = Self::get_full_key(&key);
            let result: Result<(), _> = connection.lpush(&full_key, value);
            if let Some(expire) = Self::get_expire(&key) {
                let _: Result<(), _> = connection.expire(&full_key, expire);
            }
            result.is_ok()
        } else {
            false
        }
    }

    fn get_full_key(key: &D) -> String {
        if let Some(value) = Self::get_additional_namespace() {
            format!("{}:{}", key.get_key(), value)
        } else {
            key.get_key()
        }
    }

    fn get_additional_namespace() -> Option<String> {
        None
    }
}
