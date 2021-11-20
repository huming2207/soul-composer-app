use serde::{Deserialize, Serialize};
use serialport::{available_ports, SerialPortType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedResult {
    pub port: String,
    pub serial_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialDetect {
    pub devices: Vec<EnumeratedResult>,
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
                                let device_info = EnumeratedResult {
                                    port: port.port_name,
                                    serial_number: info.serial_number.unwrap_or_default(),
                                };
                                devices.push(device_info);
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(_) => todo!(),
        }

        SerialDetect { devices }
    }
}

#[tauri::command]
pub async fn detect_device() -> Result<String, String> {
    let detected = SerialDetect::detect();
    let result = match serde_json::to_string(&detected) {
        Ok(ret) => ret,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    Ok(result)
}
