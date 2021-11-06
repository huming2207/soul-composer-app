use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Failed to open device, reason: {0}")]
    OpenError(String),

    #[error("Failed to close device, reason: {0}")]
    CloseError(String),

    #[error("Failed to handle packet, detail: {0}")]
    ReadError(String),

    #[error("Failed to send packet, detail: {0}")]
    WriteError(String),

    #[error("Nothing to read")]
    NothingToRead,

    #[error("CRC check failed: {0}")]
    ChecksumError(String),

    #[error("Decode error: {0}")]
    DecodeError(String),

    #[error("Encode error: {0}")]
    EncodeError(String),
}
