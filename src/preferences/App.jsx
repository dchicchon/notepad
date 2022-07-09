
import React, { useState, useEffect } from 'react';
import { appWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';
import { getKeyVal, setKeyVal } from '../utils';
import { FONT_COLOR, FONT_SIZE, BACKGROUND_COLOR, STORE_NAME } from '../utils/keys';
import './App.css'


// did this run twice?

// TODO: Have a list of settings in here for notepad config
function App() {

    const [fontSize, setFontSize] = useState(12);
    const [fontColor, setFontColor] = useState('white');
    const [backgroundColor, setBackgroundColor] = useState('#282c34')

    useEffect(() => {
        appWindow.setTitle('Preferences')
        // why does this run twice?
        async function getPreferences() {
            console.log('Get Preferences');
            let fontSize = await getKeyVal(FONT_SIZE);
            let fontColor = await getKeyVal(FONT_COLOR);
            let backgroundColor = await getKeyVal(BACKGROUND_COLOR);
            console.log('FontSize:', fontSize);
            console.log('FontColor:', fontColor);
            console.log('BackgroundColor:', backgroundColor);

            if (fontSize) setFontSize(fontSize);
            if (fontColor) setFontColor(fontColor);
            if (backgroundColor) setBackgroundColor(backgroundColor);

        }
        getPreferences()
    }, [])

    const handleChange = async (e) => {
        const { name, value } = e.target;
        switch (name) {
            case 'fontSize':
                setFontSize(value);
                await setKeyVal(FONT_SIZE, value);
                // emit to main app
                emit("update-setting", FONT_SIZE)
                break;
            case 'fontColor':
                setFontColor(value);
                await setKeyVal(FONT_COLOR, value);
                // emit to main app
                emit("update-setting", FONT_COLOR)
                break;
            case 'backgroundColor':
                setBackgroundColor(value);
                await setKeyVal(BACKGROUND_COLOR, value);
                // emit to main app
                emit("update-setting", BACKGROUND_COLOR)
                break;
        }
    }

    return (
        <div id='preferences'>
            <div className='input-group'>
                <label>Font Size:</label>
                <select name='fontSize' value={fontSize} onChange={handleChange}>
                    <option value={8}>8</option>
                    <option value={12}>12</option>
                    <option value={14}>14</option>
                    <option value={16}>16</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Font Color:</label>
                <select name='fontColor' value={fontColor} onChange={handleChange}>
                    <option value='black'>Black</option>
                    <option value='white'>White</option>
                    <option value='darkgreen'>Dark Green</option>
                    <option value='pink'>Pink</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Background Color:</label>
                <select name='backgroundColor' value={backgroundColor} onChange={handleChange}>
                    <option value='black'>Black</option>
                    <option value='#282c34'>Deep Purple</option>
                    <option value='white'>White</option>
                    <option value='darkgreen'>Dark Green</option>
                    <option value='pink'>Pink</option>
                </select>
            </div>

        </div>
    )
}

export default App;