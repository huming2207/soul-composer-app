import { Box, Container, Tab, Tabs } from '@mui/material';
import React, { useState } from 'react';
import { ConfigView } from './configView';
import { DeviceInfoView } from './deviceInfoView';
import { FirmwareView } from './firmwareView';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`simple-tabpanel-${index}`}
      aria-labelledby={`simple-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `simple-tab-${index}`,
    'aria-controls': `simple-tabpanel-${index}`,
  };
}

export const MainView = (): JSX.Element => {
  const [value, setValue] = useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <>
      <Container>
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs value={value} onChange={handleChange} aria-label="basic tabs example">
            <Tab label="Device Info" {...a11yProps(0)} />
            <Tab label="Configuration" {...a11yProps(1)} />
            <Tab label="Firmware Image" {...a11yProps(2)} />
          </Tabs>
        </Box>
        <TabPanel value={value} index={0}>
          <DeviceInfoView />
        </TabPanel>
        <TabPanel value={value} index={1}>
          <ConfigView />
        </TabPanel>
        <TabPanel value={value} index={2}>
          <FirmwareView />
        </TabPanel>
      </Container>
    </>
  );
};
