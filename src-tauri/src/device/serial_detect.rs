use serde::{Deserialize, Serialize};
use serialport::SerialPortType;
use tokio_serial::available_ports;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub port: String,
    pub dev_sn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialDetect {
    pub devices: Vec<DeviceInfo>
}

impl SerialDetect {
    pub fn detect() -> SerialDetect {
        let mut devices = Vec::new();
        match available_ports() {
            Ok(ports) => {
                for port in ports {
                    match port.port_type {
                        SerialPortType::UsbPort(info) => {
                            if info.vid == 0x303a && info.pid == 0x80ce {
                                let device_info = DeviceInfo { port: port.port_name, dev_sn: info.serial_number.unwrap_or_default() };
                                devices.push(device_info);
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(_) => todo!(),
        }

        SerialDetect{ devices }
    }
}