use std::{collections::HashSet, time::Duration};

use rand::Rng;
use redis::RedisResult;
use serenity::{
    builder::CreateEmbed,
    model::prelude::{ChannelId, Message, ReactionType},
    prelude::Context,
    utils::MessageBuilder,
};

use crate::{api::calls::fetch_pokemon_data, db::connections::redis_db::RedisConManager};

pub fn register_players(
    user_ids: &HashSet<u64>,
    redis_manager: &RedisConManager,
) -> RedisResult<()> {
    for user_id in user_ids.iter() {
        redis_manager
            .set(user_id.to_string(), "0".to_string())
            .expect("not registered");
    }
    println!("con successful");
    Ok(())
}

pub async fn create_countdown(ctx: &Context, msg_content: &Message, mut countdown: i64) {
    let ctx_clone = ctx.clone();
    let message_clone = msg_content.clone();

    let emojis_bank = ["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣"];
    let mut emoji_to_add = Vec::new();

    if let Err(why) = message_clone
        .react(&ctx_clone, ReactionType::Unicode("⏳".to_string()))
        .await
    {
        println!("Erreur lors de l'ajout de la réaction : {:?}", why);
    }

    tokio::spawn(async move {
        while countdown > 0 {
            emoji_to_add.clear();
            let countdown_str = countdown.to_string();
            for digit in countdown_str.chars() {
                let index = digit.to_digit(10).unwrap() as usize;
                emoji_to_add.push(emojis_bank[index]);
            }

            for &emoji_str in emoji_to_add.iter() {
                let emoji = ReactionType::Unicode(emoji_str.to_string());

                if let Err(why) = message_clone.react(&ctx_clone, emoji).await {
                    println!("Erreur lors de l'ajout de la réaction : {:?}", why);
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;

            countdown -= 1;

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
    channel_id: &ChannelId,
    countdown: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let random_id = rand::thread_rng().gen_range(1..=1010);

        let exists = redis_manager.exists(random_id.to_string())?;

        if !exists {
            let (image, name) = fetch_pokemon_data(random_id).await?;
            println!("{} 1", name);

            let mut embed = CreateEmbed::default();
            embed.title("Qui est ce Pokémon ?");
            embed.image(image);

            let embed_result = channel_id
                .send_message(&ctx.http, |m| m.set_embed(embed))
                .await;

            if let Ok(message) = embed_result {
                create_countdown(&ctx, &message, countdown).await;
                add_score(name, &ctx, &message, &channel_id).await;
            } else if let Err(why) = embed_result {
                println!("Error sending message: {:?}", why);
            }
            redis_manager.set(random_id.to_string(), "true".to_string())?;

            break;
        }
    }

    Ok(())
}

pub async fn add_score(
    answer: String,
    ctx: &Context,
    msg_content: &Message,
    channel_id: &ChannelId,
) {
    let redis_manager = RedisConManager::new().expect("Failed to initialize RedisConManager");

    let answer_clone = answer.clone();
    let ctx_clone = ctx.clone();
    let msg_content_clone = msg_content.clone();
    let channel_id_clone = channel_id.clone();

    tokio::spawn(async move {
        loop {
            println!("Waiting for reply...");
            if let Some(right_answer) = &msg_content_clone
                .author
                .await_reply(&ctx_clone)
                .timeout(Duration::from_secs(16))
                .await
            {
                println!("Received reply");
                println!("{} 2", answer_clone);

                if right_answer.content.to_lowercase() == answer_clone {
                    redis_manager
                        .increment_score(right_answer.author.id.0.to_string(), 1)
                        .expect("not scored");

                    msg_content_clone
                        .delete(&ctx_clone)
                        .await
                        .expect("Failed to delete message");
                    break;
                }
            }
        }
        let response = MessageBuilder::new()
            .push_bold_safe(&msg_content_clone.author.name)
            .push(", on a compris que t'as trouvé c bon 🤓")
            .build();

        if let Err(why) = channel_id_clone.say(&ctx_clone.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    });
}
