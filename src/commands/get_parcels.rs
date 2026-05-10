use secrecy::ExposeSecret;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::app_state::ApplicationStateAsync;
use crate::commands::{BotCommands, CommandResult};
use crate::delivery::delivery_payload::Deliveries;

#[derive(thiserror::Error, Debug)]
pub enum GetParcelError {
    #[error("request error: {0}")]
    GenericReqwestError(#[from] reqwest::Error),

    #[error("payload error: {0}")]
    InvalidResponse(#[from] serde_json::Error),

    #[error("token not present")]
    TokenNotPresent,
}

pub struct GetParcelsCommand;

impl BotCommands for GetParcelsCommand {
    async fn run(
        app_config: ApplicationStateAsync,
        _options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.parcel.app/external/deliveries/?")
            .header(
                "api-key",
                app_config
                    .0
                    .read()
                    .await
                    .get_token()
                    .ok_or(GetParcelError::TokenNotPresent)?
                    .expose_secret(),
            )
            .send()
            .await?;
        let parsed_res: Deliveries = serde_json::from_str(&res.text().await?)?;
        Ok(res.text().await?)
    }

    fn register() -> CreateCommand {
        CreateCommand::new("get_parcel").description("Get a list of parcels")
    }

    // fn construct_reply() -> serenity::all::CreateInteractionResponse {}
}
