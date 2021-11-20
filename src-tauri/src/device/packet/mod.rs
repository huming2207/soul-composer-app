pub mod device_info;
pub mod misc;
pub mod pkt_header;

use crc::{Crc, CRC_16_XMODEM};
pub const CDC_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);