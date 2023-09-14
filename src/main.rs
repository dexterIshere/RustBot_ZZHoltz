mod api;
mod commands;
mod models;

// use std::path::PathBuf;

use std::sync::Arc;

use anyhow::anyhow;
use serenity::prelude::RwLock;
// use commands::admin::format_list::format_list;
use commands::trash_cmds::admin::return_trash_list::list;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

use crate::commands::shared::states::QuizState;
struct Handler {
    guild_id: String,
    // static_folder: PathBuf,
    database: PgPool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        list(context.clone(), msg.clone(), &self.database).await;
        // format_list(context, msg, &self.static_folder).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let content: String;

            content = match command.data.name.as_str() {
                "insulte" => commands::trash_cmds::insultes::run(&command.data.options),
                "test" => commands::poke_cmds::test::run(&command.data.options).await,
                "add_bullshit" => {
                    commands::trash_cmds::add_bullshit::run(
                        &command.data.options,
                        &command,
                        &self.database,
                    )
                    .await
                }
                "delete_bullshit" => {
                    commands::trash_cmds::admin::delete_trash::run(
                        &command.data.options,
                        &command,
                        &self.database,
                    )
                    .await
                }

                "balle_perdu" => {
                    commands::trash_cmds::fast_trash::run(&command.data.options, &self.database)
                        .await
                }

                _ => "not implemented :(".to_string(),
            };
            if command.data.name == "quiz" {
                let data = ctx.data.read().await;
                let quiz_state_lock = data
                    .get::<QuizState>()
                    .expect("Expected QuizState in TypeMap");
                let quiz_state = quiz_state_lock.read().await;

                if *quiz_state {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.content("Un quiz est déjà en cours.")
                                })
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                } else {
                    commands::poke_cmds::launch_quiz::quizz_run(
                        &command.data.options,
                        &command,
                        ctx.clone(),
                    )
                    .await;
                }
            }

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let mut quiz_data = ctx.data.write().await;
        quiz_data.insert::<QuizState>(Arc::new(RwLock::new(false)));

        let guild_id = GuildId(self.guild_id.parse().unwrap());

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    commands::trash_cmds::insultes::register(command)
                })
                .create_application_command(|command| {
                    commands::trash_cmds::fast_trash::register(command)
                })
                .create_application_command(|command| {
                    commands::trash_cmds::add_bullshit::register(command)
                })
                .create_application_command(|command| {
                    commands::trash_cmds::admin::delete_trash::register(command)
                })
                .create_application_command(|command| {
                    commands::poke_cmds::launch_quiz::register(command)
                })
                .create_application_command(|command| commands::poke_cmds::test::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );

        //  let command_ids_to_delete: Vec<u64> = vec![
        //  uncoment and add some ids here to delete them
        //  ];
        //  for command_id in command_ids_to_delete.iter() {
        //      if let Err(why) = ctx
        //          .http
        //          .delete_global_application_command(*command_id)
        //          .await
        //      {
        //          println!(
        //              "Erreur lors de la suppression de la commande globale {} : {:?}",
        //              command_id, why
        //          );
        //      } else {
        //          println!("Commande globale {} supprimée avec succès.", command_id);
        //      }
        //  }//
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgresql://postgres:{secrets.PASSWORD}@db.uoioffqyfmxniqfrsmfa.supabase.co:5432/postgres"
    )]
    pool: PgPool,
    // #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let guild_id = if let Some(guild_id) = secret_store.get("GUILD_ID") {
        guild_id
    } else {
        return Err(anyhow!("'GUILD_ID' was not found").into());
    };

    // Run the schema migration
    anyhow::Context::context(
        pool.execute(include_str!("../schema.sql")).await,
        "failed to run migrations",
    )?;

    let handler = Handler {
        database: pool,
        guild_id,
        // static_folder,
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
