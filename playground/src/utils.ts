import { Settings } from "./interfaces";

export function makeDefaultSettings(): Settings {
  return {
    breaks_before_function_decl: 2,
    breaks_before_function_def: 2,
    brace_wrapping_before_loop: true,
    brace_wrapping_before_function: true,
  };
}
