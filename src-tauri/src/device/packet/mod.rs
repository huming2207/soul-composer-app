pub mod device_cfg;
pub mod device_info;
pub mod file_chunk;
pub mod misc;
pub mod pkt_header;

use std::convert::TryInto;

use crc::{Crc, CRC_16_XMODEM, CRC_32_CKSUM};
pub const CDC_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);
pub const BLOB_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);

#[inline]
pub(crate) fn slice_to_le_u32(buf: &[u8]) -> u32 {
    let arr: [u8; 4] = match buf.try_into() {
        Ok(arr) => arr,
        Err(_) => [0, 0, 0, 0],
    };

    u32::from_le_bytes(arr)
}