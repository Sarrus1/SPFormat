use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub breaks_before_function_decl: u32,
    pub breaks_before_function_def: u32,
    pub breaks_before_enum: u32,
    pub breaks_before_enum_struct: u32,
    pub breaks_before_methodmap: u32,
    pub brace_wrapping_before_function: bool,
    pub brace_wrapping_before_loop: bool,
    pub brace_wrapping_before_condition: bool,
    pub brace_wrapping_before_enum_struct: bool,
    pub brace_wrapping_before_enum: bool,
    pub brace_wrapping_before_typeset: bool,
    pub brace_wrapping_before_funcenum: bool,
    pub brace_wrapping_before_methodmap: bool,
    pub brace_wrapping_before_methodmap_property: bool,
}
