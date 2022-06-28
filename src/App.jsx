
import React, { useState, useEffect } from 'react'
import { appWindow } from '@tauri-apps/api/window'
import { writeFile, readTextFile } from '@tauri-apps/api/fs';
import { open, save } from '@tauri-apps/api/dialog';
import hotkeys from 'hotkeys-js';
import { emit, listen } from '@tauri-apps/api/event'
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
  const [ypadding, setYPadding] = useState(30);
  const [xpadding, setXPadding] = useState(25);
  const [fontSize, setFontSize] = useState(25);
  const [backgroundColor, setBackgroundColor] = useState('#282c34');
  const [color, setColor] = useState('white');
  // const textRef = useRef()
  // const fileRef = useRef({ path: null, name: 'Untitled' })
  const [currentFile, setCurrentFile] = useState({ path: null, name: 'Untitled' });
  const [text, setText] = useState('');

  useEffect(() => {
    async function unlisten() {
      await listen('newFile', msg => {
        console.log(msg)
        let newFile = {
          path: msg.payload.path,
          name: msg.payload.name
        }
        updateFile(newFile);
        updateText(msg.payload.text);
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

  // maybe work on this more
  const saveFile = async () => {
    const { current: { value: text } } = textRef
    if (text.length === 0) return

    if (currentFile.path) {
      console.log('Current file already exists, overwrite')
      let newFile = {
        path: currentFile.path,
        contents: text
      }
      writeFile(newFile)
        .then((result) => {
          console.log('saved successfully')
        }).catch((err) => {
          console.error(err)
        })
    }
    else {
      console.log('CurrentFile path does not exist')
      console.log(currentFile)
      const path = await save({
        title: 'Save Text File',
        // filters: [{ name: 'untitled', extensions: ['txt'] }],
        filters: [{ name: currentFile.name, extensions: ['txt'] }],
        defaultPath: currentFile.path ? currentFile.path : null
      })
      if (!path) return;
      const fileName = new RegExp(/([^\/]+)$/).exec(path)[0];
      const newFile = {
        path,
        contents: text
      }
      writeFile(newFile)
        .then((result) => {
          setCurrentFile({ path: path, name: fileName })

          console.log('saved successfully')
        }).catch((err) => {
          console.error(err)
        })
    }

  }

  const openFile = async () => {
    const path = await open({
      title: 'Open Text File',
    })
    if (!path) return;
    const fileName = new RegExp(/([^\/]+)$/).exec(path)[0];
    const text = await readTextFile(path)
    console.log(path);
    console.log(fileName)
    console.log(text)
    setCurrentFile({ path: path, name: fileName })
    textRef.current.value = text
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
      setTitle(file.name);
      setCurrentFile(file);
    })
      .catch(err => {
        console.log('error');
        console.log(err);
      })
  }

  return (
    <div>
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
    </div>
  )
}

export default App
