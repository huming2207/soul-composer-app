#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod device;
mod prog;

use crate::device::{
    proto_codec::cdc_close, proto_codec::cdc_get_device_info, proto_codec::cdc_open,
    proto_codec::cdc_ping, proto_codec::ProtoCodecState, serial_detect::detect_device,
};

use crate::prog::arm::flash_stub_gen::prog_arm_gen_flash_algo;

fn main() {
    tauri::Builder::default()
        .manage(ProtoCodecState::default())
        .invoke_handler(tauri::generate_handler![
            detect_device,
            cdc_get_device_info,
            cdc_open,
            cdc_close,
            cdc_ping,
            prog_arm_gen_flash_algo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
