import { OutlinedTextFieldProps, TextField } from "@material-ui/core";
import React from "react";
import { Settings } from "../interfaces";

interface SettingsPanelProps {
  settings: Settings;
  setSettings: React.Dispatch<React.SetStateAction<Settings>>;
}

interface SettingRowProps {
  name: string;
  onChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  defaultValue: number;
}

function SettingRow(props: SettingRowProps) {
  return (
    <>
      <span
        style={{
          float: "left",
          display: "inline-block",
          verticalAlign: "middle",
          lineHeight: "normal",
        }}
      >
        {props.name}
      </span>
      <TextField
        inputProps={{ inputMode: "numeric", pattern: "[0-9]*" }}
        style={{ float: "right" }}
        defaultValue={props.defaultValue}
        variant="outlined"
        size="small"
        onChange={props.onChange}
      />
      <div style={{ clear: "both" }}></div>
    </>
  );
}

function SettingsPanel(props: SettingsPanelProps) {
  return (
    <div style={{ margin: "1rem", textAlign: "center" }}>
      <SettingRow
        name="Breaks before function declaration"
        onChange={(e) => {
          props.settings.breaks_before_function_decl = Number(e.target.value);
        }}
        defaultValue={2}
      />
    </div>
  );
}

export default SettingsPanel;
