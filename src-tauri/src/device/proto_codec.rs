use std::{cmp, convert::TryFrom, fs, path::Path, sync::Mutex, time::Duration};

use crate::{device::serial_comm::SerialComm, prog::arm::flash_stub_gen::ArmFlashStub};

use super::{
    error::DeviceError,
    packet::{
        device_cfg::DeviceConfig,
        device_info::DeviceInfo,
        file_chunk::{BlobChunk, ChunkAckPkt, ChunkState, FlashAlgoMetadata, FLASH_ALGO_MAX_LEN},
        misc::PacketType,
        pkt_header::{CdcPacket, PacketHeader},
        BLOB_CRC, CDC_CRC,
    },
};

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

    fn parse_header(buf: &[u8]) -> Result<PacketHeader, DeviceError> {
        let mut rx_buf = buf.to_vec();
        let header: PacketHeader = PacketHeader::try_from(&rx_buf.clone()[..4])?;

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

    fn parse_device_info(buf: &[u8]) -> Result<CdcPacket<DeviceInfo>, DeviceError> {
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

    fn parse_ack(buf: &[u8]) -> Result<CdcPacket<()>, DeviceError> {
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

    fn parse_chunk_ack(buf: &[u8]) -> Result<CdcPacket<ChunkAckPkt>, DeviceError> {
        let header = ProtocolCodec::parse_header(buf)?;
        if header.pkt_type != PacketType::ChunkAck {
            return Err(DeviceError::DecodeError(format!(
                "Packet type is not ChunkACK: {}",
                header.pkt_type as u8
            )));
        }

        let body = ChunkAckPkt::try_from(&buf[4..])?;
        let packet = CdcPacket { header, body };
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
        let result = ProtocolCodec::parse_device_info(&rx_buf)?;
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
        let result = ProtocolCodec::parse_ack(&rx_buf)?;
        Ok(serde_json::to_string(&result)
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?)
    }

    pub fn send_config(
        &self,
        algo_path: String,
        name: String,
        default: bool,
        ram_size: u32,
    ) -> Result<String, DeviceError> {
        let serial = match self.cdc.as_ref() {
            Some(serial) => serial,
            None => return Err(DeviceError::ReadError("Device not opened".to_string())),
        };

        let buf = fs::read(Path::new(&algo_path))?;
        let algo = ArmFlashStub::from_elf(&buf, name, default, ram_size)?;

        let cfg: DeviceConfig = algo.into();
        let cfg_buf = cfg.as_packet_bytes()?;
        serial.write(&cfg_buf)?;

        let rx_buf = serial.read(Duration::from_secs(1))?;
        let result = ProtocolCodec::parse_ack(&rx_buf)?;
        Ok(serde_json::to_string(&result)
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?)
    }

    pub fn send_flash_algo(
        &self,
        algo_path: String,
        name: String,
        default: bool,
        ram_size: u32,
    ) -> Result<String, DeviceError> {
        let serial = match self.cdc.as_ref() {
            Some(serial) => serial,
            None => return Err(DeviceError::ReadError("Device not opened".to_string())),
        };

        let buf = fs::read(Path::new(&algo_path))?;
        let algo = ArmFlashStub::from_elf(&buf, name, default, ram_size)?;

        let crc = BLOB_CRC.checksum(&algo.instructions);
        if algo.instructions.len() > FLASH_ALGO_MAX_LEN {
            return Err(DeviceError::BlobTooLong(algo.instructions.len()));
        }

        let len = algo.instructions.len() as u32;
        let metadata = FlashAlgoMetadata { crc, len };
        let metadata_buf = metadata.as_packet_bytes()?;
        serial.write(&metadata_buf)?;

        self.send_blob_chunk(&algo.instructions, serial)?;

        Ok(
            serde_json::to_string(&algo)
                .map_err(|err| DeviceError::DecodeError(err.to_string()))?,
        )
    }

    fn send_blob_chunk(&self, buf: &[u8], serial: &SerialComm) -> Result<(), DeviceError> {
        let first_rx_buf = serial.read(Duration::from_secs(1))?;
        let first_ack = ProtocolCodec::parse_chunk_ack(&first_rx_buf)?;

        match first_ack.body.state {
            ChunkState::Done => return Ok(()),
            ChunkState::Next => {
                let new_offset = first_ack.body.aux;
                if new_offset as usize >= buf.len() {
                    return Err(DeviceError::WriteError(format!(
                        "Unexpected offset for blob ACK, max length is {} while new offset is {}",
                        buf.len(),
                        new_offset
                    )));
                }
            }
            ChunkState::CrcFail => {
                return Err(DeviceError::ChecksumError(format!(
                    "Checksum failed, expected {:#04x}",
                    first_ack.body.aux
                )));
            }
            ChunkState::UnexpectedError => {
                return Err(DeviceError::WriteError(format!(
                    "Unexpected error occured when sending blob chunk: {:#04x}",
                    first_ack.body.aux
                )));
            }
        }

        let mut offset: usize = 0;
        let mut remaining: usize = buf.len();
        while remaining > 0 {
            let chunk_len = cmp::min(u8::MAX as usize, remaining);
            let chunk_buf = &buf[offset..(offset + chunk_len)];
            let blob_chunk = BlobChunk {
                len: chunk_len as u8,
                buf: chunk_buf.to_vec(),
            };
            let blob_chunk_pkt = blob_chunk.as_bytes();
            serial.write(&blob_chunk_pkt);

            let rx_buf = serial.read(Duration::from_secs(1))?;
            let ack = ProtocolCodec::parse_chunk_ack(&rx_buf)?;

            match ack.body.state {
                ChunkState::Done => return Ok(()),
                ChunkState::Next => {
                    let new_offset = ack.body.aux;
                    if new_offset as usize >= buf.len() {
                        return Err(DeviceError::WriteError(format!("Unexpected offset for blob ACK, max length is {} while new offset is {}", buf.len(), new_offset)));
                    } else {
                        remaining -= (new_offset as usize) - offset;
                        offset = new_offset as usize;
                    }
                }
                ChunkState::CrcFail => {
                    return Err(DeviceError::ChecksumError(format!(
                        "Checksum failed, expected {:#04x}",
                        ack.body.aux
                    )));
                }
                ChunkState::UnexpectedError => {
                    return Err(DeviceError::WriteError(format!(
                        "Unexpected error occured when sending blob chunk: {:#04x}",
                        ack.body.aux
                    )));
                }
            }
        }

        Ok(())
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
