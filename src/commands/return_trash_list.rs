use std::fs;
use std::path::PathBuf;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn load_insultes(static_folder: &PathBuf) -> Vec<String> {
    let full_path = static_folder.join("hum.json");
    let data = fs::read_to_string(full_path).expect("Bruh wrong path");
    let json: serde_json::Value = serde_json::from_str(&data).expect("pas formater");
    json["insultes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect()
}

pub fn run(_options: &[CommandDataOption], static_folder: &PathBuf) -> String {
    let insultes = load_insultes(static_folder);
    format!("{:#?}", insultes)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("trash_list")
        .description("vérifie la list des trashs")
}
