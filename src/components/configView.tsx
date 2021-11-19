import { AttachEmail, FolderOpen } from '@mui/icons-material';
import {
  Avatar,
  Checkbox,
  IconButton,
  List,
  ListItem,
  ListItemAvatar,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  TextField,
} from '@mui/material';
import { dialog } from '@tauri-apps/api';
import React, { useState } from 'react';

export const ConfigView = (): JSX.Element => {
  const [flashAlgo, setFlashAlgo] = useState<string>();
  const [erase, setErase] = useState<boolean>(true);
  const [verify, setVerify] = useState<boolean>(true);
  const openFlashAlgo = async () => {
    const path = (await dialog.open({
      filters: [{ extensions: ['bin', 'elf'], name: 'Flash algorithm binary' }],
      multiple: false,
    })) as string;

    setFlashAlgo(path);
  };

  return (
    <>
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
            <ListItemText primary="Verify" secondary="Read back and compare after flashing" />
          </ListItemButton>
        </ListItem>
      </List>
    </>
  );
};
