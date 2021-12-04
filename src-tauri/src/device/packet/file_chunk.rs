use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::device::error::DeviceError;

use super::{misc::PacketType, pkt_header::PacketHeader, slice_to_le_u32};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareMetadata {
    crc: u32,
    len: u32,
    name: String,
}

impl FirmwareMetadata {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
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
    pub crc: u32,
    pub len: u32,
}

impl FlashAlgoMetadata {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&self.crc.to_le_bytes());
        buf.extend_from_slice(&self.len.to_le_bytes());

        buf
    }

    pub fn as_packet_bytes(&self) -> Result<Vec<u8>, DeviceError> {
        let body = self.as_bytes();
        let header = PacketHeader::new_with_body(PacketType::SetAlgoMetadata, &body)?;
        Ok(header.as_packet(&body))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobChunk {
    pub len: u8,
    pub buf: Vec<u8>,
}

impl BlobChunk {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(&self.len.to_le_bytes());

        let mut trunc_buf = self.buf.clone();
        trunc_buf.truncate(self.len.into());

        buf.extend_from_slice(&trunc_buf);
        buf
    }

    pub fn as_packet_bytes(&self) -> Result<Vec<u8>, DeviceError> {
        let body = self.as_bytes();
        let header = PacketHeader::new_with_body(PacketType::BlobChunk, &body)?;
        Ok(header.as_packet(&body))
    }
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
    pub state: ChunkState,
    pub aux: u32,
}

impl TryFrom<&[u8]> for ChunkAckPkt {
    type Error = DeviceError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() != 5 {
            return Err(DeviceError::DecodeError(format!(
                "ChunkACK packet expected in 5 bytes, got {0} bytes",
                buf.len()
            )));
        }

        let state: ChunkState = buf[0].into();
        let aux = slice_to_le_u32(&buf[1..5]);

        Ok(ChunkAckPkt { state, aux })
    }
}
