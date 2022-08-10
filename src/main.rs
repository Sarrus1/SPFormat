mod language;
mod parser;
pub mod settings;
mod writers;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, str::Utf8Error};

use settings::Settings;
use tree_sitter::Language;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::writers::source_file::write_source_file;

#[cfg(not(target_arch = "wasm32"))]
/// A tool to format SourcePawn code (new AND old syntaxes).
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The file to format.
    #[clap(short, long, value_parser)]
    file: String,

    /// Number of empty lines to insert before a function declaration.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_function_decl: u32,

    /// Number of empty lines to insert before a function definition.
    #[clap(long, value_parser, default_value_t = 2)]
    breaks_before_function_def: u32,

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

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Utf8Error> {
    let args = Args::parse();

    let settings = settings::build_settings_from_args(&args);
    let filename = args.file;
    let source =
        fs::read_to_string(&filename).expect("Something went wrong while reading the file.");
    let output = format_string(&source, settings).unwrap();
    if output.len() == 0 && source.trim().len() > 0 {
        // An error occured, don't write to the file.
        return Ok(());
    }
    fs::write(&filename, output).expect("Something went wrong while writing the file.");
    println!("Press any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn format_string(input: &String, settings: Settings) -> anyhow::Result<String> {
    let language = tree_sitter_sourcepawn::language().into();
    let output = format_string_language(&input, language, &settings)
        .expect("An error has occured while generating the Sourcepawn code.");

    Ok(output)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn sp_format(input: String, val: JsValue) -> Result<String, JsValue> {
    tree_sitter::TreeSitter::init().await?;
    let language = language::sourcepawn().await.unwrap();
    let settings: Settings = val.into_serde().unwrap();
    let output = format_string_language(&input, language, &settings)
        .expect("An error has occured while generating the SourcePawn code.");
    Ok(output)
}

fn format_string_language(
    input: &String,
    language: Language,
    settings: &Settings,
) -> anyhow::Result<String> {
    let mut parser = parser::sourcepawn(&language)?;
    let parsed = parser.parse(&input, None)?.unwrap();
    if parsed.root_node().has_error() {
        // Do not try to format, there is an error in the syntax.
        return Ok("".to_string());
    }
    #[cfg(debug_assertions)]
    println!("{}", parsed.root_node().to_sexp());
    let mut writer = writers::Writer {
        output: String::new(),
        source: input.as_bytes(),
        language: &language,
        indent: 0,
        indent_string: "\t".to_string(),
        skip: 0,
        settings: settings,
        _statement_kinds: HashSet::new(),
        _expression_kinds: HashSet::new(),
        _literal_kinds: HashSet::new(),
    };
    build_writer(&mut writer);
    write_source_file(parsed.root_node(), &mut writer)?;
    Ok(writer.output)
}

fn build_writer(writer: &mut writers::Writer) {
    let _statement_kinds = vec![
        "block",
        "variable_declaration_statement",
        "old_variable_declaration_statement",
        "for_loop",
        "while_loop",
        "do_while_loop",
        "break_statement",
        "continue_statement",
        "condition_statement",
        "switch_statement",
        "return_statement",
        "delete_statement",
        "expression_statement",
    ];
    for kind in _statement_kinds {
        writer._statement_kinds.insert(kind.to_string());
    }
    let _expression_kinds = vec![
        "assignment_expression",
        "function_call",
        "array_indexed_access",
        "ternary_expression",
        "field_access",
        "scope_access",
        "binary_expression",
        "unary_expression",
        "update_expression",
        "sizeof_expression",
        "view_as",
        "old_type_cast",
        "symbol",
        "parenthesized_expression",
        "this",
        "new_instance",
    ];
    for kind in _expression_kinds {
        writer._expression_kinds.insert(kind.to_string());
    }

    let _literal_kinds = vec![
        "int_literal",
        "float_literal",
        "char_literal",
        "string_literal",
        "concatenated_string",
        "bool_literal",
        "array_literal",
        "null",
    ];
    for kind in _literal_kinds {
        writer._literal_kinds.insert(kind.to_string());
    }
}
