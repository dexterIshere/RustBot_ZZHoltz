use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fs;
use std::path::PathBuf;

pub fn load_insultes(static_folder: &PathBuf) -> Vec<(String, String)> {
    let full_path = static_folder.join("hum.json");
    let data = fs::read_to_string(full_path).expect("Bruh wrong path");
    let json: serde_json::Value = serde_json::from_str(&data).expect("pas formater");

    let mut insultes = Vec::new();

    if let Some(insultes_map) = json["insultes"].as_object() {
        for (key, value) in insultes_map {
            if let Some(insulte) = value.as_str() {
                insultes.push((key.clone(), insulte.to_string()));
            }
        }
    }

    insultes
}

pub async fn list(context: Context, msg: Message, static_folder: &PathBuf) {
    let insultes_list = load_insultes(static_folder);
    let insultes = format!("{:?}", insultes_list);

    if msg.content == "!list" {
        let response = MessageBuilder::new()
            .push("Voici la banque d'insulte maître: ")
            .push(insultes)
            .build();
        if let Err(why) = msg.channel_id.say(&context.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
