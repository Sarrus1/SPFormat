import { Settings } from "./interfaces";

export function makeDefaultSettings(): Settings {
  return {
    breaks_before_function_decl: 2,
    breaks_before_function_def: 2,
  };
}
