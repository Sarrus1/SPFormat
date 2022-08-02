#[cfg(not(target_arch = "wasm32"))]
use crate::Args;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub breaks_before_function_decl: u32,
    pub breaks_before_function_def: u32,
    pub brace_wrapping_before_function: bool,
    pub brace_wrapping_before_loop: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_settings_from_args(args: &Args) -> Settings {
    let settings = Settings {
        breaks_before_function_def: args.breaks_before_function_def,
        breaks_before_function_decl: args.breaks_before_function_decl,
        brace_wrapping_before_function: args.brace_wrapping_before_function,
        brace_wrapping_before_loop: args.brace_wrapping_before_loop,
    };

    return settings;
}
