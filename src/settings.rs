#[cfg(not(target_arch = "wasm32"))]
use crate::Args;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub breaks_before_function_decl: u32,
    pub breaks_before_function_def: u32,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_settings_from_args(args: &Args) -> Settings {
    let settings = Settings {
        breaks_before_function_def: args.breaks_before_function_def,
        breaks_before_function_decl: args.breaks_before_function_decl,
    };

    return settings;
}
