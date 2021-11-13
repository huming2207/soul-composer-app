import { Container } from '@mui/material';
import React from 'react';
import { MainView } from './components/mainView';
import { BottomBar } from './components/bottomBar';

export function App() {
  return (
    <Container>
      <BottomBar />
      <MainView />
    </Container>
  );
}
