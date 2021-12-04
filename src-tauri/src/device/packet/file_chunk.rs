use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareMetadata {
    crc: u32,
    len: u32,
    name: String,
}

impl FirmwareMetadata {
    pub fn as_bytes(&self) -> Vec<u8> {
        let buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&self.crc.to_le_bytes());
        buf.extend_from_slice(&self.len.to_le_bytes());
        let mut short_name = self.name.clone();
        short_name.truncate(31);
        buf.extend_from_slice(short_name.as_bytes());

        buf
    }
}

pub const FLASH_ALGO_MAX_LEN: usize = 65536;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashAlgoMetadata {
    crc: u32,
    len: u32,
}

impl FlashAlgoMetadata {
    pub fn as_bytes(&self) -> Vec<u8> {
        let buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&self.crc.to_le_bytes());
        buf.extend_from_slice(&self.len.to_le_bytes());

        buf
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChunk {
    len: u8,
    buf: Vec<u8>,
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
