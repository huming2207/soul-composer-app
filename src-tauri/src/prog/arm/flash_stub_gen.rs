use std::{fs, path::Path};

use base64::STANDARD;
use goblin::elf::Elf;
use serde::{Deserialize, Serialize};

use crate::prog::arm::flash_device::FlashDevice;

use super::{algorithm_binary::AlgorithmBinary, arm_error::ArmError};
use base64_serde::base64_serde_type;

base64_serde_type!(Base64Standard, STANDARD);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmFlashStub {
    pub name: String,
    pub description: String,
    pub default: bool,

    #[serde(with = "Base64Standard")]
    pub instructions: Vec<u8>,
    pub pc_init: Option<u32>,
    pub pc_uninit: Option<u32>,
    pub pc_program_page: u32,
    pub pc_erase_sector: u32,
    pub pc_erase_all: Option<u32>,
    pub pc_verify: Option<u32>,
    pub data_section_offset: u32,
    pub flash_start_addr: u32,
    pub flash_end_addr: u32,
    pub flash_page_size: u32,
    pub erased_byte_value: u8,
    pub flash_sector_size: u32,
    pub program_timeout: u32,
    pub erase_timeout: u32,
    pub ram_size: u32,
    pub flash_size: u32,
}

fn extract_flash_device(elf: &goblin::elf::Elf, buffer: &[u8]) -> Result<FlashDevice, ArmError> {
    // Extract the flash device info.
    for sym in elf.syms.iter() {
        let name = &elf.strtab[sym.st_name];

        if let "FlashDevice" = name {
            // This struct contains information about the FLM file structure.
            let address = sym.st_value as u32;
            return FlashDevice::new(&elf, buffer, address);
        }
    }

    // Failed to find flash device
    Err(ArmError::FlashDeviceInfoNotFound)
}

impl ArmFlashStub {
    pub fn from_elf(
        buf: &[u8],
        name: String,
        default: bool,
        ram_size: u32,
    ) -> Result<ArmFlashStub, ArmError> {
        let elf = match Elf::parse(buf) {
            Ok(elf) => elf,
            Err(_) => return Err(ArmError::ElfParse),
        };

        let flash_device = extract_flash_device(&elf, buf)?;
        let algorithm_binary = AlgorithmBinary::new(&elf, buf)?;
        let mut algo = ArmFlashStub::default();

        // Extract the function pointers.
        let code_section_offset = algorithm_binary.code_section.start;
        for sym in elf.syms.iter() {
            let name = &elf.strtab[sym.st_name];

            match name {
                "Init" => algo.pc_init = Some(sym.st_value as u32 - code_section_offset),
                "UnInit" => algo.pc_uninit = Some(sym.st_value as u32 - code_section_offset),
                "EraseChip" => algo.pc_erase_all = Some(sym.st_value as u32 - code_section_offset),
                "EraseSector" => algo.pc_erase_sector = sym.st_value as u32 - code_section_offset,
                "ProgramPage" => algo.pc_program_page = sym.st_value as u32 - code_section_offset,
                "Verify" => algo.pc_verify = Some(sym.st_value as u32 - code_section_offset),
                _ => {}
            }
        }

        algo.instructions = algorithm_binary.blob();
        algo.name = name;
        algo.description = flash_device.name;
        algo.data_section_offset = algorithm_binary.data_section.start;
        algo.flash_sector_size = flash_device.sectors[0].size;
        algo.flash_start_addr = flash_device.start_address;
        algo.flash_end_addr = flash_device.start_address + flash_device.device_size;
        algo.flash_size = flash_device.device_size;
        algo.flash_page_size = flash_device.page_size;
        algo.erase_timeout = flash_device.erase_sector_timeout;
        algo.program_timeout = flash_device.program_page_timeout;
        algo.erased_byte_value = flash_device.erased_default_value;
        algo.default = default;
        algo.ram_size = ram_size;

        Ok(algo)
    }
}

#[tauri::command]
pub async fn prog_arm_gen_flash_algo_from_base64(
    buf_base64: String,
    name: String,
    default: bool,
    ram_size: u32,
) -> Result<String, String> {
    let buf = base64::decode(buf_base64).map_err(|err| err.to_string())?;

    let algo =
        ArmFlashStub::from_elf(&buf, name, default, ram_size).map_err(|err| err.to_string())?;
    let json = serde_json::to_string(&algo).map_err(|err| err.to_string())?;
    Ok(json)
}

#[tauri::command]
pub async fn prog_arm_gen_flash_algo(
    path: String,
    name: String,
    default: bool,
    ram_size: u32,
) -> Result<String, String> {
    let buf = fs::read(Path::new(&path)).map_err(|err| err.to_string())?;

    let algo =
        ArmFlashStub::from_elf(&buf, name, default, ram_size).map_err(|err| err.to_string())?;
    let json = serde_json::to_string(&algo).map_err(|err| err.to_string())?;
    Ok(json)
}
