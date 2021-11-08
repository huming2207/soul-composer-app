use std::{convert::TryFrom, sync::Mutex, time::Duration};

use crc::{CRC_16_XMODEM, Crc};

use crate::device::serial_comm::SerialComm;

use super::{
    error::DeviceError,
    packet::{CdcPacket, DeviceInfo, PacketHeader, PacketType},
};

pub const CDC_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM); 

pub struct ProtocolCodec {
    cdc: Option<SerialComm>,
}

impl ProtocolCodec {
    pub fn new() -> ProtocolCodec {
        ProtocolCodec { cdc: None }
    }

    pub fn open(&mut self, port: String) -> Result<(), DeviceError> {
        self.cdc = Some(SerialComm::new(port)?);
        Ok(())
    }

    pub fn close(&mut self) {
        self.cdc = None
    }

    fn parse_header(buf: Vec<u8>) -> Result<PacketHeader, DeviceError> {
        let mut rx_buf = buf.clone();
        let rx_buf_slice = rx_buf.as_slice();
        let header: PacketHeader = PacketHeader::try_from(&rx_buf_slice[..4])?;

        // If not ACK or NACK, then do CRC check
        if header.pkt_type != PacketType::Ack && header.pkt_type != PacketType::Nack {
            // Clear CRC
            rx_buf[2] = 0;
            rx_buf[3] = 0;
            let actual_crc = CDC_CRC.checksum(&rx_buf);

            if actual_crc != header.crc {
                return Err(DeviceError::DecodeError(format!(
                    "CRC mimatched, expect {:#04x} but got {:#04x}",
                    header.crc, actual_crc
                )));
            }
        }

        Ok(header)
    }

    fn parse_device_info(buf: Vec<u8>) -> Result<CdcPacket<DeviceInfo>, DeviceError> {
        let header = ProtocolCodec::parse_header(buf.clone())?;
        if header.pkt_type != PacketType::DeviceInfo {
            return Err(DeviceError::DecodeError(format!(
                "Packet type is not DEVICE_INFO: {}",
                header.pkt_type as u8
            )));
        }

        let device_info: DeviceInfo = DeviceInfo::try_from(&buf[4..])?;
        let packet = CdcPacket {
            header,
            body: device_info,
        };

        Ok(packet)
    }

    fn parse_ack(buf: Vec<u8>) -> Result<CdcPacket<()>, DeviceError> {
        let header = ProtocolCodec::parse_header(buf.clone())?;
        if header.pkt_type != PacketType::Ack {
            return Err(DeviceError::DecodeError(format!(
                "Packet type is not ACK: {}",
                header.pkt_type as u8
            )));
        }

        let packet = CdcPacket { header, body: () };
        Ok(packet)
    }

    fn parse_get_config(
        &self,
        buf: &[u8],
        header: PacketHeader,
    ) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }

    fn parse_get_algo_info(
        &self,
        buf: &[u8],
        header: PacketHeader,
    ) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }

    fn parse_get_fw_info(
        &self,
        buf: &[u8],
        header: PacketHeader,
    ) -> Result<(Option<Vec<u8>>, String), DeviceError> {
        Ok((None, "".to_string()))
    }

    pub fn get_device_info(&self) -> Result<String, DeviceError> {
        let serial = match self.cdc.as_ref() {
            Some(serial) => serial,
            None => return Err(DeviceError::ReadError("Device not opened".to_string())),
        };

        let header = PacketHeader::new_with_body(PacketType::DeviceInfo, &[0; 0])?;
        serial.write(&header.as_bytes())?;

        let rx_buf = serial.read(Duration::from_secs(3))?;
        let result = ProtocolCodec::parse_device_info(rx_buf)?;
        Ok(serde_json::to_string(&result)
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?)
    }

    pub fn send_ping(&self) -> Result<String, DeviceError> {
        let serial = match self.cdc.as_ref() {
            Some(serial) => serial,
            None => return Err(DeviceError::ReadError("Device not opened".to_string())),
        };

        let header = PacketHeader::new_with_body(PacketType::Ping, &[0; 0])?;
        serial.write(&header.as_bytes())?;

        let rx_buf = serial.read(Duration::from_secs(1))?;
        let result = ProtocolCodec::parse_ack(rx_buf)?;
        Ok(serde_json::to_string(&result)
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?)
    }
}

impl Default for ProtocolCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct ProtoCodecState {
    codec: Mutex<ProtocolCodec>,
}

#[tauri::command]
pub async fn cdc_open(
    invoke_message: String,
    state: tauri::State<'_, ProtoCodecState>,
) -> Result<(), String> {
    println!("Port is: {}", invoke_message);
    let codec = &mut *state.codec.lock().unwrap();
    match codec.open(invoke_message) {
        Ok(ret) => return Ok(ret),
        Err(err) => return Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn cdc_close(state: tauri::State<'_, ProtoCodecState>) -> Result<(), String> {
    let codec = &mut *state.codec.lock().unwrap();
    Ok(codec.close())
}

#[tauri::command]
pub async fn cdc_get_device_info(
    state: tauri::State<'_, ProtoCodecState>,
) -> Result<String, String> {
    let codec = &*state.codec.lock().unwrap();
    match codec.get_device_info() {
        Ok(ret) => return Ok(ret),
        Err(err) => return Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn cdc_ping(state: tauri::State<'_, ProtoCodecState>) -> Result<String, String> {
    let codec = &*state.codec.lock().unwrap();
    match codec.send_ping() {
        Ok(ret) => return Ok(ret),
        Err(err) => return Err(err.to_string()),
    }
}
