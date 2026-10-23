use thiserror::Error;

#[derive(Debug, Error)]
pub enum TraceGuardError {
    #[error("Invalid event: {0}")]
    InvalidEvent(String),

    #[error("Invalid liquidity: {0}")]
    InvalidLiquidity(String),

    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
