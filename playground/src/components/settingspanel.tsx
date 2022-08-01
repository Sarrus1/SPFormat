import { TextField, Select, MenuItem, FormControl } from "@material-ui/core";
import React from "react";
import { Settings } from "../interfaces";

interface SettingsPanelProps {
  settings: Settings;
  setSettings: React.Dispatch<React.SetStateAction<Settings>>;
}

interface SettingRowBoolProps {
  name: string;
  onChange: (
    e: React.ChangeEvent<{
      name?: string | undefined;
      value: unknown;
    }>
  ) => void;
  defaultValue: number;
}

interface SettingRowNumericProps {
  name: string;
  onChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  defaultValue: number;
}

function SettingRowNumeric(props: SettingRowNumericProps) {
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

function SettingRowBool(props: SettingRowBoolProps) {
  return (
    <div className="flex items-center grid grid-cols-2 gap-4 mb-1">
      <span>{props.name}</span>
      <FormControl size="small" style={{ maxWidth: "10rem" }}>
        <Select
          className="float-right"
          variant="outlined"
          defaultValue={1}
          onChange={props.onChange}
        >
          <MenuItem value={1}>True</MenuItem>
          <MenuItem value={0}>False</MenuItem>
        </Select>
      </FormControl>
    </div>
  );
}

function SettingsPanel(props: SettingsPanelProps) {
  return (
    <div style={{ margin: "1rem" }}>
      <SettingRowNumeric
        name="Breaks before function declaration"
        onChange={(e) => {
          props.settings.breaks_before_function_decl = Number(e.target.value);
        }}
        defaultValue={2}
      />
      <SettingRowNumeric
        name="Breaks before function definition"
        onChange={(e) => {
          props.settings.breaks_before_function_def = Number(e.target.value);
        }}
        defaultValue={2}
      />
      <SettingRowBool
        name="Break before function braces"
        onChange={(e) => {
          props.settings.function_break_before_braces = Boolean(e.target.value);
        }}
        defaultValue={1}
      />
      <SettingRowBool
        name="Break before loop braces"
        onChange={(e) => {
          props.settings.loop_break_before_braces = Boolean(e.target.value);
        }}
        defaultValue={1}
      />
    </div>
  );
}

export default SettingsPanel;
