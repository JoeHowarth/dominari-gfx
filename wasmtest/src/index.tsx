import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import init from 'dominari-gfx'

init()

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
