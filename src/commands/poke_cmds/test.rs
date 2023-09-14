use crate::api::pokemons::pokecalls::test;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::Permissions;

pub async fn run(_options: &[CommandDataOption]) -> String {
    match test().await {
        Ok(test_result) => format!("{}", test_result),
        Err(_) => "Une erreur s'est produite".to_string(),
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    let permissions = Permissions::ADMINISTRATOR;
    command
        .name("test")
        .description("testPokemon fetch")
        .default_member_permissions(permissions)
}
