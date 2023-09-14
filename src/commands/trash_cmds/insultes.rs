use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

// 1 option pour l' USER
// 1 option pour la chaine de charactère personaliser

pub fn run(options: &[CommandDataOption]) -> String {
    let victime_option = options
        .get(0)
        .expect("Expected victim option")
        .resolved
        .as_ref()
        .expect("Expected victim object");

    let insulte_option = options
        .get(1)
        .expect("Expected message")
        .resolved
        .as_ref()
        .expect("Expected message");

    let mut user_id = 0;
    let mut insulte = String::new();

    if let CommandDataOptionValue::User(user, _member) = victime_option {
        user_id = user.id.0;
    }

    if let CommandDataOptionValue::String(msg) = insulte_option {
        insulte = msg.clone();
    }

    let mention = format!("<@{}>", user_id);

    if user_id != 0 && insulte.is_empty() {
        "mets un texte stp".to_string()
    } else {
        format!("{} {}", insulte, mention)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("insulte")
        .description("insulte a user")
        .create_option(|option| {
            option
                .name("victime")
                .description("The user to blow up")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("trash")
                .description("l'insulte en question")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
