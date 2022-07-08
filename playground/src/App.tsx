import { useState, useRef, useEffect } from "react";
import init from "../../pkg/sp_format";
import Editor from "@monaco-editor/react";
import Header from "./components/Header";
import "./App.css";
import { defaultCode } from "./text";

function App() {
  const [code, setCode] = useState(defaultCode);
  const editorRef = useRef(null);

  function handleEditorDidMount(editor: any, monaco: any) {
    editorRef.current = editor;
  }

  function handleEditorChange(value: string | undefined, event: any) {
    if (value !== undefined) {
      setCode(value);
    }
  }

  useEffect(() => {
    init().then((e) => console.log("Formatter instantiated successfully."));
  }, []);

  return (
    <>
      <Header code={code} setCode={setCode} />
      <Editor
        height="100vh"
        theme="vs-dark"
        defaultLanguage="cpp"
        value={code}
        onChange={handleEditorChange}
        onMount={handleEditorDidMount}
      />
    </>
  );
}

export default App;
