use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use sqlx::PgPool;

use crate::db::sentences::trash_lister;

pub async fn list(context: Context, msg: Message, pool: &PgPool) {
    let insultes_list = trash_lister(pool).await;
    let insultes = format!("{:?}", insultes_list);

    if msg.content == "!list" {
        let response = MessageBuilder::new()
            .push("Voici la banque d'insultes maître: ")
            .push(insultes)
            .build();
        if let Err(why) = msg.channel_id.say(&context.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
