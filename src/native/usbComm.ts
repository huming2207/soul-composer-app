import { SOUL_INJECTOR_USB_PID, SOUL_INJECTOR_USB_VID } from "../common/hardware";

export const openSoulInjectorDevice = async (): Promise<void> => {
  if (!("serial" in navigator)) {
    console.error("Your browser doesn't support WebSerial!");
    throw new Error("Lacking WebSerial support!");
  }

  const serial = await navigator.serial.requestPort({
    filters: [{ usbProductId: SOUL_INJECTOR_USB_PID, usbVendorId: SOUL_INJECTOR_USB_VID }],
  });

  await serial.open({ baudRate: 9600 });
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
