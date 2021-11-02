use futures::{stream::StreamExt, SinkExt};
use slip_codec::tokio::SlipCodec;
use tokio::task::JoinHandle;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::Framed;

use crate::device::error::DeviceError;

pub struct SerialComm {
    pub serial_port: String,
    serial: SerialStream,
    read_task: Option<JoinHandle<()>>,
}

impl SerialComm {
    pub fn new(port: String) -> Result<SerialComm, DeviceError> {
        let mut serial = tokio_serial::new(&port, 115200)
            .open_native_async()
            .map_err(|err| DeviceError::DeviceOpenError(err.to_string()))?;

        #[cfg(unix)]
        serial
            .set_exclusive(true)
            .map_err(|err| DeviceError::DeviceOpenError(err.to_string()))?;

        Ok(SerialComm {
            serial,
            serial_port: port,
            read_task: None,
        })
    }

    pub async fn read(&mut self) -> Result<Vec<u8>, DeviceError> {
        let serial = &mut self.serial;
        let mut reader = Framed::new(serial, SlipCodec::new());
        let mut buf: Vec<u8> = Vec::new();
        while let Some(result) = reader.next().await {
            let mut data = match result {
                Ok(data) => data,
                Err(_) => return Err(DeviceError::ReadError("SLIP decode fail".to_string())),
            };

            buf.extend_from_slice(data.as_mut());
        }

        Ok(buf)
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), DeviceError> {
        let serial = &mut self.serial;
        let mut writer = Framed::new(serial, SlipCodec::new());

        let bytes = bytes::Bytes::copy_from_slice(data);
        match writer.send(bytes).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(DeviceError::ReadError("SLIP encode fail".to_string())),
        }
    }
}
