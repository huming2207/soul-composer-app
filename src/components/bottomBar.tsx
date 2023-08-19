import { UsbOffRounded, UsbRounded } from "@mui/icons-material";
import { AppBar, IconButton, Toolbar, Typography } from "@mui/material";
import React from "react";
import { SIDeviceInfoState } from "../models/SoulDevice";
import { useSnapshot } from "valtio";

export const BottomBar = (): JSX.Element => {
  const deviceSnap = useSnapshot(SIDeviceInfoState);
  return (
    <>
      <AppBar position="fixed" color="primary" sx={{ top: "auto", bottom: 0 }}>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            {"Device: " + deviceSnap.macAddr || "disconnected"}
          </Typography>
          <IconButton size="large" color="inherit">
            {deviceSnap.macAddr ? <UsbRounded /> : <UsbOffRounded />}
          </IconButton>
        </Toolbar>
      </AppBar>
    </>
  );
};
