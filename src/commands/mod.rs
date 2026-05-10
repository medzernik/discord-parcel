use std::error::Error;

use serenity::all::{CreateCommand, ResolvedOption};

pub mod get_parcels;
pub mod wonderful_command;

pub trait BotCommands {
    async fn run(_options: &[ResolvedOption<'_>]) -> Result<String, Box<dyn Error>>;
    fn register() -> CreateCommand;
    // fn construct_reply() -> CreateInteractionResponse;
}
