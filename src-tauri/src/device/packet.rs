use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::error::DeviceError;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive)]
pub enum PacketType {
    Ack = 0,
    DeviceInfo = 1,
    GetConfig = 2,
    SetConfig = 3,
    GetAlgoInfo = 4,
    SetAlgoBin = 5,
    GetFirmwareInfo = 6,
    SetFirmwareBin = 7,
    Ping = 8,
    Nack = 0xff,
}

#[derive(Debug, Clone, Copy)]
pub struct PacketHeader {
    pub pkt_type: PacketType,
    pub len: u8,
    pub crc: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct AckPacket {
    pub pkt_type: PacketType,
    pub len: u8,
    pub crc: u16,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub mac_addr: [u8; 6],
    pub flash_id: [u8; 8],
    pub esp_idf_ver: String,
    pub dev_model: String,
    pub dev_build: String,
}

impl TryFrom<&[u8]> for PacketHeader {
    type Error = DeviceError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 4 {
            return Err(DeviceError::DecodeError(format!("Packet too short: {} bytes", buf.len())))
        }

        let pkt_type_num = buf[0];
        let pkt_type = match FromPrimitive::from_u8(pkt_type_num) {
            Some(PacketType::Ack) => PacketType::Ack,
            Some(PacketType::DeviceInfo) => PacketType::DeviceInfo,
            Some(PacketType::GetAlgoInfo) => PacketType::GetAlgoInfo,
            Some(PacketType::GetConfig) => PacketType::GetConfig,
            Some(PacketType::GetFirmwareInfo) => PacketType::GetFirmwareInfo,
            Some(PacketType::SetConfig) => PacketType::SetConfig,
            Some(PacketType::SetFirmwareBin) => PacketType::SetFirmwareBin,
            Some(PacketType::SetAlgoBin) => PacketType::SetAlgoBin,
            Some(PacketType::Nack) => PacketType::Nack,
            Some(PacketType::Ping) => PacketType::Ping,
            None => return Err(DeviceError::DecodeError(format!("Packet type {} not found", pkt_type_num)))
        };

        Ok(PacketHeader{ pkt_type, len: buf[1], crc: ((buf[3] as u16) << 8 | (buf[2] as u16))  })
    }
}

impl TryFrom<&[u8]> for DeviceInfo {
    type Error = DeviceError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 110 {
            return Err(DeviceError::DecodeError(format!("Packet too short: {} bytes", buf.len())))
        }

        let mut mac_addr: [u8; 6] = [0; 6];
        let mut flash_id: [u8; 8] = [0; 8];

        mac_addr[0..6].copy_from_slice(&buf[0..6]);
        flash_id[0..8].copy_from_slice(&buf[6..14]);
        let esp_idf_ver = String::from_utf8((&buf[14..46]).to_vec()).map_err(|err| DeviceError::DecodeError(err.to_string()))?;
        let dev_model = String::from_utf8((&buf[46..78]).to_vec()).map_err(|err| DeviceError::DecodeError(err.to_string()))?;
        let dev_build = String::from_utf8((&buf[78..110]).to_vec()).map_err(|err| DeviceError::DecodeError(err.to_string()))?;
        Ok(DeviceInfo { mac_addr, flash_id, esp_idf_ver, dev_model, dev_build })
    }
}