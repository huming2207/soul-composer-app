import React from 'react';
import ReactDOM from 'react-dom';
import { App } from './App';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import { invoke } from '@tauri-apps/api/tauri';

const app = document.getElementById('app');
ReactDOM.render(<App />, app);

setInterval(() => {
  invoke('detect_device')
    .then((ret) => {
      console.log(ret);
    })
    .catch((err) => {
      console.error(err);
    });
}, 3000);
