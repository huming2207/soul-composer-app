use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareMetadata {
    len: u32,
    crc: u32,
    name: String,
}


#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashAlgoMetadata {
    len: u32,
    crc: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChunk {
    len: u8,
    buf: Vec<u8>
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromPrimitive, Deserialize_repr, Serialize_repr)]
pub enum ChunkState {
    Done = 0,
    Next = 1,
    CrcFail = 2,
    UnexpectedError = 3,
}

impl Into<u8> for ChunkState {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for ChunkState {
    fn from(byte: u8) -> Self {
        let state: ChunkState = match FromPrimitive::from_u8(byte) {
            Some(ChunkState::Done) => ChunkState::Done,
            Some(ChunkState::Next) => ChunkState::Next,
            Some(ChunkState::CrcFail) => ChunkState::CrcFail,
            Some(ChunkState::UnexpectedError) => ChunkState::UnexpectedError,
            None => ChunkState::UnexpectedError,
        };

        state
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkAckPkt {
    state: ChunkState,
    aux: u32,
}