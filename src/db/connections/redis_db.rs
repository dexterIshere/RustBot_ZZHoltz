use std::sync::Arc;

use dotenvy;
use redis::{Connection, RedisResult};
use tokio::sync::Mutex;

pub fn init_redis_con() -> RedisResult<Connection> {
    let _ = dotenvy::dotenv();
    let redis_pwd = dotenvy::var("REDIS_PWD").expect("la var n'est pas définie");

    let redis_url = format!(
        "redis://default:{}@allowing-warthog-36110.upstash.io:36110",
        redis_pwd
    );
    let client = redis::Client::open(redis_url)?;
    let redis_con = client.get_connection()?;
    println!("con initied");
    Ok(redis_con)
}
