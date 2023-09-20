use std::{collections::HashSet, time::Duration};

use rand::Rng;
use redis::RedisResult;
use serenity::{
    builder::CreateEmbed,
    model::prelude::{ChannelId, Message, ReactionType},
    prelude::Context,
};
use tokio::time::sleep;

use crate::{api::calls::fetch_pokemon_data, db::connections::redis_db::RedisConManager};

pub fn register_players(
    user_ids: &HashSet<u64>,
    redis_manager: &RedisConManager,
) -> RedisResult<()> {
    for user_id in user_ids.iter() {
        &redis_manager.set(user_id.to_string(), "0".to_string())?;
    }
    println!("con successful");
    Ok(())
}

pub async fn create_countdown(ctx: &Context, msg_content: &Message, countdown: i64) {
    let ctx_clone = ctx.clone();
    let message_clone = msg_content.clone();
    let countdown_clone = countdown.clone();

    if let Err(why) = message_clone
        .react(&ctx_clone, ReactionType::Unicode("⏳".to_string()))
        .await
    {
        println!("Erreur lors de l'ajout de la réaction : {:?}", why);
    }

    tokio::spawn(async move {
        let emojis_bank = ["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣"];
        let mut ephemeral_countdown = countdown_clone;

        while ephemeral_countdown > 0 {
            let countdown_tostr = ephemeral_countdown.to_string();
            let emoji_to_add: Vec<&str> = countdown_tostr
                .chars()
                .map(|digit| {
                    let index = digit.to_digit(10).unwrap() as usize;
                    emojis_bank[index]
                })
                .collect();

            for &emoji_str in emoji_to_add.iter() {
                let emoji = ReactionType::Unicode(emoji_str.to_string());

                if let Err(why) = message_clone.react(&ctx_clone, emoji).await {
                    println!("Erreur lors de l'ajout de la réaction : {:?}", why);
                }
            }
            sleep(Duration::from_secs(1)).await;

            ephemeral_countdown -= 1;

            for &emoji_str in emoji_to_add.iter() {
                let emoji = ReactionType::Unicode(emoji_str.to_string());

                if let Err(why) = message_clone.delete_reaction_emoji(&ctx_clone, emoji).await {
                    println!("Erreur lors de la suppression des réactions : {:?}", why);
                }
            }
        }

        if let Err(why) = message_clone.delete(&ctx_clone).await {
            println!("Erreur lors de la suppression du message : {:?}", why);
        }
    });
}

pub async fn ask_question(
    redis_manager: &RedisConManager,
    ctx: &Context,
    channel_id: ChannelId,
    countdown: i64,
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
                create_countdown(&ctx, &message, countdown).await;
            } else if let Err(why) = embed_result {
                println!("Error sending message: {:?}", why);
            }
            redis_manager.set(random_id.to_string(), "true".to_string())?;

            break;
        }
    }

    Ok(())
}
