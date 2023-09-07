use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use std::fs;

use rand::Rng;

fn load_insultes() -> Vec<String> {
    let data = fs::read_to_string("src/hum.json").expect("Bruh wrong path");
    let json: serde_json::Value = serde_json::from_str(&data).expect("pas formater");
    json["insultes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect()
}

pub fn run(options: &[CommandDataOption]) -> String {
    let insultes = load_insultes();
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
