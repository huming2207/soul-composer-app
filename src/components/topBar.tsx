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
import { toJS } from 'mobx';
import React, { useState } from 'react';
import SoulDevice, { ScannedDevice, SoulDeviceStateInstance } from '../models/SoulDevice';

export const TopBar = (): JSX.Element => {
  const [open, setOpen] = useState<boolean>(false);
  const handleDeviceSelect = (device: ScannedDevice) => {
    SoulDevice.setSelectedDevice(device);
    setOpen(false);
  };

  const deviceState = SoulDeviceStateInstance;
  console.log(deviceState);
  return (
    <>
      <AppBar>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            {`Soul Composer - ${deviceState.selectedDevice ? deviceState.selectedDevice.port : 'Disconnected'}`}
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
