use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

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

    pub fn get_ids_scores(&self, user_ids: &HashSet<u64>) -> RedisResult<Vec<u64>> {
        let mut con = self.redis_manager.lock().unwrap();
        let mut score = Vec::new();

        for &user_id in user_ids.iter() {
            let id_score: u64 = con.get(user_id.to_string())?;
            score.push(id_score);
        }

        Ok(score)
    }

    pub fn clear_db(&self) -> RedisResult<()> {
        let mut con = self.redis_manager.lock().unwrap();
        let keys: Vec<String> = con.keys("*")?;

        for key in keys {
            con.del(key)?;
        }
        Ok(())
    }
}
