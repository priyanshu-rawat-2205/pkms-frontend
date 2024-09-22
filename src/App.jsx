import { useState, useEffect } from 'react'
import './App.css'

function App() {
  const [msg, setMsg] = useState([])
  useEffect(() => {
    fetch('http://localhost:3000/')
      .then((res) => {
        return res.json();
      })
      .then((data) => {
        setMsg(data[1].title);
      });
  }, [])
  return (
    <>
      <div>trying to connect ...</div>
      <div>{msg}</div>
    </>
  )
}

export default App
