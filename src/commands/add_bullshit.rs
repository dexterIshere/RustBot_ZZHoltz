use crate::models::sentences::add_trash;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::utils::MessageBuilder;
use sqlx::PgPool;

pub async fn run(
    options: &[CommandDataOption],
    command: &ApplicationCommandInteraction,
    pool: &PgPool,
) -> String {
    let new_insulte_option = options
        .get(0)
        .expect("écris une insulte")
        .resolved
        .as_ref()
        .expect("écris une insulte");

    let mut new_trash = String::new();

    if let CommandDataOptionValue::String(msg) = new_insulte_option {
        new_trash = msg.clone();
    }

    let _ = add_trash(pool, &new_trash).await;

    let response = MessageBuilder::new()
        .push_bold_safe(&command.user.name)
        .push(" à ajouté l'insulte: ")
        .push(&new_trash)
        .push(" à la banque d'insultes")
        .build();

    format!("{}", response)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_bullshit")
        .description("ajoute une balle perdu suplémentaire à la liste")
        .create_option(|option| {
            option
                .name("trash_quote")
                .description("the trash quote")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
