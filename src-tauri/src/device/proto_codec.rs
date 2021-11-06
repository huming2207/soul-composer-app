use std::convert::TryFrom;

use crc::{CRC_16_KERMIT, Crc};

use crate::device::serial_comm::SerialComm;

use super::{error::DeviceError, packet::{CdcPacket, DeviceInfo, PacketHeader, PacketType}};

pub const CDC_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_KERMIT); // Kermit seems to be CCITT on ESP32??

pub struct ProtocolCodec {
    cdc: Option<SerialComm>,
}

impl ProtocolCodec {
    pub fn parse(&self) -> Result<String, DeviceError> {
        let serial = match self.cdc.as_ref() {
            Some(serial) => serial,
            None => return Err(DeviceError::ReadError("Device not opened".to_string())),
        };

        let mut rx_buf = serial.read()?;
        let rx_buf_slice = rx_buf.as_slice();
        let header: PacketHeader = PacketHeader::try_from(&rx_buf_slice[..4])?;

        // If not ACK or NACK, then do CRC check
        if header.pkt_type != PacketType::Ack && header.pkt_type != PacketType::Nack {
            // Clear CRC
            rx_buf[2] = 0;
            rx_buf[3] = 0;
            let actual_crc = CDC_CRC.checksum(&rx_buf);

            if actual_crc != header.crc {
                return Err(DeviceError::DecodeError(format!("CRC mimatched, expect {:#04x} but got {:#04x}", header.crc, actual_crc)));
            }
        }

        let parsed = match header.pkt_type {
            PacketType::DeviceInfo => self.parse_device_info(rx_buf.as_slice(), header)?,
            PacketType::GetConfig => self.parse_get_config(rx_buf.as_slice(), header)?,
            PacketType::GetAlgoInfo => self.parse_get_algo_info(rx_buf.as_slice(), header)?,
            PacketType::GetFirmwareInfo => self.parse_get_fw_info(rx_buf.as_slice(), header)?,
            PacketType::Ping => self.parse_ping(rx_buf.as_slice(), header)?,
            _ => (None, "".to_string())
        };

        Ok(parsed.1)
    }

    fn parse_device_info(&self, buf: &[u8], header: PacketHeader) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        let device_info: DeviceInfo = DeviceInfo::try_from(&buf[4..])?;
        let packet = CdcPacket{ header, body: device_info };
        let json_str = serde_json::to_string(&packet).map_err(|err| DeviceError::EncodeError(err.to_string()))?;
        Ok((None, json_str))
    }

    fn parse_get_config(&self, buf: &[u8], header: PacketHeader) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }

    fn parse_get_algo_info(&self, buf: &[u8], header: PacketHeader) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }

    fn parse_get_fw_info(&self, buf: &[u8], header: PacketHeader) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }

    fn parse_ping(&self, buf: &[u8], header: PacketHeader) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }
}