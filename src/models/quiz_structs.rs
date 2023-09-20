use std::collections::HashSet;

use serenity::{model::prelude::ChannelId, prelude::Context};

use crate::db::connections::redis_db::RedisConManager;

use super::{
    quiz_logic::{ask_question, register_players},
    quiz_mess::register_msg,
};

enum QuizState {
    Stopped,
    Registering,
    Active,
}

pub struct QuizBuilder {
    state: QuizState,
    redis_manager: RedisConManager,
    ctx: Context,
    channel_id: ChannelId,
    countdown: i64,
    user_ids: HashSet<u64>,
    theme: String,
}

impl QuizBuilder {
    pub fn build_quiz(
        valid_redis_manager: RedisConManager,
        valid_ctx: Context,
        valid_channel_id: ChannelId,
        valid_countdown: i64,
        valid_theme: String,
    ) -> Self {
        Self {
            state: QuizState::Stopped,
            redis_manager: valid_redis_manager,
            ctx: valid_ctx,
            channel_id: valid_channel_id,
            countdown: valid_countdown,
            user_ids: HashSet::new(),
            theme: valid_theme,
        }
    }

    pub async fn lesgo(&mut self) {
        match self.state {
            QuizState::Stopped => self.quiz_mess().await,
            QuizState::Registering => {
                println!("Marche pas")
            }
            QuizState::Active => {
                println!("Marche pas")
            }
        }
    }

    pub async fn quiz_mess(&mut self) {
        self.state = QuizState::Registering;
        register_msg(
            &self.theme,
            self.channel_id,
            &self.ctx,
            self.countdown,
            &mut self.user_ids,
        );
        register_players(&self.user_ids, &self.redis_manager).expect("not registered");

        self.state = QuizState::Active;
        self.question_builder();
    }

    pub async fn question_builder(&mut self) {
        ask_question(
            &self.redis_manager,
            &self.ctx,
            self.channel_id,
            self.countdown,
        );
        self.state = QuizState::Stopped;
    }
}
