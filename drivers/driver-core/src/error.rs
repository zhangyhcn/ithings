use thiserror::Error;

#[derive(Debug, Error)]
pub enum DriverError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Communication error: {0}")]
    Communication(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Data error: {0}")]
    Data(String),

    #[error("Device error: {0}")]
    Device(String),

    #[error("Tag error: {0}")]
    Tag(String),

    #[error("ZMQ error: {0}")]
    Zmq(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, DriverError>;
