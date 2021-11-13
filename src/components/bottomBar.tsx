import { DeveloperBoard, UsbOffRounded, UsbRounded } from '@mui/icons-material';
import {
  AppBar,
  Dialog,
  DialogTitle,
  IconButton,
  Toolbar,
  Typography,
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  Avatar,
} from '@mui/material';
import { Observer } from 'mobx-react-lite';
import React, { useState } from 'react';
import SoulDevice, { ScannedDevice, SoulDeviceStateInstance } from '../models/SoulDevice';
import { useInterval } from 'usehooks-ts';
import { scanSoulInjectorDevices } from '../native/invoke';

export const BottomBar = (): JSX.Element => {
  const [open, setOpen] = useState<boolean>(false);
  const handleDeviceSelect = async (device: ScannedDevice) => {
    await SoulDevice.setSelectedDevice(device);
    setOpen(false);
  };

  useInterval(async () => {
    SoulDevice.setScanResult(await scanSoulInjectorDevices());
  }, 3000);

  const deviceState = SoulDeviceStateInstance;
  return (
    <>
      <AppBar position="fixed" color="primary" sx={{ top: 'auto', bottom: 0 }}>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            {deviceState.selectedDevice ? 'Device: ' + deviceState.selectedDevice.port : 'Device disconnected'}
          </Typography>
          <IconButton size="large" color="inherit" onClick={() => setOpen(true)}>
            <Observer>{() => (SoulDevice.selectedDevice ? <UsbRounded /> : <UsbOffRounded />)}</Observer>
          </IconButton>
        </Toolbar>
      </AppBar>
      <Dialog onClose={() => setOpen(false)} open={open}>
        <DialogTitle>Select device</DialogTitle>
        <List sx={{ pt: 0 }}>
          <Observer>
            {() => {
              return (
                <>
                  {deviceState.scanResult.slice().map((device) => (
                    <ListItem button onClick={() => handleDeviceSelect(device)} key={device.serialNumber}>
                      <ListItemAvatar>
                        <Avatar>
                          <DeveloperBoard />
                        </Avatar>
                      </ListItemAvatar>
                      <ListItemText primary={device.port} secondary={device.serialNumber} />
                    </ListItem>
                  ))}
                </>
              );
            }}
          </Observer>
        </List>
      </Dialog>
    </>
  );
};
