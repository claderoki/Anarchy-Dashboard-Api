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
