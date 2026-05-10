use std::{error::Error, sync::Arc};

use serenity::all::{CreateCommand, ResolvedOption};
use tokio::sync::RwLock;

use crate::app_state::ApplicationStateAsync;

pub mod get_parcels;
pub mod wonderful_command;

pub trait BotCommands {
    async fn run(_options: &[ResolvedOption<'_>]) -> Result<String, Box<dyn Error>>;
    fn register() -> CreateCommand;
    // fn construct_reply() -> CreateInteractionResponse;
}
