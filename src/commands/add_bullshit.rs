use std::fs;
use std::path::PathBuf;

use serenity::builder::CreateApplicationCommand;
use serenity::json::Value;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::utils::MessageBuilder;

pub fn run(
    options: &[CommandDataOption],
    static_folder: &PathBuf,
    command: &ApplicationCommandInteraction,
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

    let full_path = static_folder.join("hum.json");

    let data = fs::read_to_string(&full_path).expect("Bruh wrong path");

    let mut parsed_data: Value =
        serde_json::from_str(&data).expect("Erreur lors du parsing du JSON");

    if let Some(array) = parsed_data["insultes"].as_array_mut() {
        array.push(Value::String(new_trash.clone()));
    }

    fs::write(
        &full_path,
        serde_json::to_string(&parsed_data).expect("Erreur lors de la sérialisation du JSON"),
    )
    .expect("Impossible d'écrire dans le fichier");

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
