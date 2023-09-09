use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serenity::json::json;
use serenity::json::Value;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
pub async fn format_list(context: Context, msg: Message, static_folder: &PathBuf) {
    let full_path = static_folder.join("hum.json");

    let data = fs::read_to_string(&full_path).expect("Bruh wrong path");
    let json: Value = serde_json::from_str(&data).expect("Erreur lors du parsing du JSON");

    let insultes_map = json["insultes"]
        .as_object()
        .expect("Le champ 'insultes' doit être un objet");

    let mut new_insultes = BTreeMap::new();

    for (key, value) in insultes_map {
        let value_str = value
            .as_str()
            .expect("Chaque insulte doit être une chaîne de caractères")
            .to_string();
        new_insultes.insert(key.clone(), value_str);
    }

    let new_json = json!({
        "insultes": new_insultes
    });

    fs::write(
        full_path,
        serde_json::to_string_pretty(&new_json).expect("Erreur lors de la sérialisation du JSON"),
    )
    .expect("Impossible d'écrire dans le fichier");

    if msg.content == "!format" {
        let response = MessageBuilder::new()
            .push("formatage en cours ... ")
            .build();
        if let Err(why) = msg.channel_id.say(&context.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
