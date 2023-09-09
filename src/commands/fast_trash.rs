use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use std::fs;
use std::path::PathBuf;

use rand::Rng;

pub fn load_insultes(static_folder: &PathBuf) -> Vec<String> {
    let full_path = static_folder.join("hum.json");
    let data = fs::read_to_string(full_path).expect("Bruh wrong path");
    let json: serde_json::Value = serde_json::from_str(&data).expect("pas formater");

    let mut insultes = Vec::new();

    if let Some(insultes_map) = json["insultes"].as_object() {
        for (_key, value) in insultes_map {
            if let Some(insulte) = value.as_str() {
                insultes.push(insulte.to_string());
            }
        }
    }

    insultes
}

pub fn run(options: &[CommandDataOption], static_folder: &PathBuf) -> String {
    let insultes = load_insultes(static_folder);
    let mut rng = rand::thread_rng();
    let insulte = insultes[rng.gen_range(0..insultes.len())].clone();

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
