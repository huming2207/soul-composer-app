import { AppBar, Toolbar, Typography } from '@mui/material';
import React from 'react';

export const TopBar = (): JSX.Element => {
  return (
    <>
      <AppBar>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Soul Composer
          </Typography>
        </Toolbar>
      </AppBar>
    </>
  );
};
