use serenity::{all::ResolvedOption, builder::CreateCommand};

use crate::{
    app_state::ApplicationStateAsync,
    commands::{BotCommands, CommandResult},
};

pub struct WonderfulCommand;
impl BotCommands for WonderfulCommand {
    async fn run(
        app_config: ApplicationStateAsync,
        _options: &[ResolvedOption<'_>],
    ) -> CommandResult {
        Ok("wonderful ping".to_string())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("wonderful_command").description("An amazing command")
    }
}
