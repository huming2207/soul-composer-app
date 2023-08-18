import { UsbOffRounded, UsbRounded } from "@mui/icons-material";
import { AppBar, IconButton, Toolbar, Typography } from "@mui/material";
import React from "react";
import { SIDevice, SIDeviceInfo } from "../models/SoulDevice";

export const BottomBar = (): JSX.Element => {
  const { deviceOpened } = SIDevice;
  return (
    <>
      <AppBar position="fixed" color="primary" sx={{ top: "auto", bottom: 0 }}>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            {"Device: " + SIDeviceInfo.macAddr}
          </Typography>
          <IconButton size="large" color="inherit">
            {deviceOpened ? <UsbRounded /> : <UsbOffRounded />}
          </IconButton>
        </Toolbar>
      </AppBar>
    </>
  );
};
