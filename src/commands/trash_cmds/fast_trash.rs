use crate::models::sentences::select_random_sentence;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use sqlx::PgPool;

pub async fn run(options: &[CommandDataOption], pool: &PgPool) -> String {
    let insulte = match select_random_sentence(&pool).await {
        Ok(val) => val,
        Err(_) => "Une erreur est survenue".to_string(),
    };
    let victime_option = options
        .get(0)
        .expect("Expected victim option")
        .resolved
        .as_ref()
        .expect("Expected victim object");

    let mut user_id = 0;

    if let CommandDataOptionValue::User(user, _member) = victime_option {
        user_id = user.id.0;
    }

    let mention = format!("<@{}>", user_id);

    format!("{} {}", insulte, mention)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("balle_perdu")
        .description("envoie rapidement une insulte aléatoire")
        .create_option(|option| {
            option
                .name("victime")
                .description("The user to free trash")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
