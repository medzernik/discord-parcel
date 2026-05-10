use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::PARCEL_TOKEN;
use crate::commands::BotCommands;
use crate::delivery::delivery_payload::Deliveries;

#[derive(thiserror::Error, Debug)]
pub enum GetParcelError {
    #[error("request error: {0}")]
    Disconnect(#[from] reqwest::Error),

    #[error("payload error: {0}")]
    InvalidResponse(#[from] serde_json::Error),
}

pub struct GetParcelsCommand;

impl BotCommands for GetParcelsCommand {
    async fn run(_options: &[ResolvedOption<'_>]) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.parcel.app/external/deliveries/?")
            .header("api-key", PARCEL_TOKEN.get().unwrap())
            .send()
            .await?;
        let parsed_res = Deliveries::from(res.text().await.expect("cannot convert to text"));
        Ok(res.text().await?)
    }

    fn register() -> CreateCommand {
        CreateCommand::new("get_parcel").description("Get a list of parcels")
    }

    // fn construct_reply() -> serenity::all::CreateInteractionResponse {}
}
