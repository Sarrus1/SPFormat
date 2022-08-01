import { useState, useRef } from "react";
import init from "../../pkg/sp_format";
import { Settings } from "./interfaces";
import Editor from "@monaco-editor/react";
import Header from "./components/header";
import SettingsPanel from "./components/settingspanel";
import "./App.css";
import { defaultCode } from "./text";
import { makeDefaultSettings } from "./utils";

init();

function App() {
  const [code, setCode] = useState(defaultCode);
  const [settings, setSettings] = useState<Settings>(makeDefaultSettings());

  const editorRef = useRef(null);

  function handleEditorDidMount(editor: any, monaco: any) {
    editorRef.current = editor;
  }

  function handleEditorChange(value: string | undefined, event: any) {
    if (value !== undefined) {
      setCode(value);
    }
  }

  return (
    <div style={{ overflowX: "hidden" }}>
      <Header code={code} settings={settings} setCode={setCode} />
      <div className="row">
        <div className="column">
          <SettingsPanel settings={settings} setSettings={setSettings} />
        </div>
        <div className="column">
          <Editor
            height="100vh"
            width="50vw"
            theme="vs-dark"
            defaultLanguage="cpp"
            value={code}
            onChange={handleEditorChange}
            onMount={handleEditorDidMount}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
