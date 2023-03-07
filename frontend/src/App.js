import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import logo1 from './logo1.svg';

const endPoint = "POST_api_url"

function App() {
  const [inputCode, setInputCode] = useState("");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");

  const handleRunCode = async () => {
    var response = await (await fetch(endPoint, {
      method: 'POST',
      body: JSON.stringify({
        code: inputCode,
        input: input
      })
    })).json();
    
    setOutput(response);
  }

  return (
    <div className="App">
      <div className='logoHeader'>
        <img src={logo1} className="logo"></img>
      </div>
      <header className="App-header">
        <div className='codeBlock'>
          <textarea placeholder='Write your code here' onChange={(event) => {setInputCode(event.target.value)}}></textarea>
        </div>
        <div className='btns'>
        <button className='btn-three' onClick={handleRunCode}>Run Code</button>
        {/* <button className='btn-three' onClick={window.open("https://iitk-traveller-docs.netlify.app/", "_blank")}>See Documentation</button> */}
        <button className='btn-three'>See Documentation</button>
        </div>
        <div className='inputOutput'>
          <textarea placeholder='Write Input if any' onChange={(event) => {setInput(event.target.value)}}></textarea>
          <textarea placeholder='Your Code Output'>{output}</textarea>
        </div>
      </header>
    </div>
  );
}

export default App;
