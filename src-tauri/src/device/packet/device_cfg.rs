use std::convert::{TryFrom, TryInto};

use crate::device::error::DeviceError;

pub const DEV_CFG_PKT_MAGIC: u32 = 0x4a485349;

pub struct DeviceConfig {
    magic: u32,
    pc_init: u32,
    pc_uninit: u32,
    pc_program_page: u32,
    pc_erase_sector: u32,
    pc_erase_all: u32,
    data_section_offset: u32,
    flash_start_addr: u32,
    flash_end_addr: u32,
    flash_page_size: u32,
    erased_byte: u32,
    flash_sector_size: u32,
    program_timeout: u32,
    erase_timeout: u32,
    ram_size: u32,
    flash_size: u32,
    name: String,
    target: String,
}

pub struct FlashAlgoInfo {
    algo_crc: u32,
    algo_len: u32,
}

impl DeviceConfig {
    #[inline]
    fn to_le_u32(buf: &[u8]) -> u32 {
        let arr: [u8; 4] = match buf.try_into() {
            Ok(arr) => arr,
            Err(_) => [0, 0, 0, 0],
        };

        u32::from_le_bytes(arr)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        let magic = DEV_CFG_PKT_MAGIC;
        buf.extend_from_slice(&magic.to_le_bytes()); // 0..4
        buf.extend_from_slice(&self.pc_init.to_le_bytes()); // 4..8
        buf.extend_from_slice(&self.pc_uninit.to_le_bytes()); // 8..12
        buf.extend_from_slice(&self.pc_program_page.to_le_bytes()); // 12..16
        buf.extend_from_slice(&self.pc_erase_sector.to_le_bytes()); // 16..20
        buf.extend_from_slice(&self.pc_erase_all.to_le_bytes()); // 20..24
        buf.extend_from_slice(&self.data_section_offset.to_le_bytes()); // 24..28
        buf.extend_from_slice(&self.flash_start_addr.to_le_bytes()); // 28..32
        buf.extend_from_slice(&self.flash_end_addr.to_le_bytes()); // 32..36
        buf.extend_from_slice(&self.flash_page_size.to_le_bytes()); // 36..40
        buf.extend_from_slice(&self.erased_byte.to_le_bytes()); // 40..44
        buf.extend_from_slice(&self.flash_sector_size.to_le_bytes()); // 44..48
        buf.extend_from_slice(&self.program_timeout.to_le_bytes()); // 48..52
        buf.extend_from_slice(&self.erase_timeout.to_le_bytes()); // 52..56
        buf.extend_from_slice(&self.ram_size.to_le_bytes()); // 56..60
        buf.extend_from_slice(&self.flash_size.to_le_bytes()); // 60..64

        let mut name_trunc = self.name.clone();
        name_trunc.truncate(32);
        let mut name_bytes: [u8; 32] = [0; 32];
        name_bytes.copy_from_slice(name_trunc.as_bytes());
        buf.extend_from_slice(&name_bytes); // 64..96

        let mut target_trunc = self.target.clone();
        target_trunc.truncate(32);
        let mut target_bytes: [u8; 32] = [0; 32];
        target_bytes.copy_from_slice(target_trunc.as_bytes());
        buf.extend_from_slice(&target_bytes); // 96..128
        
        buf
    }
}

impl TryFrom<&[u8]> for DeviceConfig {
    type Error = DeviceError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let magic = DeviceConfig::to_le_u32(&buf[0..4]);

        if magic != DEV_CFG_PKT_MAGIC {
            return Err(DeviceError::DecodeError(format!(
                "Device Config magic unmatched, got {:#x} but wanted 0x4a485349",
                magic
            )));
        }

        let pc_init = DeviceConfig::to_le_u32(&buf[4..8]);
        let pc_uninit = DeviceConfig::to_le_u32(&buf[8..12]);
        let pc_program_page = DeviceConfig::to_le_u32(&buf[12..16]);
        let pc_erase_sector = DeviceConfig::to_le_u32(&buf[16..20]);
        let pc_erase_all = DeviceConfig::to_le_u32(&buf[20..24]);
        let data_section_offset = DeviceConfig::to_le_u32(&buf[24..28]);
        let flash_start_addr = DeviceConfig::to_le_u32(&buf[28..32]);
        let flash_end_addr = DeviceConfig::to_le_u32(&buf[32..36]);
        let flash_page_size = DeviceConfig::to_le_u32(&buf[36..40]);
        let erased_byte = DeviceConfig::to_le_u32(&buf[40..44]);
        let flash_sector_size = DeviceConfig::to_le_u32(&buf[44..48]);
        let program_timeout = DeviceConfig::to_le_u32(&buf[48..52]);
        let erase_timeout = DeviceConfig::to_le_u32(&buf[52..56]);
        let ram_size = DeviceConfig::to_le_u32(&buf[56..60]);
        let flash_size = DeviceConfig::to_le_u32(&buf[60..64]);
        let name = String::from_utf8((&buf[64..96]).to_vec())
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?
            .trim_matches(char::from(0))
            .to_string();
        let target = String::from_utf8((&buf[96..128]).to_vec())
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?
            .trim_matches(char::from(0))
            .to_string();

        Ok(DeviceConfig {
            magic,
            pc_init,
            pc_uninit,
            pc_program_page,
            pc_erase_sector,
            pc_erase_all,
            data_section_offset,
            flash_start_addr,
            flash_end_addr,
            flash_page_size,
            erased_byte,
            flash_sector_size,
            program_timeout,
            erase_timeout,
            ram_size,
            flash_size,
            name,
            target,
        })
    }
}
