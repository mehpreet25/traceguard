use thiserror::Error;

#[derive(Debug, Error)]
pub enum TraceGuardError {
    #[error("Invalid event: {0}")]
    InvalidEvent(String),
}
