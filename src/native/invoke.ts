import { invoke } from '@tauri-apps/api';
import { ScannedResult } from '../models/SoulDevice';

export const scanSoulInjectorDevices = async (): Promise<ScannedResult> => {
  return new Promise<ScannedResult>((resolve, reject) => {
    invoke('detect_device')
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};

export const openSoulInjectorDevice = async (port: string): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    invoke('cdc_open', { invokeMessage: port })
      .then(() => resolve())
      .catch((err) => reject(err));
  });
};

export interface PacketHeader {
  pktType: number;
  len: number;
  crc: number;
}

export interface SoulDeviceInfo {
  macAddr: string;
  flashId: string;
  espIdfVer: string;
  devModel: string;
  devBuild: string;
}

export interface SoulPacket<T> {
  header: PacketHeader;
  body: T;
}

export interface FirmwareMetadata {
  crc: number;
  len: number;
  name: string;
}

export const queryDeviceInfo = async (): Promise<SoulPacket<SoulDeviceInfo>> => {
  return new Promise<SoulPacket<SoulDeviceInfo>>((resolve, reject) => {
    invoke('cdc_get_device_info')
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};

export const closeSoulInjectorDevice = async (): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    invoke('cdc_close')
      .then(() => resolve())
      .catch((err) => reject(err));
  });
};

export interface FlashAlgoMetadata {
  name: string;
  description: string;
  default: boolean;
  instructions: string;
  pcInit?: number;
  pcUninit?: number;
  pcProgramPage: number;
  pcEraseSector: number;
  pcEraseAll?: number;
  dataSectionOffset: number;
  flashStartAddr: number;
  flashEndAddr: number;
  flashPageSize: number;
  erasedByteValue: number;
  flashSectorSize: number;
  programTimeout: number;
  eraseTimeout: number;
  ramSize: number;
  flashSize: number;
}

export const genArmFlashAlgoMetadata = async (
  path: string,
  name: string,
  defaultAlgo: boolean,
  ramSize: number,
): Promise<FlashAlgoMetadata> => {
  return new Promise<FlashAlgoMetadata>((resolve, reject) => {
    invoke('prog_arm_gen_flash_algo', { path, name, default: defaultAlgo, ramSize })
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};

export const sendConfig = async (
  path: string,
  name: string,
  defaultAlgo: boolean,
  ramSize: number,
): Promise<FlashAlgoMetadata> => {
  return new Promise<FlashAlgoMetadata>((resolve, reject) => {
    invoke('cdc_send_config', { path, name, default: defaultAlgo, ramSize })
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};

export const sendFlashAlgo = async (
  path: string,
  name: string,
  defaultAlgo: boolean,
  ramSize: number,
): Promise<FlashAlgoMetadata> => {
  return new Promise<FlashAlgoMetadata>((resolve, reject) => {
    invoke('cdc_send_flash_algo', { path, name, default: defaultAlgo, ramSize })
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};

export const sendFirmware = async (path: string, name: string): Promise<FirmwareMetadata> => {
  return new Promise<FirmwareMetadata>((resolve, reject) => {
    invoke('cdc_send_firmware', { path, name })
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};
