import { TextField } from "@material-ui/core";
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
    <div className="flex items-center grid grid-cols-2 gap-4 mb-1">
      <span>{props.name}</span>
      <div>
        <TextField
          className="float-right"
          inputProps={{ inputMode: "numeric", pattern: "[0-9]*" }}
          defaultValue={props.defaultValue}
          variant="outlined"
          size="small"
          style={{ maxWidth: "10rem" }}
          onChange={props.onChange}
        />
      </div>
    </div>
  );
}

function SettingsPanel(props: SettingsPanelProps) {
  return (
    <div style={{ margin: "1rem" }}>
      <SettingRow
        name="Breaks before function declaration"
        onChange={(e) => {
          props.settings.breaks_before_function_decl = Number(e.target.value);
        }}
        defaultValue={2}
      />
      <SettingRow
        name="Breaks before function definition"
        onChange={(e) => {
          props.settings.breaks_before_function_def = Number(e.target.value);
        }}
        defaultValue={2}
      />
    </div>
  );
}

export default SettingsPanel;
