
import React, { useState, useEffect } from 'react';
import { appWindow } from '@tauri-apps/api/window';
import './App.css'

const selectedWindow = appWindow.label;
const windowMap = {
  [selectedWindow]: appWindow
}

// TODO: Have a list of settings in here for notepad config
function App() {

    const [fontSize, setFontSize] = useState(12);
    const [backgroundColor, setBackgroundColor] = useState('#282c34')
    const [fontColor, setFontColor] = useState('white');

    useEffect(() => {
        appWindow.setTitle('Preferences')
    }, [])

    return (
        <div id='preferences'>
            <div className='input-group'>
                <label>Font Color:</label>
                <select value={fontColor} onChange={setFontColor}>
                    <option value={'black'}>Black</option>
                    <option value={'white'}>White</option>
                    <option value={'darkgreen'}>Dark Green</option>
                    <option value={'pink'}>Pink</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Background Color:</label>
                <select value={backgroundColor} onChange={setBackgroundColor}>
                    <option value={'black'}>Black</option>
                    <option value={'white'}>White</option>
                    <option value={'darkgreen'}>Dark Green</option>
                    <option value={'pink'}>Pink</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Font Size:</label>
                <select value={fontSize} onChange={setFontSize}>
                    <option value={8}>8</option>
                    <option value={12}>12</option>
                    <option value={14}>14</option>
                    <option value={16}>16</option>
                </select>
            </div>
        </div>
    )
}

export default App;