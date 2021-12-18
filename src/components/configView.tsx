import { AttachEmail, DeveloperBoard, DriveFileRenameOutline, FolderOpen } from '@mui/icons-material';
import {
  Avatar,
  Button,
  Checkbox,
  Dialog,
  DialogTitle,
  Divider,
  Grid,
  IconButton,
  List,
  ListItem,
  ListItemAvatar,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Paper,
  Stack,
  TextField,
  Typography,
} from '@mui/material';
import { dialog } from '@tauri-apps/api';
import React, { useState } from 'react';
import { FlashAlgoMetadata, genArmFlashAlgoMetadata, sendConfig, sendFlashAlgo } from '../native/invoke';

export const ConfigView = (): JSX.Element => {
  const [flashAlgoPath, setFlashAlgoPath] = useState<string>();
  const [firmwarePath, setFirmwarePath] = useState<string>();
  const [erase, setErase] = useState<boolean>(true);
  const [targetName, setTargetName] = useState<string>();
  const [ramSize, setRamSize] = useState<number>();
  const [verify, setVerify] = useState<boolean>(true);
  const [algoMetadata, setAlgoMetadata] = useState<FlashAlgoMetadata>();
  const [openAttributeDialog, setOpenAttributeDialog] = useState<boolean>(false);
  const openFlashAlgo = async () => {
    const path = (await dialog.open({
      filters: [{ extensions: ['elf', 'ELF'], name: 'Flash algorithm binary' }],
      multiple: false,
    })) as string;

    setFlashAlgoPath(path);
    setAlgoMetadata(await genArmFlashAlgoMetadata(path, targetName || 'Generic', true, ramSize || 8192));
  };

  const openFirmwareBlob = async () => {
    const path = (await dialog.open({
      filters: [{ extensions: ['bin', 'BIN'], name: 'Firmware binary' }],
      multiple: false,
    })) as string;

    setFirmwarePath(path);
  };

  const sendConfigAndAlgo = async () => {
    if (flashAlgoPath && targetName && ramSize) {
      await sendConfig(flashAlgoPath, targetName, true, ramSize);
      await sendFlashAlgo(flashAlgoPath, targetName, true, ramSize);
    }
  };

  return (
    <>
      <Paper>
        <List>
          <ListItem divider>
            <Typography variant="h5" component="div">
              Flash algorithm
            </Typography>
          </ListItem>
          <ListItem></ListItem>
          <ListItem
            secondaryAction={
              <IconButton edge="end" aria-label="open-file" onClick={openFlashAlgo}>
                <FolderOpen />
              </IconButton>
            }
          >
            <TextField
              id="outlined"
              value={flashAlgoPath}
              fullWidth
              label="Flash algorithm binary"
              size="small"
              defaultValue=""
              focused
            />
          </ListItem>
          <ListItem
            secondaryAction={
              <IconButton edge="end" aria-label="open-file" onClick={openFirmwareBlob}>
                <FolderOpen />
              </IconButton>
            }
          >
            <TextField
              id="outlined"
              value={firmwarePath}
              fullWidth
              label="Firmware binary"
              size="small"
              defaultValue=""
              focused
            />
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
                    setTargetName(event.target.value);
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
                  inputProps={{ inputMode: 'numeric', pattern: '[0-9]*' }}
                  focused
                  onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                    setRamSize(parseInt(event.target.value));
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
                    setErase(!erase);
                  }}
                  dense
                >
                  <ListItemIcon>
                    <Checkbox edge="start" checked={erase} tabIndex={-1} disableRipple />
                  </ListItemIcon>
                  <ListItemText primary="Erase" secondary="Wipe the flash before flashing" />
                </ListItemButton>
              </Grid>
              <Grid item xs={6}>
                <ListItemButton
                  role={undefined}
                  onClick={() => {
                    setVerify(!verify);
                  }}
                  dense
                >
                  <ListItemIcon>
                    <Checkbox edge="start" checked={verify} tabIndex={-1} disableRipple />
                  </ListItemIcon>
                  <ListItemText primary="Verify" secondary="Validate after flashing" />
                </ListItemButton>
              </Grid>
            </Grid>
          </ListItem>
          <ListItem>
            <Stack direction="row" spacing={3}>
              <Button size="small" variant="contained" color="success" onClick={async () => await sendConfigAndAlgo()}>
                Write to programmer
              </Button>
              <Button
                size="small"
                variant="contained"
                disabled={algoMetadata === undefined}
                color="info"
                onClick={() => {
                  setOpenAttributeDialog(true);
                }}
              >
                Flash algorithm attributes
              </Button>
              <Button size="small" variant="contained" color="warning">
                Load settings
              </Button>
            </Stack>
          </ListItem>
        </List>
      </Paper>
      <Paper sx={{ marginTop: 1, padding: 2 }}></Paper>
      <Dialog onClose={() => setOpenAttributeDialog(false)} open={openAttributeDialog}>
        <DialogTitle>Flash algorithm information</DialogTitle>
        <List>
          <ListItem>
            <ListItemText
              primary="Function pointers"
              secondary={`init: ${algoMetadata?.pcInit?.toString(16) || '??'}; unInit: ${
                algoMetadata?.pcUninit?.toString(16) || '??'
              }; eraseSector: ${algoMetadata?.pcEraseSector?.toString(16) || '??'}; programPage: ${
                algoMetadata?.pcProgramPage?.toString(16) || '??'
              }`}
            />
          </ListItem>
          <ListItem>
            <ListItemText primary="Flash size" secondary={algoMetadata?.flashSize || 'Unknown'}></ListItemText>
          </ListItem>
          <ListItem>
            <ListItemText primary="RAM size" secondary={algoMetadata?.ramSize || 'Unknown'}></ListItemText>
          </ListItem>
          <ListItem>
            <ListItemText
              primary="Flash attribute"
              secondary={`startAddr: ${algoMetadata?.flashStartAddr?.toString(16) || '??'}; endAddr: ${
                algoMetadata?.flashEndAddr?.toString(16) || '??'
              }; pageSize: ${algoMetadata?.flashPageSize?.toString(10) || '??'}; sectorSize: ${
                algoMetadata?.flashSectorSize?.toString(10) || '??'
              }; erasedOffset: ${algoMetadata?.erasedByteValue || '??'}`}
            ></ListItemText>
          </ListItem>
        </List>
      </Dialog>
    </>
  );
};
