import { useState, useRef, useEffect } from "react";
import init, { sp_format } from "../../pkg/sp_format";
import Editor from "@monaco-editor/react";
import "./App.css";

function App() {
  const [code, setCode] = useState("// Your code here");
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
      <button
        onClick={(e) => {
          sp_format(code)
            .then((res) => setCode(res))
            .catch((err) => console.log(err));
        }}
      >
        Format Code
      </button>
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
