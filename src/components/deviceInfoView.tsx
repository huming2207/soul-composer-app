import { AssignmentTurnedIn, Build, Fingerprint, Memory } from '@mui/icons-material';
import { Card, CardContent, Typography, Avatar, List, ListItem, ListItemAvatar, ListItemText } from '@mui/material';
import { Observer } from 'mobx-react-lite';
import React, { useState } from 'react';
import SoulDeviceStateInstance from '../models/SoulDevice';
import { queryDeviceInfo, SoulDeviceInfo, SoulPacket } from '../native/invoke';

export const DeviceInfoView = (): JSX.Element => {
  const deviceState = SoulDeviceStateInstance;
  const [deviceInfo, setDeviceInfo] = useState<SoulPacket<SoulDeviceInfo>>();
  const refreshDeviceInfo = () => {
    (async () => {
      setDeviceInfo(await queryDeviceInfo());
    })();
  };

  return (
    <Observer>
      {() => {
        if (deviceState.selectedDevice) {
          refreshDeviceInfo();
        }

        return deviceState.selectedDevice ? (
          <List sx={{ width: '100%', maxWidth: 600, bgcolor: 'background.paper' }}>
            <ListItem>
              <ListItemAvatar>
                <Avatar>
                  <AssignmentTurnedIn />
                </Avatar>
              </ListItemAvatar>
              <ListItemText primary="Model name" secondary={deviceInfo?.body.devModel || 'Unknown'} />
            </ListItem>
            <ListItem>
              <ListItemAvatar>
                <Avatar>
                  <Fingerprint />
                </Avatar>
              </ListItemAvatar>
              <ListItemText primary="Serial number" secondary={deviceState.selectedDevice.serialNumber || 'Unknown'} />
            </ListItem>
            <ListItem>
              <ListItemAvatar>
                <Avatar>
                  <Build />
                </Avatar>
              </ListItemAvatar>
              <ListItemText primary="Firmware version" secondary={deviceInfo?.body.devBuild || 'Unknown'} />
            </ListItem>
            <ListItem>
              <ListItemAvatar>
                <Avatar>
                  <Memory />
                </Avatar>
              </ListItemAvatar>
              <ListItemText primary="SDK version" secondary={deviceInfo?.body.espIdfVer || 'Unknown'} />
            </ListItem>
          </List>
        ) : (
          <Card sx={{ minWidth: 275 }}>
            <CardContent>
              <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
                Oops...
              </Typography>
              <Typography variant="h5" component="div">
                Device disconnected
              </Typography>
              <Typography variant="body2">Click the USB button at the bottom-left corner to start</Typography>
            </CardContent>
          </Card>
        );
      }}
    </Observer>
  );
};
