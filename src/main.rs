mod app_state;
mod commands;
mod delivery;

use std::env;
use std::sync::OnceLock;

use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::BotCommands;

// Statics

/// The actual rate limit that can be called per hour to the cached API of Parcel
pub const RATE_LIMIT_HOUR_ABSOLUTE: u8 = 20;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {}", command.data.name);

            let content = match command.data.name.as_str() {
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

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
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
async fn main() {
    dotenv::dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Expected a token for Discord: DISCORD_TOKEN");

    // Build our client.
    let intents = GatewayIntents::DIRECT_MESSAGES;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
