import { useState, useRef } from "react";

import Editor from "@monaco-editor/react";
import "./App.css";

function App() {
  const [code, setCode] = useState("// Your code here");
  const editorRef = useRef(null);

  function handleEditorDidMount(editor: any, monaco: any) {
    editorRef.current = editor;
  }

  function formatValue() {
    alert("Test");
  }

  function handleEditorChange(value: string | undefined, event: any) {
    if (value !== undefined) {
      setCode(value);
    }
  }

  return (
    <>
      <button onClick={formatValue}>Format Code</button>
      <Editor
        height="100vh"
        theme="vs-dark"
        defaultLanguage="cpp"
        onChange={handleEditorChange}
        onMount={handleEditorDidMount}
      />
    </>
  );
}

export default App;
