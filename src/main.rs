use log::*;
use poise::{FrameworkError, serenity_prelude as serenity};

use crate::app_state::ApplicationState;

mod app_state;
mod commands;
mod delivery;

type DiscordError = Box<dyn std::error::Error + Send + Sync>;
// type AppContext<'a> = poise::Context<'a, ApplicationState, DiscordError>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::get_parcels::get_parcels()],
            on_error: |error: FrameworkError<'_, _, DiscordError>| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::ArgumentParse { error, .. } => {
                            if let Some(error) = error.downcast_ref::<serenity::RoleParseError>() {
                                warn!("Found a RoleParseError: {:?}", error);
                            } else {
                                warn!("Not a RoleParseError :(");
                            }
                        }
                        other => poise::builtins::on_error(other)
                            .await
                            .expect("unexpected error occured"),
                    }
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(ApplicationState::new())
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN")?;
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    Ok(client?.start().await?)
}
