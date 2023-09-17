use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateButton;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
// use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::ReactionType;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;
use tokio::sync::Mutex;

use crate::commands::poke_cmds::quiz::generate_questions::create_question;
use crate::models::quiz_logic::register_players;

fn quiz_button(name: &str, emoji: ReactionType) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.emoji(emoji);
    b.label(name);
    b.style(ButtonStyle::Primary);
    b
}

fn counter(name: &str, label: &str) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label(label);
    b.style(ButtonStyle::Primary);
    b
}

pub async fn quizz_run(
    options: &[CommandDataOption],
    command: &ApplicationCommandInteraction,
    ctx: Context,
    redis_con: &Arc<Mutex<redis::Connection>>,
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
    let mut score: i64 = 0;

    if let CommandDataOptionValue::String(msg) = quiz_theme {
        theme = msg.clone();
    }
    if let CommandDataOptionValue::Integer(msg) = quiz_timer {
        timer = *msg;
    }
    if let CommandDataOptionValue::Integer(msg) = quiz_score {
        score = *msg;
    }

    let param = MessageBuilder::new()
        .mention(&command.user.id)
        .push(" souhaite faire un quiz de ")
        .push_bold(&theme)
        .build();

    match theme.as_str() {
        "Pokemon" => init_pokemon_quizz(ctx.clone(), param, command, redis_con, timer, score).await,
        "Drapeaux" => init_flags_quizz(ctx.clone(), param, command, redis_con, timer, score).await,
        _ => {
            println!("Thème inconnu");
        }
    }
}

async fn init_pokemon_quizz(
    ctx: Context,
    param: String,
    command: &ApplicationCommandInteraction,
    redis_con: &Arc<Mutex<redis::Connection>>,
    timer: i64,
    score: i64,
) {
    let mut user_ids: HashSet<u64> = HashSet::new();
    let mut player_count = 0;

    let m = command
        .channel_id
        .send_message(&ctx, |m| {
            m.content(&param)
                .components(|c| {
                    c.create_action_row(|r| {
                        r.add_button(quiz_button("I play", "🤙".parse().unwrap()));
                        r.add_button(counter("player_nbr", "nombre de joueur : 0"))
                    })
                })
                .add_file("./static/pokequiz.jpg")
        })
        .await;

    if let Ok(mut message) = m {
        loop {
            if let Some(interaction) = message
                .await_component_interaction(&ctx)
                .timeout(Duration::from_secs(15))
                .await
            {
                if interaction.data.custom_id == "I play" {
                    if user_ids.contains(&(interaction.user.id.0)) {
                        player_count -= 1;
                        user_ids.remove(&(interaction.user.id.0));
                    } else {
                        player_count += 1;
                        user_ids.insert(interaction.user.id.0);
                    }

                    message
                        .edit(&ctx, |m| {
                            m.components(|c| {
                                c.create_action_row(|r| {
                                    r.add_button(quiz_button("I play", "🤙".parse().unwrap()));
                                    r.add_button(counter(
                                        "player_nbr",
                                        &format!("nombre de joueur : {}", player_count),
                                    ))
                                })
                            })
                        })
                        .await
                        .unwrap();
                    interaction
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::DeferredUpdateMessage)
                        })
                        .await
                        .unwrap();
                }
            } else {
                register_players(redis_con, &user_ids)
                    .await
                    .expect("not registered");

                println!("{},{}", timer, score);
                message.delete(&ctx).await.unwrap();
                break;
            }
        }
    }
}

async fn init_flags_quizz(
    ctx: Context,
    param: String,
    command: &ApplicationCommandInteraction,
    redis_con: &Arc<Mutex<redis::Connection>>,
    timer: i64,
    score: i64,
) {
    let mut user_ids: HashSet<u64> = HashSet::new();
    let mut player_count = 0;

    let m = command
        .channel_id
        .send_message(&ctx, |m| {
            m.content(&param)
                .components(|c| {
                    c.create_action_row(|r| {
                        r.add_button(quiz_button("I play", "🤙".parse().unwrap()));
                        r.add_button(counter("player_nbr", "nombre de joueur : 0"))
                    })
                })
                .add_file("./static/drapeaux.jpg")
        })
        .await;

    if let Ok(mut message) = m {
        loop {
            if let Some(interaction) = message
                .await_component_interaction(&ctx)
                .timeout(Duration::from_secs(15))
                .await
            {
                if interaction.data.custom_id == "I play" {
                    if user_ids.contains(&(interaction.user.id.0)) {
                        player_count -= 1;
                        user_ids.remove(&(interaction.user.id.0));
                    } else {
                        player_count += 1;
                        user_ids.insert(interaction.user.id.0);
                    }

                    message
                        .edit(&ctx, |m| {
                            m.components(|c| {
                                c.create_action_row(|r| {
                                    r.add_button(quiz_button("I play", "🤙".parse().unwrap()));
                                    r.add_button(counter(
                                        "player_nbr",
                                        &format!("nombre de joueur : {}", player_count),
                                    ))
                                })
                            })
                        })
                        .await
                        .unwrap();
                    interaction
                        .create_interaction_response(&ctx, |r| {
                            r.kind(InteractionResponseType::DeferredUpdateMessage)
                        })
                        .await
                        .unwrap();
                }
            } else {
                register_players(redis_con, &user_ids)
                    .await
                    .expect("not registered");
                println!("{},{}", timer, score);
                message.delete(&ctx).await.unwrap();
                break;
            }
        }
    }
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
                .max_int_value(164)
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
