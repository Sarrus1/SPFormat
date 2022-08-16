import { Settings } from "./interfaces";

export function makeDefaultSettings(): Settings {
  return {
    breaks_before_function_decl: 2,
    breaks_before_function_def: 2,
    breaks_before_enum: 2,
    breaks_before_enum_struct: 2,
    breaks_before_methodmap: 2,
    brace_wrapping_before_loop: true,
    brace_wrapping_before_function: true,
    brace_wrapping_before_condition: true,
    brace_wrapping_before_enum_struct: true,
    brace_wrapping_before_enum: true,
    brace_wrapping_before_typeset: true,
    brace_wrapping_before_funcenum: true,
    brace_wrapping_before_methodmap: true,
    brace_wrapping_before_methodmap_property: true,
  };
}
