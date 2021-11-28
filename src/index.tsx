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
