use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use rand::Rng;
use redis::RedisResult;
use serenity::{
    builder::CreateEmbed,
    collector::MessageCollectorBuilder,
    futures::StreamExt,
    model::prelude::{ChannelId, Message, ReactionType},
    prelude::Context,
    utils::MessageBuilder,
};

use crate::{api::calls::fetch_pokemon_data, db::redis_db::RedisConManager};

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

pub async fn create_countdown(
    ctx: &Context,
    msg_content: &Message,
    mut countdown: i64,
    tx: tokio::sync::mpsc::Sender<bool>,
    flag: Arc<AtomicBool>,
) {
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
    if flag.load(Ordering::SeqCst) {
        return; // Sortez de la fonction si le drapeau est vrai
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

                let _ = message_clone.delete_reaction_emoji(&ctx_clone, emoji).await;
            }
        }
        if !flag.load(Ordering::SeqCst) {
            flag.store(true, Ordering::SeqCst);
            tx.send(true).await.expect("Failed to send signal");
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
    tx: tokio::sync::mpsc::Sender<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let flag = Arc::new(AtomicBool::new(false));
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
                let tx_for_countdown = tx.clone();
                let tx_for_add_score = tx.clone();
                let flag_for_countdown = flag.clone();
                let flag_for_add_score = flag.clone();
                create_countdown(
                    &ctx,
                    &message,
                    countdown,
                    tx_for_countdown,
                    flag_for_countdown,
                )
                .await;

                add_score(
                    name,
                    &ctx,
                    &message,
                    &channel_id,
                    tx_for_add_score,
                    flag_for_add_score,
                )
                .await
                .expect("revoie la fonction add_score");
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
    message: &Message,
    channel_id: &ChannelId,
    tx: tokio::sync::mpsc::Sender<bool>,
    flag: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let redis_manager = RedisConManager::new().expect("Failed to initialize RedisConManager");

    let ctx_clone = ctx.clone();
    let message_clone = message.clone();
    let channel_id_clone = channel_id.clone();
    if flag.load(Ordering::SeqCst) {
        return Ok(()); // Sortez de la fonction si le drapeau est vrai
    }

    tokio::spawn(async move {
        let mut collector = MessageCollectorBuilder::new(&ctx_clone)
            .channel_id(message_clone.channel_id)
            .build();

        while let Some(received_message) = collector.next().await {
            if received_message.content.to_lowercase() == answer.to_lowercase() {
                redis_manager
                    .increment_score(received_message.author.id.0.to_string(), 1)
                    .expect("not scored");
                let response = MessageBuilder::new()
                    .mention(&received_message.author.id)
                    .push_bold(", Bonne réponse 🤓")
                    .build();

                if let Err(why) = channel_id_clone.say(&ctx_clone.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                message_clone
                    .delete(&ctx_clone.http)
                    .await
                    .expect("Failed to delete message");
                flag.store(true, Ordering::SeqCst);
                tx.send(true).await.expect("Failed to send signal");
                break;
            }
        }
    });

    Ok(())
}
