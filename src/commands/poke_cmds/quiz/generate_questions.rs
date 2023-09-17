use std::sync::Arc;

use crate::api::pokemons::pokecalls::fetch_pokemon_data;
use rand::Rng;
use redis::Commands;
use serenity::{model::prelude::ChannelId, prelude::Context, utils::MessageBuilder};
use tokio::sync::Mutex;

pub async fn create_question(
    ctx: &Context,
    channel_id: ChannelId,
    redis_con: &Arc<Mutex<redis::Connection>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    loop {
        let redis_con = Arc::clone(&redis_con);
        let mut con = redis_con.lock().await;

        let random_id: u64 = rng.gen_range(1..=1010);
        let exists: bool = con.exists(random_id.to_string())?;

        if !exists {
            con.set::<String, String, ()>(random_id.to_string(), "used".to_string())?;

            let (image, name) = fetch_pokemon_data(random_id).await?;

            let content = MessageBuilder::new()
                .push_bold_line("Qui est ce Pokémon ?")
                .push(image)
                .push(name)
                .build();

            if let Err(why) = channel_id.say(&ctx.http, &content).await {
                println!("Error sending message: {:?}", why);
            }
        };
    }
}

// pub async fn question_gestion(
//     // timer: u8,
//     score: u8,
//     registered_players: u8,
//     user_ids: HashSet<u8>,
//     redis_con: &Mutex<redis::Connection>,
//     // channel_id: ChannelId,
//     // ctx: Context,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let (highest_score, highest_score_players) = get_highest_score_players(&redis_con, &user_ids)?;

//     println!("{},{:?}", registered_players, highest_score_players);
//     while highest_score < score {
//         // let message = create_question(&ctx, channel_id, redis_con);

//         // if let Ok(message) = channel_id.say(&ctx.http, &content).await {
//         //     sleep(Duration::from_secs(timer as u64));
//         //     message.delete(&ctx.http).await?;
//         // }
//     }
//     Ok(())
// }
