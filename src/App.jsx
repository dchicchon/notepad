import { useState, useEffect } from 'react'
import './App.css'

function App() {

  const [ypadding, setYPadding] = useState(30);
  const [xpadding, setXPadding] = useState(25);
  const [fontSize, setFontSize] = useState(25);
  const [backgroundColor, setBackgroundColor] = useState('#282c34')
  const [color, setColor] = useState('white')
  const [text, setText] = useState('')

  useEffect(() => {
    // window.onresize = function() {
    // console.log("Resize")
    // } 
  })

  return (
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
  )
}

// use styles here later on? maybe

export default App
