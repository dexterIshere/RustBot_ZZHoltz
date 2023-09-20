// Standard Library

// Serenity
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
        command::CommandOptionType,
    },
    prelude::Context,
};

// Internal Modules
use crate::{db::connections::redis_db::RedisConManager, models::quiz_structs::QuizBuilder};

// Super Modules

pub async fn quizz_run(
    options: &[CommandDataOption],
    command: &ApplicationCommandInteraction,
    ctx: Context,
    redis_manager: RedisConManager,
) {
    let quiz_theme = options
        .get(0)
        .expect("Expected theme option")
        .resolved
        .as_ref()
        .expect("Expected theme object");
    let quiz_timer = options
        .get(1)
        .expect("Expected timer")
        .resolved
        .as_ref()
        .expect("Expected timer object");
    let quiz_score = options
        .get(2)
        .expect("Expected score")
        .resolved
        .as_ref()
        .expect("Expected score object");

    let mut theme = String::new();
    let mut timer: i64 = 0;
    let mut _score: i64 = 0;

    if let CommandDataOptionValue::String(msg) = quiz_theme {
        theme = msg.clone();
    }
    if let CommandDataOptionValue::Integer(msg) = quiz_timer {
        timer = *msg;
    }
    if let CommandDataOptionValue::Integer(msg) = quiz_score {
        _score = *msg;
    }

    let mut quiz_builder =
        QuizBuilder::build_quiz(redis_manager, ctx, command.channel_id, timer, theme);
    quiz_builder.lesgo();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("quiz")
        .description("Create a quizz")
        .create_option(|option| {
            option
                .name("theme")
                .description("le theme de la partie")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Pokemon", "Pokemon")
                .add_string_choice("Drapeaux", "Drapeaux")
        })
        .create_option(|option| {
            option
                .name("timer")
                .description("le temps de réponse")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(5)
                .max_int_value(16)
        })
        .create_option(|option| {
            option
                .name("objectif")
                .description("le score requis pour gagner la partie")
                .kind(CommandOptionType::Integer)
                .required(true)
                .min_int_value(1)
                .max_int_value(50)
        })
}
