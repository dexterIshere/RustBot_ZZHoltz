use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use serenity::{
    model::prelude::{ChannelId, Message, ReactionType},
    prelude::Context,
};
use tokio::time::sleep;

use crate::db::connections::redis_db::RedisConManager;

// pub struct QuizBuilder {
//     ctx: Context,
//     redis_manager: RedisConManager,
//     timer: u64,
//     score: u64,
//     user_ids: HashSet<u64>,
//     channel_id: ChannelId,
//     player_count: u64,
//     message: Message,
// }

// impl QuizBuilder {
//     // pub fn question_chain() {}

//     // pub fn stop_quiz() {}
// }

pub async fn create_countdown(ctx: &Context, timer: i64, message: &Message) {
    let ctx_clone = ctx.clone();
    let message_clone = message.clone();
    let timer_clone = timer.clone();

    if let Err(why) = message_clone
        .react(&ctx_clone, ReactionType::Unicode("⏳".to_string()))
        .await
    {
        println!("Erreur lors de l'ajout de la réaction : {:?}", why);
    }

    tokio::spawn(async move {
        let emojis_bank = ["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣"];
        let mut countdown = timer_clone;

        while countdown > 0 {
            let countdown_tostr = countdown.to_string();
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

            countdown -= 1;

            for &emoji_str in emoji_to_add.iter() {
                let emoji = ReactionType::Unicode(emoji_str.to_string());

                if let Err(why) = message_clone.delete_reaction_emoji(&ctx_clone, emoji).await {
                    println!("Erreur lors de la suppression des réactions : {:?}", why);
                }
            }
        }
    });
}
