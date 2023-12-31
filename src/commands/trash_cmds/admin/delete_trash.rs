use crate::db::sentences::delete_trash;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::Permissions;
use serenity::utils::MessageBuilder;
use sqlx::PgPool;

pub async fn run(
    options: &[CommandDataOption],
    command: &ApplicationCommandInteraction,
    pool: &PgPool,
) -> String {
    let delete_option = options
        .get(0)
        .expect("chiffre invalide")
        .resolved
        .as_ref()
        .expect("chiffre invalide");

    let mut deleted: u64 = 0;

    if let CommandDataOptionValue::Integer(int) = delete_option {
        deleted = *int as u64;
    }

    let _ = delete_trash(pool, deleted.try_into().unwrap()).await;

    let response = MessageBuilder::new()
        .push_bold_safe(&command.user.name)
        .push(" à suprimé l'insulte n° ")
        .push(&deleted)
        .push(" de banque d'insultes")
        .build();

    format!("{}", response)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let permissions = Permissions::ADMINISTRATOR;
    command
        .name("delete_bullshit")
        .description("suprime une balle perdu de la liste")
        .default_member_permissions(permissions)
        .create_option(|option| {
            option
                .name("numéro")
                .description("le numéro de la phrase à suprimer")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
