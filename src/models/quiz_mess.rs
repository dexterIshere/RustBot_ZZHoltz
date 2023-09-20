use std::{collections::HashSet, path::Path};

use serenity::{
    builder::CreateButton,
    model::prelude::{component::ButtonStyle, ChannelId, InteractionResponseType, ReactionType},
    prelude::Context,
    utils::MessageBuilder,
};

use super::quiz_logic::create_countdown;

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

pub async fn register_msg(
    theme: &String,
    channel_id: ChannelId,
    ctx: &Context,
    countdown: i64,
    user_ids: &mut HashSet<u64>,
) {
    let param = MessageBuilder::new()
        .push(" souhaite faire un quiz de ")
        .push_bold(&theme)
        .build();

    let mut player_count = 0;
    let filepath = format!("./static/{}.jpg", &theme);
    let path = Path::new(&filepath);

    let m = channel_id
        .send_message(&ctx, |m| {
            m.content(&param)
                .components(|c| {
                    c.create_action_row(|r| {
                        r.add_button(quiz_button("I play", "🤙".parse().unwrap()));
                        r.add_button(counter("player_nbr", "nombre de joueur : 0"))
                    })
                })
                .add_file(path)
        })
        .await;

    if let Ok(mut message) = m {
        create_countdown(&ctx, &message, countdown).await;
        loop {
            if let Some(interaction) = message.await_component_interaction(&ctx).await {
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
            }
        }
    }
}
