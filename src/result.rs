use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum MonarchError {
    #[error("TCP Server Error: {0}")]
    TcpServerError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("Parsing error: {0}")]
    ParseError(#[from] ParseIntError),
}

pub type Result<T> = std::result::Result<T, MonarchError>;
