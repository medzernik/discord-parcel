use crate::{commands::CommandContext, delivery::delivery_payload::Deliveries};
use secrecy::ExposeSecret;

#[poise::command(slash_command)]
/// Gets all parcels currently associated with the parcels token
pub async fn get_parcels(ctx: CommandContext<'_>) -> Result<(), serenity::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.parcel.app/external/deliveries/?")
        .header(
            "api-key",
            ctx.data()
                .get_token()
                .ok_or(serenity::Error::Other(
                    "missing token for parcels! set it first using a command",
                ))?
                .expose_secret(),
        )
        .send()
        .await
        .expect("invalid request");

    let parsed_res: Deliveries =
        serde_json::from_str(&res.text().await.expect(" cannot be converted to text"))?;

    ctx.reply(format!("{parsed_res:#?}")).await?;
    Ok(())
}
