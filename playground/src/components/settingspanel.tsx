import { TextField, Switch, FormGroup, FormControlLabel } from "@mui/material";
import React from "react";
import { Settings } from "../interfaces";

interface SettingsPanelProps {
  settings: Settings;
  setSettings: React.Dispatch<React.SetStateAction<Settings>>;
}

interface SettingRowBoolProps {
  name: string;
  onChange: (
    event: React.ChangeEvent<HTMLInputElement>,
    checked: boolean
  ) => void;
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
    <div
      className="items-center grid gap-4 mb-1"
      style={{ gridTemplateColumns: "1fr 15rem" }}
    >
      <span className="font-roboto font-bold">{props.name}</span>
      <TextField
        className="col-span-1"
        inputProps={{ inputMode: "numeric", pattern: "[0-9]*" }}
        defaultValue={props.defaultValue}
        variant="outlined"
        size="small"
        onChange={props.onChange}
      />
    </div>
  );
}

function SettingRowBool(props: SettingRowBoolProps) {
  return (
    <FormGroup>
      <FormControlLabel
        control={<Switch defaultChecked onChange={props.onChange} />}
        label={props.name}
      />
    </FormGroup>
  );
}

function BraceWrappingRow(props: SettingsPanelProps) {
  return (
    <div
      className="grid grid-cols-2 gap-4 mb-1 mt-2"
      style={{ gridTemplateColumns: "1fr 15rem" }}
    >
      <span className="col-span-1 font-roboto font-bold">Brace Wrapping</span>
      <fieldset>
        <p className="font-roboto font-bold">Brace Wrapping</p>
        <SettingRowBool
          name="Before function braces"
          onChange={(e) => {
            const old = props.settings.brace_wrapping_before_function;
            props.settings.brace_wrapping_before_function = !old;
          }}
        />
        <SettingRowBool
          name="Before loop braces"
          onChange={(e) => {
            const old = props.settings.brace_wrapping_before_loop;
            props.settings.brace_wrapping_before_loop = !old;
          }}
        />
      </fieldset>
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
      <BraceWrappingRow {...props} />
    </div>
  );
}

export default SettingsPanel;
