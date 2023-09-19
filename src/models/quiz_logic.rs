use std::collections::HashSet;

use redis::RedisResult;

use crate::db::connections::redis_db::RedisConManager;

pub fn register_players(
    redis_manager: &RedisConManager,
    user_ids: &HashSet<u64>,
) -> RedisResult<()> {
    for user_id in user_ids.iter() {
        redis_manager.set(user_id.to_string(), "0".to_string())?;
    }
    println!("con successful");
    Ok(())
}

// pub async fn get_highest_score_players(
//     redis_con: &Mutex<redis::Connection>,
//     user_ids: &HashSet<u8>,
// ) -> Result<(u8, Vec<u8>), redis::RedisError> {
//     let mut highest_score = 0;
//     let mut highest_score_players = Vec::new();
//     let mut con = redis_con.lock().await;

//     for &user_id in user_ids {
//         let score: u8 = con.get(user_id.to_string())?;
//         if score > highest_score {
//             highest_score = score;
//             highest_score_players.clear();
//             highest_score_players.push(user_id);
//         } else if score == highest_score {
//             highest_score_players.push(user_id);
//         }
//     }

//     Ok((highest_score, highest_score_players))
// }
