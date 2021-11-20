use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::device::error::DeviceError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    #[serde(with = "hex")]
    pub mac_addr: [u8; 6],
    #[serde(with = "hex")]
    pub flash_id: [u8; 8],
    pub esp_idf_ver: String,
    pub dev_model: String,
    pub dev_build: String,
}

impl TryFrom<&[u8]> for DeviceInfo {
    type Error = DeviceError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        if buf.len() < 110 {
            return Err(DeviceError::DecodeError(format!(
                "Packet too short: {} bytes",
                buf.len()
            )));
        }

        let mut mac_addr: [u8; 6] = [0; 6];
        let mut flash_id: [u8; 8] = [0; 8];

        mac_addr[0..6].copy_from_slice(&buf[0..6]);
        flash_id[0..8].copy_from_slice(&buf[6..14]);
        let esp_idf_ver = String::from_utf8((&buf[14..46]).to_vec())
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?
            .trim_matches(char::from(0))
            .to_string();
        let dev_model = String::from_utf8((&buf[46..78]).to_vec())
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?
            .trim_matches(char::from(0))
            .to_string();
        let dev_build = String::from_utf8((&buf[78..110]).to_vec())
            .map_err(|err| DeviceError::DecodeError(err.to_string()))?
            .trim_matches(char::from(0))
            .to_string();
        Ok(DeviceInfo {
            mac_addr,
            flash_id,
            esp_idf_ver,
            dev_model,
            dev_build,
        })
    }
}
