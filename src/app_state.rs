use secrecy::SecretString;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// The actual rate limit that can be called per hour to the cached API of Parcel
const RATE_LIMIT_HOUR_ABSOLUTE: u8 = 20;
const DEFAULT_CHECK_PER_HOUR: u8 = 2;

/// Main type to initialize
#[derive(Debug)]
pub struct ApplicationStateAsync(Arc<RwLock<ApplicationState>>);
impl ApplicationStateAsync {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::from(ApplicationState::new())))
    }
}

#[derive(Debug, Error)]
pub enum ApplicationStateError {
    #[error("autocheck cannot be larger than {RATE_LIMIT_HOUR_ABSOLUTE}")]
    AutocheckLargerThanMax,

    #[error("Reached the rate limit of {0}")]
    Ratelimited(u8),
}

/// Application state/config
#[derive(Debug)]
pub struct ApplicationState {
    /// Currently available requests
    current_requests: u8,
    /// The token for interacting with parcel API.
    // TODO: Make the token configurable via the Discord UI
    parcel_token: Option<SecretString>,
    autocheck_per_hour: u8,
}

impl ApplicationState {
    pub fn new() -> Self {
        let parcel_token = match std::env::var("PARCEL_TOKEN").ok() {
            Some(token) => Some(token.into()),
            None => None,
        };
        Self {
            current_requests: RATE_LIMIT_HOUR_ABSOLUTE - DEFAULT_CHECK_PER_HOUR,
            parcel_token,
            autocheck_per_hour: DEFAULT_CHECK_PER_HOUR,
        }
    }

    pub fn set_token(&mut self, token: String) -> Result<(), ApplicationStateError> {
        Ok(self.parcel_token = Some(token.into()))
    }

    pub fn get_token(&self) -> Option<&SecretString> {
        self.parcel_token.as_ref()
    }
}
