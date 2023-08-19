import { proxy } from "valtio";

export interface SIDeviceInfo {
  macAddr?: string;
  flashId?: string;
  sdkVer?: string;
  devModel?: string;
  devBuild?: string;
}

export const SIDeviceInfoState = proxy({
  macAddr: "Unknown",
  flashId: "Unknown",
  sdkVer: "Unknown",
  devModel: "Unknown",
  devBuild: "Unknown",
} as SIDeviceInfo);
