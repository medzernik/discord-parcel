use crate::app_state::ApplicationStateAsync;

use serenity::all::{CreateCommand, ResolvedOption, standard::CommandResult};

pub mod get_parcels;
pub mod wonderful_command;

// pub type CommandResult<T: std::fmt::Debug> = Result<T, Box<dyn std::error::Error>>;

pub trait BotCommands {
    async fn run<T>(
        app_config: ApplicationStateAsync,
        _options: &[ResolvedOption<'_>],
    ) -> serenity::Result<T>;
    fn register() -> CreateCommand;
    // fn construct_reply() -> CreateInteractionResponse;
}
