import { proxy } from "valtio";

export const SIDeviceInfo = proxy({
  macAddr: "Unknown",
  flashId: "Unknown",
  sdkVer: "Unknown",
  devModel: "Unknown",
  devBuild: "Unknown",
});

export const SIDevice = proxy({
  deviceOpened: false,
});
