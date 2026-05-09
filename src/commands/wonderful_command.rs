use serenity::{all::ResolvedOption, builder::CreateCommand};

use crate::commands::BotCommands;

pub struct WonderfulCommand;
impl BotCommands for WonderfulCommand {
    async fn run(_options: &[ResolvedOption<'_>]) -> Result<String, Box<dyn std::error::Error>> {
        Ok("wonderful ping".to_string())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("wonderful_command").description("An amazing command")
    }
}
