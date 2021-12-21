import { Dialog, DialogTitle, List, ListItem, ListItemText, Paper } from '@mui/material';
import React from 'react';
import { FlashAlgoMetadata } from '../native/invoke';

export interface FlashAlgoDialogProps {
  metadata?: FlashAlgoMetadata;
  open: boolean;
  handleClose: () => void;
}

export const FlashAlgoDialog = (props: FlashAlgoDialogProps): JSX.Element => {
  return (
    <>
      <Dialog onClose={props.handleClose} open={props.open}>
        <DialogTitle>Flash algorithm information</DialogTitle>
        <Paper>
          <List>
            <ListItem>
              <ListItemText
                primary="Function pointers"
                secondary={`init: ${props.metadata?.pcInit?.toString(16) || '??'}; unInit: ${
                  props.metadata?.pcUninit?.toString(16) || '??'
                }; eraseSector: ${props.metadata?.pcEraseSector?.toString(16) || '??'}; programPage: ${
                  props.metadata?.pcProgramPage?.toString(16) || '??'
                }`}
              />
            </ListItem>
            <ListItem>
              <ListItemText primary="Flash size" secondary={props.metadata?.flashSize || 'Unknown'}></ListItemText>
            </ListItem>
            <ListItem>
              <ListItemText primary="RAM size" secondary={props.metadata?.ramSize || 'Unknown'}></ListItemText>
            </ListItem>
            <ListItem>
              <ListItemText
                primary="Flash attribute"
                secondary={`startAddr: ${props.metadata?.flashStartAddr?.toString(16) || '??'}; endAddr: ${
                  props.metadata?.flashEndAddr?.toString(16) || '??'
                }; pageSize: ${props.metadata?.flashPageSize?.toString(10) || '??'}; sectorSize: ${
                  props.metadata?.flashSectorSize?.toString(10) || '??'
                }; erasedOffset: ${props.metadata?.erasedByteValue || '??'}`}
              ></ListItemText>
            </ListItem>
          </List>
        </Paper>
      </Dialog>
    </>
  );
};
