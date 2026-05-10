mod app_state;
mod commands;
mod delivery;

use std::env;

use debug_print::debug_println;
use secrecy::{ExposeSecret, SecretString};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::app_state::ApplicationStateAsync;
use crate::commands::BotCommands;

// Statics

#[async_trait]
impl EventHandler for ApplicationStateAsync {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {}", command.data.name);

            let command_exec_result = match command.data.name.as_str() {
                "get_parcel" => Some(
                    commands::get_parcels::GetParcelsCommand::run(&command.data.options())
                        .await
                        .expect("critical unhandled error in GetParcelsCommand"),
                ),
                "wonderful_command" => Some(
                    commands::wonderful_command::WonderfulCommand::run(&command.data.options())
                        .await
                        .expect("critical unhandled error in WonderfulCommand"),
                ),
                _ => Some("not implemented".to_string()),
            };

            if let Some(content) = command_exec_result {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(err) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {err}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let global_command = Command::create_global_command(
            &ctx.http,
            commands::wonderful_command::WonderfulCommand::register(),
        )
        .await;

        let cmd_get_parcels = Command::create_global_command(
            &ctx.http,
            commands::get_parcels::GetParcelsCommand::register(),
        )
        .await;

        println!("I created the following global slash command: {global_command:#?}");
        println!("I created the following global slash command: {cmd_get_parcels:#?}");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    // initialize the application state
    let application_state = ApplicationStateAsync::new();
    debug_println!("{application_state:#?}");

    // configure the client with your Discord bot token in the environment.
    // contrary to the parcels token, this has to be done before the bot runs, as without this token the bot has no way of running
    let discord_token: SecretString = env::var("DISCORD_TOKEN")?.into();

    // build our client.
    let intents = GatewayIntents::DIRECT_MESSAGES;
    let mut client = Client::builder(discord_token.expose_secret(), intents)
        .event_handler(application_state)
        .await?;

    client.start().await?;
    Ok(())
}
