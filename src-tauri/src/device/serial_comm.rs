use serialport::SerialPort;

use crate::device::error::DeviceError;

pub struct SerialComm {
    pub serial_port: String,
    serial: Box<dyn SerialPort>,
}

impl SerialComm {
    pub fn new(port: String) -> Result<SerialComm, DeviceError> {
        let mut serial = serialport::new(&port, 115200).open().map_err(|err| DeviceError::OpenError(err.to_string()))?;

        Ok(SerialComm {
            serial,
            serial_port: port,
        })
    }

    pub fn read(&mut self) -> Result<Vec<u8>, DeviceError> {
        let len = self.serial.bytes_to_read().map_err(|err| DeviceError::ReadError(err.to_string()))?;
        if len < 1 {
            return Err(DeviceError::NothingToRead);
        }

        let buf: Vec<u8> = vec![0; len as usize];
        let read_len = self.serial.read(&mut buf).map_err(|err| DeviceError::ReadError(err.to_string()))?;
        
        if read_len < 1 {
            return Err(DeviceError::NothingToRead);
        } else {
            return Ok(buf);
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError> {
        match self.serial.write(data) {
            Ok(ret) => Ok(ret),
            Err(err) => Err(DeviceError::WriteError(err.to_string()))
        }
    }
}
