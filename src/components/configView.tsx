import { AttachEmail, DeveloperBoard, DriveFileRenameOutline, FolderOpen } from '@mui/icons-material';
import {
  Avatar,
  Button,
  Checkbox,
  Dialog,
  DialogTitle,
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
} from '@mui/material';
import { maxHeight } from '@mui/system';
import { dialog } from '@tauri-apps/api';
import React, { useState } from 'react';
import { FixedSizeList } from 'react-window';
import { FlashAlgoMetadata, genArmFlashAlgoMetadata } from '../native/invoke';

export const ConfigView = (): JSX.Element => {
  const [flashAlgo, setFlashAlgo] = useState<string>();
  const [erase, setErase] = useState<boolean>(true);
  const [targetName, setTargetName] = useState<string>();
  const [ramSize, setRamSize] = useState<number>();
  const [verify, setVerify] = useState<boolean>(true);
  const [algoMetadata, setAlgoMetadata] = useState<FlashAlgoMetadata>();
  const [openAttributeDialog, setOpenAttributeDialog] = useState<boolean>(false);
  const openFlashAlgo = async () => {
    const path = (await dialog.open({
      filters: [{ extensions: ['bin', 'elf', 'ELF', 'BIN'], name: 'Flash algorithm binary' }],
      multiple: false,
    })) as string;

    setFlashAlgo(path);
    setAlgoMetadata(await genArmFlashAlgoMetadata(path, targetName || 'Generic', true, ramSize || 8192));
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
            <ListItemAvatar>
              <Avatar>
                <AttachEmail />
              </Avatar>
            </ListItemAvatar>
            <TextField
              id="outlined"
              value={flashAlgo}
              fullWidth
              label="Flash algorithm binary"
              size="small"
              defaultValue=""
              focused
            />
          </ListItem>
          <ListItem>
            <ListItemAvatar>
              <Avatar>
                <DriveFileRenameOutline />
              </Avatar>
            </ListItemAvatar>
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
          </ListItem>
          <ListItem>
            <ListItemAvatar>
              <Avatar>
                <DeveloperBoard />
              </Avatar>
            </ListItemAvatar>
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
          </ListItem>
          <ListItem>
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
              <ListItemText primary="Erase" secondary="Clear out the whole flash before flashing" />
            </ListItemButton>
          </ListItem>
          <ListItem>
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
          </ListItem>
        </List>
      </Paper>
      <Paper sx={{ marginTop: 1, padding: 2 }}>
        <Stack direction="row" spacing={2}>
          <Button variant="contained" color="success">
            Write to programmer
          </Button>
          <Button
            variant="contained"
            disabled={algoMetadata === undefined}
            color="info"
            onClick={() => {
              setOpenAttributeDialog(true);
            }}
          >
            Flash algorithm attributes
          </Button>
          <Button variant="contained" color="warning">
            Load settings
          </Button>
        </Stack>
      </Paper>
      <Dialog onClose={() => setOpenAttributeDialog(false)} open={openAttributeDialog}>
        <DialogTitle>Flash algorithm information</DialogTitle>
        <List sx={{ pt: 0 }}>
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
