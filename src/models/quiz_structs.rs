use std::{collections::HashSet, sync::atomic::Ordering};

use serenity::{model::prelude::ChannelId, prelude::Context, utils::MessageBuilder};
use sqlx::PgPool;

use crate::{
    commands::states::QuizState,
    db::{quiz_sql::add_winner, redis_db::RedisConManager},
};

use super::{
    quiz_logic::{ask_question, register_players},
    quiz_mess::register_msg,
};

pub struct QuizBuilder<'a> {
    redis_manager: &'a RedisConManager,
    ctx: Context,
    channel_id: ChannelId,
    countdown: i64,
    user_ids: HashSet<u64>,
    theme: String,
    player_count: u32,
    score: i64,
    pool: &'a PgPool,
}

impl<'a> QuizBuilder<'a> {
    pub fn build_quiz(
        valid_redis_manager: &'a RedisConManager,
        valid_ctx: Context,
        valid_channel_id: ChannelId,
        valid_countdown: i64,
        valid_theme: String,
        valid_score: i64,
        valid_pool: &'a PgPool,
    ) -> Self {
        Self {
            redis_manager: valid_redis_manager,
            ctx: valid_ctx,
            channel_id: valid_channel_id,
            countdown: valid_countdown,
            user_ids: HashSet::new(),
            theme: valid_theme,
            player_count: 0,
            score: valid_score,
            pool: valid_pool,
        }
    }

    pub async fn lesgo(&mut self) {
        self.quiz_mess().await;
    }

    pub async fn quiz_mess(&mut self) {
        register_msg(
            &self.theme,
            self.channel_id,
            &self.ctx,
            &mut self.user_ids,
            &mut self.player_count,
        )
        .await;
        register_players(&self.user_ids, &self.redis_manager).expect("not registered");
        println!("{:?} so deleted", self.user_ids);
        self.question_builder().await;
    }

    pub async fn check_winner(&mut self) -> Option<u64> {
        let scores = self.redis_manager.get_ids_scores(&self.user_ids).unwrap();
        for (user_id, score) in self.user_ids.iter().zip(scores.iter()) {
            if *score >= self.score as u64 {
                if let Err(why) = add_winner(self.pool, *user_id as i64).await {
                    println!(
                        "Erreur lors de l'ajout du vainqueur à la base de données : {:?}",
                        why
                    );
                }
                return Some(*user_id);
            }
        }
        None
    }

    pub async fn question_builder(&mut self) {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<bool>(1);

        loop {
            let tx_clone = tx.clone();
            let _ = ask_question(
                &self.redis_manager,
                &self.ctx,
                &self.channel_id,
                self.countdown,
                tx_clone,
            )
            .await
            .expect("mess bug");

            if let Some(true) = rx.recv().await {
                if let Some(winner) = self.check_winner().await {
                    let mention = format!("<@{}>", winner);
                    let end_msg = MessageBuilder::new()
                        .push(mention)
                        .push_bold("à gagné")
                        .build();

                    if let Err(why) = self.channel_id.say(&self.ctx.http, &end_msg).await {
                        println!("Error sending message: {:?}", why);
                    }

                    let data = self.ctx.data.read().await;
                    let quiz_state = data
                        .get::<QuizState>()
                        .expect("Expected QuizState in TypeMap");
                    quiz_state.store(false, Ordering::SeqCst);

                    self.user_ids.clear();
                    self.redis_manager.clear_db().expect("failed to clear db");
                    break;
                }
            }
        }
    }
}
