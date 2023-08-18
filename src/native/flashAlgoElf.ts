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
