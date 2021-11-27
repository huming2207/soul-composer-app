pub mod device_cfg;
pub mod device_info;
pub mod misc;
pub mod pkt_header;
pub mod file_chunk;

use crc::{Crc, CRC_16_XMODEM};
pub const CDC_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);
