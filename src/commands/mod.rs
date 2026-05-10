use crate::app_state::ApplicationState;

pub mod get_parcels;
pub mod ping;

type CommandContext<'a> = poise::Context<'a, ApplicationState, serenity::Error>;
