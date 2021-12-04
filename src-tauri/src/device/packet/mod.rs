pub mod device_cfg;
pub mod device_info;
pub mod file_chunk;
pub mod misc;
pub mod pkt_header;

use crc::{Crc, CRC_16_XMODEM, CRC_32_CKSUM};
pub const CDC_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);
pub const BLOB_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);
