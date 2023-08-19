import { FolderOpen } from "@mui/icons-material";
import { Button, Checkbox, Grid, IconButton, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Paper, Stack, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import React, { useState } from "react";
import { FlashAlgoDialog } from "./flashAlgoDialog";
import { useSnackbar } from "notistack";
import { FlashAlgoMetadata } from "../native/flashAlgoElf";
import { useSnapshot } from "valtio";
import { WebUIConfigState } from "../models/webUIState";

export const ConfigView = (): JSX.Element => {
  const { enqueueSnackbar } = useSnackbar();
  const { algoMetadata, setAlgoMetadata } = useState<FlashAlgoMetadata | undefined>(undefined);
  const configSnap = useSnapshot(WebUIConfigState);

  return (
    <>
      <Paper>
        <List>
          <ListItem
            secondaryAction={
              <IconButton edge="end" aria-label="open-file">
                <FolderOpen />
              </IconButton>
            }
          >
            <TextField id="outlined" fullWidth label="Flash algorithm binary" size="small" defaultValue="" focused />
          </ListItem>
          <ListItem
            secondaryAction={
              <IconButton edge="end" aria-label="open-file">
                <FolderOpen />
              </IconButton>
            }
          >
            <TextField id="outlined" fullWidth label="Firmware binary" size="small" defaultValue="" focused />
          </ListItem>
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <TextField
                  id="outlined"
                  fullWidth
                  label="Target name"
                  size="small"
                  defaultValue=""
                  focused
                  onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                    // setTargetName(event.target.value);
                  }}
                />
              </Grid>
              <Grid item xs={6}>
                <TextField
                  id="outlined"
                  fullWidth
                  label="RAM size"
                  size="small"
                  type="text"
                  inputProps={{ inputMode: "numeric", pattern: "[0-9]*" }}
                  focused
                  onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                    // setRamSize(parseInt(event.target.value));
                  }}
                />
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <ListItemButton
                  role={undefined}
                  onClick={() => {
                    // setErase(!erase);
                  }}
                  dense
                >
                  <ListItemIcon>
                    <Checkbox edge="start" checked={configSnap.erase} tabIndex={-1} disableRipple />
                  </ListItemIcon>
                  <ListItemText primary="Erase" secondary="Wipe the flash before flashing" />
                </ListItemButton>
              </Grid>
              <Grid item xs={6}>
                <ListItemButton
                  role={undefined}
                  onClick={() => {
                    // setVerify(!verify);
                  }}
                  dense
                >
                  <ListItemIcon>
                    <Checkbox edge="start" checked={configSnap.verify} tabIndex={-1} disableRipple />
                  </ListItemIcon>
                  <ListItemText primary="Verify" secondary="Validate after flashing" />
                </ListItemButton>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Stack direction="row" spacing={3}>
              <LoadingButton loading={configSnap.sendingFw} size="medium" loadingPosition="start" variant="contained" color="success">
                Write to programmer
              </LoadingButton>
              <Button
                size="medium"
                variant="contained"
                disabled={algoMetadata === undefined}
                color="info"
                onClick={() => {
                  // setOpenAttributeDialog(true);
                }}
              >
                Flash algorithm attributes
              </Button>
              <Button size="medium" variant="contained" color="warning">
                Load settings
              </Button>
            </Stack>
          </ListItem>
        </List>
      </Paper>
      <FlashAlgoDialog
        open={openAttributeDialog}
        handleClose={() => {
          setOpenAttributeDialog(false);
        }}
        metadata={algoMetadata}
      />
    </>
  );
};
