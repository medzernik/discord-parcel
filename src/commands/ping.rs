use crate::commands::CommandContext;

#[poise::command(slash_command)]
/// Pongs back at you!
async fn ping(ctx: CommandContext<'_>) -> Result<(), serenity::Error> {
    ctx.reply("pong").await?;
    Ok(())
}
