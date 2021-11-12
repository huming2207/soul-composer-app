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

export const queryDeviceInfo = async (): Promise<SoulPacket<SoulDeviceInfo>> => {
  return new Promise<SoulPacket<SoulDeviceInfo>>((resolve, reject) => {
    invoke('cdc_open')
      .then((ret: any) => resolve(JSON.parse(ret)))
      .catch((err) => reject(err));
  });
};
