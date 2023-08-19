import { proxy } from "valtio";
import { FlashAlgoMetadata } from "../native/flashAlgoElf";

export interface WebUIConfig {
  flashAlgoPath?: string;
  firmwarePath?: string;
  erase: boolean;
  targetName?: string;
  ramSize: number;
  verify: boolean;
  sendingFw: boolean;
}

export const WebUIConfigState = proxy({
  flashAlgoPath: "",
  firmwarePath: "",
  erase: true,
  targetName: "",
  ramSize: 0,
  verify: true,
  sendingFw: false,
} as WebUIConfig);

export const FlashAlgoMetadataState = proxy({
  name: "N/A",
  description: "Unknown",
  default: false,
  instructions: "",
  pcInit: 0,
  pcUninit: 0,
  pcProgramPage: 0,
  pcEraseSector: 0,
  pcEraseAll: 0,
  dataSectionOffset: 0,
  flashStartAddr: 0,
  flashEndAddr: 0,
  flashPageSize: 0,
  erasedByteValue: 0,
  flashSectorSize: 0,
  programTimeout: 0,
  eraseTimeout: 0,
  ramSize: 0,
  flashSize: 0,
} as FlashAlgoMetadata);
