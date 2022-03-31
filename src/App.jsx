import { useState, useEffect } from 'react'
import { appWindow } from '@tauri-apps/api/window'
import { writeFile, readTextFile } from '@tauri-apps/api/fs';
import { open, save } from '@tauri-apps/api/dialog';
import hotkeys from "hotkeys-js";
import './App.css'

const OPEN_FILE_HOTKEY = 'ctrl+o, command+o';
const SAVE_FILE_HOTKEY = 'ctrl+s, command+s';

let selectedWindow = appWindow.label;
const windowMap = {
  [selectedWindow]: appWindow
}

const App = () => {
  const [ypadding, setYPadding] = useState(30);
  const [xpadding, setXPadding] = useState(25);
  const [fontSize, setFontSize] = useState(25);
  const [backgroundColor, setBackgroundColor] = useState('#282c34')
  const [color, setColor] = useState('white')
  const [text, setText] = useState('')
  const [currentFile, setCurrentFile] = useState({ path: null, name: 'Untitled' })

  const registerHotkeys = () => {
    // enable hotkeys for input/textarea
    // this runs every time a hotkey is pressed. We want to allow all so return true
    hotkeys.filter = function (event) {
      // console.log('hotkeys filter')
      // console.log(event);
      return true;
    }
    hotkeys(OPEN_FILE_HOTKEY, openFile)
    hotkeys(SAVE_FILE_HOTKEY, saveFile)
  }

  const setTitle = (title) => {
    windowMap[selectedWindow].setTitle(title)
  }

  const saveFile = async () => {
    let path = await save({
      title: 'Save Text File',
      filters: [{ name: currentFile.name, extensions: ['txt'] }],
      defaultPath: currentFile.path ? currentFile.path : null
    })
    console.log(path)
    if (!path) return;
    let fileName = path.split('/').find(item => item.includes('.'))
    let newFile = {
      path,
      text
    }
    writeFile(newFile)
      .then(() => {
        console.log("Wrote file successfully");
        setCurrentFile({ path, name: fileName })

      })
      .catch(err => {
        console.error(err);
      })

  }

  const openFile = async () => {
    // check if there is any text, if so ask the user if they want to save the current file
    let path = await open({
      title: 'Open Text File',
    })
    console.log(path)
    if (!path) return;
    let fileName = path.split('/').find(item => item.includes('.'))
    let text = await readTextFile(path)
    setCurrentFile({ path, name: fileName })
    setText(text)
  }

  useEffect(() => {
    setTitle(currentFile.name)
  }, [currentFile])

  useEffect(() => {
    registerHotkeys();
  }, [])

  return (
    <div>
      <textarea
        className="paper"
        style={{
          color,
          backgroundColor,
          fontSize: `${fontSize}px`,
          padding: `${ypadding}px ${xpadding}px`,
        }}
        autoFocus={true}
        value={text}
        onChange={(e) => setText(e.target.value)}
      >
      </textarea>
    </div>
  )
}

// use styles here later on? maybe

export default App
