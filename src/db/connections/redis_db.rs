use std::sync::{Arc, Mutex};

use dotenvy;
use redis::{Client, Commands, Connection, RedisResult};
pub struct RedisConManager {
    redis_manager: Arc<Mutex<Connection>>,
}

impl RedisConManager {
    pub fn new() -> RedisResult<Self> {
        let _ = dotenvy::dotenv();
        let redis_pwd = dotenvy::var("REDIS_PWD").expect("la var n'est pas définie");

        let _redis_url = format!(
            "redis://default:{}@mutual-gull-31864.upstash.io:31864",
            redis_pwd
        );
        let client = Client::open("redis://127.0.0.1/")?;
        let redis_con = client.get_connection()?;

        println!("new con initied");

        Ok(Self {
            redis_manager: Arc::new(Mutex::new(redis_con)),
        })
    }

    pub fn set(&self, key: String, value: String) -> RedisResult<()> {
        let mut con = self.redis_manager.lock().unwrap();
        con.set(key, value)?;
        Ok(())
    }

    pub fn exists(&self, key: String) -> RedisResult<bool> {
        let mut con = self.redis_manager.lock().unwrap();
        let exists: bool = con.exists(key)?;
        Ok(exists)
    }

    pub fn increment_score(&self, key: String, increment: i64) -> RedisResult<i64> {
        let mut con = self.redis_manager.lock().unwrap();
        let new_score: i64 = con.incr(key, increment)?;
        Ok(new_score)
    }
}