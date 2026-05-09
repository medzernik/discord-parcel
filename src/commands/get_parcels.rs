use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::commands::BotCommands;

#[derive(thiserror::Error, Debug)]
pub enum GetParcelError {
    #[error("request error: {0}")]
    Disconnect(#[from] reqwest::Error),
}

pub struct GetParcelsCommand;

impl BotCommands for GetParcelsCommand {
    async fn run(_options: &[ResolvedOption<'_>]) -> Result<String, Box<dyn std::error::Error>> {
        Ok(reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?)
    }

    fn register() -> CreateCommand {
        CreateCommand::new("get_parcel").description("Get a list of parcels")
    }
}
