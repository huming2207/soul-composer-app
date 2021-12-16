import { FolderOpen, AttachEmail, DriveFileRenameOutline, DeveloperBoard } from '@mui/icons-material';
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
  Paper,
  TextField,
} from '@mui/material';
import { verify } from 'crypto';
import React from 'react';

export const FirmwareView = (): JSX.Element => {
  return (
    <>
      <Paper>
        <List>
          <ListItem
            secondaryAction={
              <IconButton edge="end" aria-label="open-file" onClick={() => {}}>
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
              value={'Test'}
              fullWidth
              label="Firmware binary"
              size="small"
              defaultValue=""
              focused
            />
          </ListItem>
          <ListItem></ListItem>
        </List>
      </Paper>
    </>
  );
};
