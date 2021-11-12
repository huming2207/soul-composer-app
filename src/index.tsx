import React from 'react';
import ReactDOM from 'react-dom';
import { App } from './App';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import { invoke } from '@tauri-apps/api/tauri';
import SoulDeviceStateInstance, { ScannedResult } from './models/SoulDevice';

const app = document.getElementById('app');
ReactDOM.render(<App />, app);

setInterval(() => {
  invoke('detect_device')
    .then((ret: any) => {
      console.log(ret);
      SoulDeviceStateInstance.setScanResult(JSON.parse(ret));
    })
    .catch((err) => {
      console.error(err);
    });
}, 3000);

invoke('cdc_open', { invokeMessage: '/dev/ttyACM0' })
  .then((ret) => {
    console.log(ret);
  })
  .catch((err) => {
    console.error(err);
  });

invoke('cdc_get_device_info')
  .then((ret) => {
    console.log(ret);
  })
  .catch((err) => {
    console.error(err);
  });
