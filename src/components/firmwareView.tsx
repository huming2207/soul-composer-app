import { FolderOpen, AttachEmail } from "@mui/icons-material";
import { Avatar, IconButton, List, ListItem, ListItemAvatar, Paper, TextField } from "@mui/material";
import React from "react";

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
            <TextField id="outlined" value={"Test"} fullWidth label="Firmware binary" size="small" defaultValue="" focused />
          </ListItem>
          <ListItem></ListItem>
        </List>
      </Paper>
    </>
  );
};
