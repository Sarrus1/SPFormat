export interface Settings {
  breaks_before_function_decl: number;
  breaks_before_function_def: number;
  breaks_before_enum: number;
  breaks_before_enum_struct: number;
  breaks_before_methodmap: number;
  brace_wrapping_before_function: boolean;
  brace_wrapping_before_loop: boolean;
  brace_wrapping_before_condition: boolean;
  brace_wrapping_before_enum_struct: boolean;
  brace_wrapping_before_enum: boolean;
  brace_wrapping_before_typeset: boolean;
  brace_wrapping_before_funcenum: boolean;
  brace_wrapping_before_methodmap: boolean;
  brace_wrapping_before_methodmap_property: boolean;
}

export interface HeaderProps {
  readonly code: string;
  settings: Settings;
  setCode: React.Dispatch<React.SetStateAction<string>>;
}

export interface SettingsPanelProps {
  settings: Settings;
  setSettings: React.Dispatch<React.SetStateAction<Settings>>;
}

export interface SettingRowBoolProps {
  name: string;
  onChange: (
    event: React.ChangeEvent<HTMLInputElement>,
    checked: boolean
  ) => void;
}

export interface SettingRowNumericProps {
  name: string;
  onChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  defaultValue: number;
}
