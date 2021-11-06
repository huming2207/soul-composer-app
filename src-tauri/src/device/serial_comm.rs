use std::sync::{Arc, Mutex};

use serial_line_ip::{Decoder, Encoder};
use serialport::{SerialPort, TTYPort};
use crate::device::error::DeviceError;

pub struct SerialComm {
    pub serial_port: String,
    serial: Box<dyn SerialPort>,
}



impl SerialComm {
    pub fn new(port: String) -> Result<SerialComm, DeviceError> {
        let serial = serialport::new(&port, 115200).open().map_err(|err| DeviceError::OpenError(err.to_string()))?;

        Ok(SerialComm {
            serial,
            serial_port: port,
        })
    }

    pub fn read(&self) -> Result<Vec<u8>, DeviceError> {
        let mut serial = self.serial.try_clone().map_err(|err| DeviceError::ReadError(err.to_string()))?;
        let len = serial.bytes_to_read().map_err(|err| DeviceError::ReadError(err.to_string()))?;
        if len < 1 {
            return Err(DeviceError::NothingToRead);
        }

        let mut buf: Vec<u8> = vec![0; len as usize];
        let read_len = serial.read(&mut buf).map_err(|err| DeviceError::ReadError(err.to_string()))?;
        
        if read_len < 1 {
            return Err(DeviceError::NothingToRead);
        } else {
            let mut decoder = Decoder::new();
            let mut output_slice: Vec<u8> = vec![0; read_len];
            let (_bytes_processed, output, is_eop) = decoder.decode(&buf, &mut output_slice).map_err(|err| DeviceError::ReadError(err.to_string()))?;

            if is_eop {
                return Ok(output.to_vec());
            } else {
                return Err(DeviceError::ReadError("Packet not ended".to_string()))
            }
        }
    }

    pub fn write(&self, data: &[u8]) -> Result<usize, DeviceError> {
        let mut serial = self.serial.try_clone().map_err(|err| DeviceError::ReadError(err.to_string()))?;
        let mut output: Vec<u8> = vec![0; data.len() * 2 + 2]; // Worst case senario is 2x of the original buffer (assuming someone send all ESC and/or END) + 2 of 0xC0
        let mut encoder = Encoder::new();
        let encoded = encoder.encode(data, &mut output).map_err(|err| DeviceError::ReadError(err.to_string()))?;


        match serial.write(&output[..encoded.written]) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(DeviceError::WriteError(err.to_string()))
        }
    }
}
