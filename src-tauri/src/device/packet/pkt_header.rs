use std::convert::TryFrom;

use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::device::error::DeviceError;

use super::{misc::PacketType, CDC_CRC};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PacketHeader {
    pub pkt_type: PacketType,
    pub len: u8,
    pub crc: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(bound = "T: Serialize, for<'de2> T: Deserialize<'de2>")]
pub struct CdcPacket<T>
where
    T: Serialize,
    for<'de2> T: Deserialize<'de2>,
{
    pub header: PacketHeader,
    pub body: T,
}

impl TryFrom<&[u8]> for PacketHeader {
    type Error = DeviceError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 4 {
            return Err(DeviceError::DecodeError(format!(
                "Packet too short: {} bytes",
                buf.len()
            )));
        }

        let pkt_type_num = buf[0];
        let pkt_type = match FromPrimitive::from_u8(pkt_type_num) {
            Some(PacketType::Ack) => PacketType::Ack,
            Some(PacketType::DeviceInfo) => PacketType::DeviceInfo,
            Some(PacketType::GetAlgoInfo) => PacketType::GetAlgoInfo,
            Some(PacketType::GetConfig) => PacketType::GetConfig,
            Some(PacketType::GetFirmwareInfo) => PacketType::GetFirmwareInfo,
            Some(PacketType::SetConfig) => PacketType::SetConfig,
            Some(PacketType::SetFirmwareMetadata) => PacketType::SetFirmwareMetadata,
            Some(PacketType::SetAlgoMetadata) => PacketType::SetAlgoMetadata,
            Some(PacketType::Nack) => PacketType::Nack,
            Some(PacketType::Ping) => PacketType::Ping,
            Some(PacketType::BlobChunk) => PacketType::BlobChunk,
            Some(PacketType::ChunkAck) => PacketType::ChunkAck,
            None => {
                return Err(DeviceError::DecodeError(format!(
                    "Packet type {} not found",
                    pkt_type_num
                )))
            }
        };

        Ok(PacketHeader {
            pkt_type,
            len: buf[1],
            crc: ((buf[3] as u16) << 8 | (buf[2] as u16)),
        })
    }
}

impl PacketHeader {
    pub fn as_bytes(&self) -> [u8; 4] {
        let mut buf: [u8; 4] = [0; 4];
        buf[0] = self.pkt_type as u8;
        buf[1] = self.len;
        buf[2] = (self.crc & 0xff) as u8;
        buf[3] = ((self.crc >> 8) & 0xff) as u8;

        buf
    }

    pub fn new_with_body(pkt_type: PacketType, body: &[u8]) -> Result<PacketHeader, DeviceError> {
        if body.len() > u8::MAX.into() {
            return Err(DeviceError::EncodeError(format!(
                "Packet too long, need to chunk: {}",
                body.len()
            )));
        }

        let mut header = PacketHeader {
            crc: 0,
            len: body.len() as u8,
            pkt_type,
        };

        let header_buf = header.as_bytes();
        let mut hasher = CDC_CRC.digest();
        hasher.update(&header_buf);
        if !body.is_empty() {
            hasher.update(body);
        }

        header.crc = hasher.finalize();

        Ok(header)
    }

    pub fn as_packet(&self, body: &[u8]) -> Vec<u8> {
        let mut buf = Vec::from(self.as_bytes());
        buf.extend_from_slice(body);
        buf
    }
}
