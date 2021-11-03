use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Failed to open device, reason: {0}")]
    DeviceOpenError(String),

    #[error("Failed to handle packet, detail: {0}")]
    ReadError(String),
}
