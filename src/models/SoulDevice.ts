import { makeAutoObservable, runInAction } from 'mobx';

export interface ScannedDevice {
  port: string;
  serialNumber: string;
}

export interface ScannedResult {
  devices: ScannedDevice[];
}

class SoulDeviceState {
  public selectedDevice?: ScannedDevice = undefined;
  public scanResult: ScannedDevice[] = [];

  constructor() {
    makeAutoObservable(this);
  }

  public setScanResult(result: ScannedResult): void {
    console.log(result.devices);
    console.log(this.scanResult);
    runInAction(() => {
      this.scanResult = result.devices;
    });
  }

  public setSelectedDevice(device: ScannedDevice): void {
    runInAction(() => {
      this.selectedDevice = device;
    });
  }
}

export const SoulDeviceStateInstance = new SoulDeviceState();
export default SoulDeviceStateInstance;
