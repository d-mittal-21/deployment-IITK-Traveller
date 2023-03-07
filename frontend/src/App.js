import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import logo1 from './logo1.svg';

const endPoint = "http://127.0.0.1:8081/interpret"

function App() {
  const [inputCode, setInputCode] = useState("");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");

  const handleRunCode = async () => {
    if(inputCode === ""){
      setOutput("Write some code first!")
      return
    }

    var response = await (await fetch(endPoint, {
      method: 'POST',
      body: new URLSearchParams({
        code: inputCode,
        input: input
      })
    })).json();
    
    console.log(response.output)

    setOutput(response.output);
  }

  const handleRedirect = () =>{
    window.open("https://iitk-traveller-docs.netlify.app/")
  }

  return (
    <div className="App">
      <div className='logoHeader'>
        <img src={logo1} className="logo"></img>
      </div>
      <header className="App-header">
        <div className='codeBlock'>
          <textarea placeholder='Write your code here' spellcheck="false" onChange={(event) => {setInputCode(event.target.value)}}></textarea>
        </div>
        <div className='btns'>
        <button className='btn-three' onClick={handleRunCode}>Run Code</button>
        {/* <button className='btn-three' onClick={window.open("https://iitk-traveller-docs.netlify.app/", "_blank")}>See Documentation</button> */}
        <button className='btn-three' onClick={handleRedirect} >See Documentation</button>
        </div>
        <div className='inputOutput'>
          <textarea placeholder='Write Input if any' spellcheck="false" onChange={(event) => {setInput(event.target.value)}}></textarea>
          <textarea value={output} spellcheck="false" placeholder='Your Code Output'></textarea>
        </div>
      </header>
    </div>
  );
}

export default App;
