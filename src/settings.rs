#[cfg(not(target_arch = "wasm32"))]
use crate::Args;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub breaks_before_function_decl: u32,
    pub breaks_before_function_def: u32,
    pub brace_wrapping_before_function: bool,
    pub brace_wrapping_before_loop: bool,
    pub brace_wrapping_before_condition: bool,
    pub brace_wrapping_before_enum_struct: bool,
    pub brace_wrapping_before_enum: bool,
    pub brace_wrapping_before_typeset: bool,
    pub brace_wrapping_before_funcenum: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_settings_from_args(args: &Args) -> Settings {
    let settings = Settings {
        breaks_before_function_def: args.breaks_before_function_def,
        breaks_before_function_decl: args.breaks_before_function_decl,
        brace_wrapping_before_function: args.brace_wrapping_before_function,
        brace_wrapping_before_loop: args.brace_wrapping_before_loop,
        brace_wrapping_before_condition: args.brace_wrapping_before_condition,
        brace_wrapping_before_enum_struct: args.brace_wrapping_before_enum_struct,
        brace_wrapping_before_enum: args.brace_wrapping_before_enum,
        brace_wrapping_before_typeset: args.brace_wrapping_before_typeset,
        brace_wrapping_before_funcenum: args.brace_wrapping_before_funcenum,
    };

    return settings;
}
