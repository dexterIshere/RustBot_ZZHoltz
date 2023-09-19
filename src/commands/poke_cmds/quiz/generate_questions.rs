use crate::{
    api::pokemons::pokecalls::fetch_pokemon_data, db::connections::redis_db::RedisConManager,
};
use rand::Rng;
use serenity::{builder::CreateEmbed, model::prelude::ChannelId, prelude::Context};

use super::quiz_builder::create_countdown;

pub async fn ask_question(
    ctx: &Context,
    channel_id: ChannelId,
    redis_manager: &RedisConManager,
    timer: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let random_id = rand::thread_rng().gen_range(1..=1010);

        let exists = redis_manager.exists(random_id.to_string())?;

        if !exists {
            let (image, _name) = fetch_pokemon_data(random_id).await?;

            let mut embed = CreateEmbed::default();
            embed.title("Qui est ce Pokémon ?");
            embed.image(image);

            let embed_result = channel_id
                .send_message(&ctx.http, |m| m.set_embed(embed))
                .await;

            if let Ok(message) = embed_result {
                create_countdown(&ctx, timer, &message).await;
            } else if let Err(why) = embed_result {
                println!("Error sending message: {:?}", why);
            }

            redis_manager.set(random_id.to_string(), "true".to_string())?;

            break;
        }
    }

    Ok(())
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
