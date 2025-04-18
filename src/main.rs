#[cfg(not(target_arch = "wasm32"))]
use std::{fs, str::Utf8Error};

use clap::Parser;

use sp_format::format_string;
use sp_format::settings::Settings;

#[cfg(not(target_arch = "wasm32"))]
/// A tool to format SourcePawn code (new AND old syntaxes).
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The file to format.
    #[clap(value_parser)]
    file: String,

    /// Number of empty lines to insert before a function declaration.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_function_decl: u32,

    /// Number of empty lines to insert before a function definition.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_function_def: u32,

    /// Number of empty lines to insert before an enum declaration.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_enum: u32,

    /// Number of empty lines to insert before an enum struct declaration.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_enum_struct: u32,

    /// Number of empty lines to insert before a methodmap declaration.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_methodmap: u32,

    /// Whether or not to break before a function declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_function: bool,

    /// Whether or not to break before a loop statement brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_loop: bool,

    /// Whether or not to break before a condition statement brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_condition: bool,

    /// Whether or not to break before an enum struct declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_enum_struct: bool,

    /// Whether or not to break before an enum declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_enum: bool,

    /// Whether or not to break before a typeset declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_typeset: bool,

    /// Whether or not to break before a funcenum declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_funcenum: bool,

    /// Whether or not to break before a methodmap declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_methodmap: bool,

    /// Whether or not to break before a methodmap property declaration brace.
    #[clap(long, value_parser, default_value_t = true)]
    brace_wrapping_before_methodmap_property: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_settings_from_args(args: &Args) -> Settings {
    let settings = Settings {
        breaks_before_function_def: args.breaks_before_function_def,
        breaks_before_function_decl: args.breaks_before_function_decl,
        breaks_before_enum: args.breaks_before_enum,
        breaks_before_enum_struct: args.breaks_before_enum_struct,
        breaks_before_methodmap: args.breaks_before_methodmap,
        brace_wrapping_before_function: args.brace_wrapping_before_function,
        brace_wrapping_before_loop: args.brace_wrapping_before_loop,
        brace_wrapping_before_condition: args.brace_wrapping_before_condition,
        brace_wrapping_before_enum_struct: args.brace_wrapping_before_enum_struct,
        brace_wrapping_before_enum: args.brace_wrapping_before_enum,
        brace_wrapping_before_typeset: args.brace_wrapping_before_typeset,
        brace_wrapping_before_funcenum: args.brace_wrapping_before_funcenum,
        brace_wrapping_before_methodmap: args.brace_wrapping_before_methodmap,
        brace_wrapping_before_methodmap_property: args.brace_wrapping_before_methodmap_property,
    };

    return settings;
}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Utf8Error> {
    let args = Args::parse();

    let settings = build_settings_from_args(&args);
    let filename = args.file;
    let source =
        fs::read_to_string(&filename).expect("Something went wrong while reading the file.");
    let output = format_string(&source, settings).unwrap();
    if output.len() == 0 && source.trim().len() > 0 {
        // An error occured, don't write to the file.
        return Ok(());
    }
    fs::write(&filename, output).expect("Something went wrong while writing the file.");
    Ok(())
}
