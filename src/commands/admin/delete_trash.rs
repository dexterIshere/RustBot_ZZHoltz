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
    let delete_option = options
        .get(0)
        .expect("chiffre invalide")
        .resolved
        .as_ref()
        .expect("chiffre invalide");

    let delete_key = match delete_option {
        CommandDataOptionValue::Integer(int) => int.to_string(),
        _ => return "Type de donnée invalide".to_string(),
    };

    let full_path = static_folder.join("hum.json");

    let data = fs::read_to_string(&full_path).expect("Bruh wrong path");

    let mut parsed_data: Value =
        serde_json::from_str(&data).expect("Erreur lors du parsing du JSON");

    let mut deleted = "Non supprimé".to_string();

    if let Some(insultes_map) = parsed_data["insultes"].as_object_mut() {
        if insultes_map.remove(&delete_key).is_some() {
            deleted = delete_key;
        }
    }

    fs::write(
        &full_path,
        serde_json::to_string(&parsed_data).expect("Erreur lors de la sérialisation du JSON"),
    )
    .expect("Impossible d'écrire dans le fichier");

    fs::write(
        &full_path,
        serde_json::to_string(&parsed_data).expect("Erreur lors de la sérialisation du JSON"),
    )
    .expect("Impossible d'écrire dans le fichier");

    let response = MessageBuilder::new()
        .push_bold_safe(&command.user.name)
        .push(" à suprimé l'insulte n° ")
        .push(&deleted)
        .push(" de banque d'insultes")
        .build();

    format!("{}", response)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("delete_bullshit")
        .description("suprime une balle perdu de la liste")
        .create_option(|option| {
            option
                .name("numéro")
                .description("le numéro de la phrase à suprimer")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}
