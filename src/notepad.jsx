import React from 'react'
import { createRoot } from 'react-dom/client'
import Notepad from './pages/Notepad'

const container = document.getElementById('root')
const root = createRoot(container)

root.render(
  <React.StrictMode>
    <Notepad />
  </React.StrictMode>)