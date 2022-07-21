
import React, { useState, useEffect } from 'react';
import { appWindow } from '@tauri-apps/api/window';
import { emit } from '@tauri-apps/api/event';
import { getKeyVal, setKeyVal } from '../../utils/store';
import { FONT_COLOR, FONT_SIZE, BACKGROUND_COLOR, FONT_FAMILY } from '../../utils/keys';
import { invoke } from '@tauri-apps/api';
import './Preferences.css'

function Preferences() {
    const [size, setSize] = useState(0);
    const [fontSize, setFontSize] = useState(12);
    const [fontFamily, setFontFamily] = useState("");
    const [fontFamilies, setFontFamilies] = useState([])
    const [fontColor, setFontColor] = useState('white');
    const [backgroundColor, setBackgroundColor] = useState('#282c34')

    useEffect(() => {
        getPreferences()
    }, [])

    const getPreferences = async () => {
        console.log('Get Preferences');
        appWindow.setTitle('Preferences')
        let fontSize = await getKeyVal(FONT_SIZE);
        let fontColor = await getKeyVal(FONT_COLOR);
        let backgroundColor = await getKeyVal(BACKGROUND_COLOR);
        let fontFamily = await getKeyVal(FONT_FAMILY);
        // invoke to get the fonts we have already in the app directory
        let fontFamilies = await invoke('get_fonts');

        console.log('FontSize:', fontSize);
        console.log('FontColor:', fontColor);
        console.log('BackgroundColor:', backgroundColor);
        console.log('FontFamily:', fontFamily);
        console.log('FontFamilies:', fontFamilies);

        if (fontSize) setFontSize(fontSize);
        if (fontColor) setFontColor(fontColor);
        if (backgroundColor) setBackgroundColor(backgroundColor);
        if (fontFamily) setFontFamily(fontFamily);
        if (fontFamilies) setFontFamilies(fontFamilies);
    }

    const handleChange = async (e) => {
        const { name, value } = e.target;
        switch (name) {
            case FONT_SIZE:
                setFontSize(value);
                await setKeyVal(FONT_SIZE, value);
                // emit to main app
                emit("update-setting", FONT_SIZE)
                break;
            case FONT_COLOR:
                setFontColor(value);
                await setKeyVal(FONT_COLOR, value);
                // emit to main app
                emit("update-setting", FONT_COLOR)
                break;
            case BACKGROUND_COLOR:
                setBackgroundColor(value);
                await setKeyVal(BACKGROUND_COLOR, value);
                // emit to main app
                emit("update-setting", BACKGROUND_COLOR)
                break;
            case FONT_FAMILY:
                setSize(0);
                setFontFamily(value);
                await setKeyVal(FONT_FAMILY, value)
                emit("update-setting", FONT_FAMILY)
                break;
            default:
                break;
        }
    }

    return (
        <div id='preferences'>
            <div className='input-group'>
                <label>Font Size:</label>
                <select name={FONT_SIZE} value={fontSize} onChange={handleChange}>
                    <option value={8}>8</option>
                    <option value={12}>12</option>
                    <option value={14}>14</option>
                    <option value={16}>16</option>
                    <option value={20}>20</option>
                    <option value={24}>24</option>
                    <option value={28}>28</option>
                    <option value={32}>32</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Font Color:</label>
                <select name={FONT_COLOR} value={fontColor} onChange={handleChange}>
                    <option value='black'>Black</option>
                    <option value='white'>White</option>
                    <option value='darkgreen'>Dark Green</option>
                    <option value='pink'>Pink</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Background Color:</label>
                <select name={BACKGROUND_COLOR} value={backgroundColor} onChange={handleChange}>
                    <option value='black'>Black</option>
                    <option value='#282c34'>Deep Purple</option>
                    <option value='white'>White</option>
                    <option value='darkgreen'>Dark Green</option>
                    <option value='pink'>Pink</option>
                </select>
            </div>
            <div className='input-group'>
                <label>Font Family</label>
                <select size={size} name={FONT_FAMILY}
                    onMouseDown={() => setSize(3)}
                    onBlur={() => setSize(0)}
                    value={fontFamily}
                    onChange={handleChange}
                >
                    {fontFamilies.map((family, index) => (
                        <option key={index} style={{ fontFamily: family }}>{family}</option>
                    ))}
                </select>
            </div>

        </div>
    )
}

export default Preferences;