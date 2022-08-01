import { Settings } from "./interfaces";

export function makeDefaultSettings(): Settings {
  return {
    breaks_before_function_decl: 2,
    breaks_before_function_def: 2,
    loop_break_before_braces: true,
    function_break_before_braces: true,
  };
}
