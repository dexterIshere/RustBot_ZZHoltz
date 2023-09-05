use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let user_option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let message_option = options
        .get(1)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let mut user_tag = String::new();
    let mut message = String::new();

    // Vérifier le type de l'option User
    if let CommandDataOptionValue::User(user, _member) = user_option {
        user_tag = user.tag();
    }

    // Vérifier le type de l'option String
    if let CommandDataOptionValue::String(msg) = message_option {
        message = msg.clone();
    }
    if user_tag.is_empty() || message.is_empty() {
        "Please provide both a user and a message".to_string()
    } else {
        format!("Hello {} {}", user_tag, message)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("welcome")
        .name_localized("de", "begrüßen")
        .description("Welcome a user")
        .description_localized("de", "Einen Nutzer begrüßen")
        .create_option(|option| {
            option
                .name("user")
                .name_localized("de", "nutzer")
                .description("The user to welcome")
                .description_localized("de", "Der zu begrüßende Nutzer")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("message")
                .name_localized("de", "nachricht")
                .description("The message to send")
                .description_localized("de", "Die versendete Nachricht")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice_localized(
                    "pizza",
                    "Welcome to our cool server! Ask me if you need help",
                    [(
                        "de",
                        "Willkommen auf unserem coolen Server! Frag mich, falls du Hilfe brauchst",
                    )],
                )
                .add_string_choice_localized(
                    "coffee",
                    "Hey, do you want a coffee?",
                    [("de", "Hey, willst du einen Kaffee?")],
                )
                .add_string_choice_localized(
                    "club",
                    "Welcome to the club, you're now a good person. Well, I hope.",
                    [(
                        "de",
                        "Willkommen im Club, du bist jetzt ein guter Mensch. Naja, hoffentlich.",
                    )],
                )
                .add_string_choice_localized(
                    "game",
                    "I hope that you brought a controller to play together!",
                    [(
                        "de",
                        "Ich hoffe du hast einen Controller zum Spielen mitgebracht!",
                    )],
                )
        })
}
