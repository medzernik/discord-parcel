use secrecy::SecretString;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::RATE_LIMIT_HOUR_ABSOLUTE;

const DEFAULT_CHECK_PER_HOUR: u8 = 2;

/// Main application state/config
#[derive(Debug)]
pub struct ApplicationState {
    /// Currently available requests
    current_requests: Arc<RwLock<u8>>,
    /// The token for interacting with parcel API.
    // TODO: Make the token configurable via the Discord UI
    parcel_token: SecretString,
    autocheck_per_hour: u8,
}

impl ApplicationState {
    pub fn new() -> Self {
        let parcel_token =
            std::env::var("PARCEL_TOKEN").expect("expected token for parcel: PARCEL_TOKEN");

        Self {
            current_requests: Arc::new(RwLock::from(
                RATE_LIMIT_HOUR_ABSOLUTE - DEFAULT_CHECK_PER_HOUR,
            )),
            parcel_token: parcel_token.into(),
            autocheck_per_hour: DEFAULT_CHECK_PER_HOUR,
        }
    }
}
