
import React, { useState, useEffect } from 'react'
import { appWindow } from '@tauri-apps/api/window'
import hotkeys from 'hotkeys-js';
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api';

import {
  INCREASE_FONT,
  DECREASE_FONT,
} from './utils/hotkeyMap'
import './App.css'

const selectedWindow = appWindow.label;
const windowMap = {
  [selectedWindow]: appWindow
}

function App() {
  const [ypadding, setYPadding] = useState(25);
  const [xpadding, setXPadding] = useState(25);
  const [fontSize, setFontSize] = useState(25);
  const [backgroundColor, setBackgroundColor] = useState('#282c34');
  const [color, setColor] = useState('white');
  const [currentFile, setCurrentFile] = useState({ path: null, name: 'Untitled' });
  const [text, setText] = useState('');

  useEffect(() => {
    async function unlisten() {
      await listen('state_change', msg => {
        console.log(msg)
        // check all of the items in the msg.
        if (!msg.payload) return;
        if (msg.payload.text) updateText(msg.payload.text);
        if (msg.payload.name) {
          let newFile = {
            path: msg.payload.path,
            name: msg.payload.name
          }
          updateFile(newFile);
        }
      })
    }
    unlisten();
  }, [])

  useEffect(() => {
    registerHotkeys();
    setTitle(currentFile.name)
    return unRegisterHotkeys
  }, [currentFile])

  const unRegisterHotkeys = () => hotkeys.unbind();
  const registerHotkeys = () => {
    // enable hotkeys for input/textarea
    // this runs every time a hotkey is pressed. We want to allow all so return true
    hotkeys.filter = (event) => true
    // hotkeys(OPEN_FILE_HOTKEY, openFile)
    // hotkeys(SAVE_FILE_HOTKEY, saveFile)
    hotkeys(INCREASE_FONT, () => setFontSize(val => val + 1));
    hotkeys(DECREASE_FONT, () => setFontSize(val => val - 1));
  }
  const setTitle = (title) => {
    windowMap[selectedWindow].setTitle(title)
  }

  const updateText = async (text) => {
    invoke('db_insert', {
      key: 'text',
      value: text,
    }).then(response => {
      setText(text);
    })
      .catch(err => {
        console.log('error');
        console.log(err);
      })
  }
  const updateFile = async (file) => {
    invoke('db_insert', {
      key: 'file',
      value: file.path,
    }).then(response => {
      setCurrentFile(file);
    })
      .catch(err => {
        console.log('error');
        console.log(err);
      })
  }

  return (
    <textarea
      className='paper'
      style={{
        color,
        backgroundColor,
        fontSize: `${fontSize}px`,
        padding: `${ypadding}px ${xpadding}px`,
      }}
      value={text}
      onChange={(e) => updateText(e.target.value)}
      autoFocus={true}
    />
  )
}

export default App
