use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, Serialize_repr, Deserialize_repr)]
pub enum PacketType {
    Ack = 0,
    DeviceInfo = 1,
    GetConfig = 2,
    SetConfig = 3,
    GetAlgoInfo = 4,
    SetAlgoMetadata = 5,
    GetFirmwareInfo = 6,
    SetFirmwareMetadata = 7,
    Ping = 8,
    SendChunk = 9,
    ChunkAck = 10,
    Nack = 0xff,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AckPacket {
    pub pkt_type: PacketType,
    pub len: u8,
    pub crc: u16,
}
