use std::io;

use thiserror::Error;

use crate::prog::arm::arm_error::ArmError;

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

    #[error("Timeout on serial comm")]
    TimeoutError,

    #[error("Encode error: {0}")]
    EncodeError(String),

    #[error("Message/blob too long: {0} bytes")]
    BlobTooLong(usize),

    #[error(transparent)]
    ArmFlashStubError(#[from] ArmError),

    #[error(transparent)]
    FileIoError(#[from] io::Error),
}
