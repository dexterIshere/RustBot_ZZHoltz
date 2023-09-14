use std::collections::HashSet;
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

use crate::commands::shared::states::QuizState;

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
) {
    let quiz_theme = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let mut theme = String::new();

    if let CommandDataOptionValue::String(msg) = quiz_theme {
        theme = msg.clone();
    }

    let quiz_data = ctx.data.read().await;
    let quiz_state_lock = quiz_data
        .get::<QuizState>()
        .expect("Expected QuizState in TypeMap");
    let quiz_state = quiz_state_lock.read().await;
    println!("{}comment ça ce passe", quiz_state);
    let mut quiz_state_write = quiz_state_lock.write().await;

    if *quiz_state {
        command
            .channel_id
            .say(&ctx.http, "Un quiz est déjà en cours.")
            .await
            .expect("Impossible d'envoyer le message.");
    } else {
        let param = MessageBuilder::new()
            .mention(&command.user.id)
            .push(" souhaite faire un quiz de ")
            .push_bold(&theme)
            .build();

        match theme.as_str() {
            "Pokemon" => run_pokemon_quizz(ctx.clone(), param, command).await,
            "Drapeaux" => run_flags_quizz(ctx.clone(), param, command).await,
            _ => {
                println!("Thème inconnu");
            }
        }
    }
}

async fn run_pokemon_quizz(ctx: Context, param: String, command: &ApplicationCommandInteraction) {
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
                    if user_ids.contains(&interaction.user.id.0) {
                        player_count -= 1;
                        user_ids.remove(&interaction.user.id.0);
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
                break;
            }
        }
        user_ids.clear();
    }
}

async fn run_flags_quizz(ctx: Context, param: String, command: &ApplicationCommandInteraction) {
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
                    if user_ids.contains(&interaction.user.id.0) {
                        player_count -= 1;
                        user_ids.remove(&interaction.user.id.0);
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
                break;
            }
        }
        user_ids.clear();
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
}
