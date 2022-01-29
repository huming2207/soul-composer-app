import { FolderOpen } from '@mui/icons-material';
import {
  Button,
  Checkbox,
  Grid,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Paper,
  Stack,
  TextField,
} from '@mui/material';
import { LoadingButton } from '@mui/lab';
import { dialog } from '@tauri-apps/api';
import React, { useState } from 'react';
import { FlashAlgoMetadata, genArmFlashAlgoMetadata, sendConfig, sendFirmware, sendFlashAlgo } from '../native/invoke';
import { FlashAlgoDialog } from './flashAlgoDialog';
import { useSnackbar } from 'notistack';

export const ConfigView = (): JSX.Element => {
  const [flashAlgoPath, setFlashAlgoPath] = useState<string>();
  const [firmwarePath, setFirmwarePath] = useState<string>();
  const [erase, setErase] = useState<boolean>(true);
  const [targetName, setTargetName] = useState<string>();
  const [ramSize, setRamSize] = useState<number>();
  const [verify, setVerify] = useState<boolean>(true);
  const [algoMetadata, setAlgoMetadata] = useState<FlashAlgoMetadata>();
  const [openAttributeDialog, setOpenAttributeDialog] = useState<boolean>(false);
  const [sendingFw, setSendingFw] = useState<boolean>(false);
  const { enqueueSnackbar } = useSnackbar();
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
    if (flashAlgoPath && firmwarePath && targetName && ramSize) {
      setSendingFw(true);
      enqueueSnackbar('Writing configuration...', {
        variant: 'info',
        anchorOrigin: {
          vertical: 'top',
          horizontal: 'right',
        },
      });
      await sendConfig(flashAlgoPath, targetName, true, ramSize);
      await sendFlashAlgo(flashAlgoPath, targetName, true, ramSize);
      sendFirmware(firmwarePath, targetName)
        .then(() => {
          setSendingFw(false);
          enqueueSnackbar('Configuration has been committed!', {
            variant: 'success',
            anchorOrigin: {
              vertical: 'top',
              horizontal: 'right',
            },
          });
        })
        .catch((err) => {
          setSendingFw(false);
          enqueueSnackbar(`Failed when writing configuration: ${err}`, {
            variant: 'error',
            anchorOrigin: {
              vertical: 'top',
              horizontal: 'right',
            },
          });
        });
    }
  };

  return (
    <>
      <Paper>
        <List>
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
              <LoadingButton
                loading={sendingFw}
                size="medium"
                loadingPosition="start"
                variant="contained"
                color="success"
                onClick={async () => await sendConfigAndAlgo()}
              >
                Write to programmer
              </LoadingButton>
              <Button
                size="medium"
                variant="contained"
                disabled={algoMetadata === undefined}
                color="info"
                onClick={() => {
                  setOpenAttributeDialog(true);
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
