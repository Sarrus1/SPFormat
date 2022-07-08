mod language;
mod parser;
mod writers;

use std::borrow::Borrow;
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, str::Utf8Error};

use tree_sitter::Language;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Utf8Error> {
    let filename = "test.sp";
    let source =
        fs::read_to_string(filename).expect("Something went wrong while reading the file.");
    let output = format_string(&source).unwrap();
    fs::write(filename, output).expect("Something went wrong writing the file.");
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn format_string(input: &String) -> anyhow::Result<String> {
    let language = tree_sitter_sourcepawn::language().into();
    let output = format_string_language(&input, language)
        .expect("An error has occured while generating the SourcePawn code.");
    Ok(output)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn sp_format(input: String) -> Result<String, JsValue> {
    let language = language::sourcepawn().await.unwrap();
    let output = format_string_language(&input, language)
        .expect("An error has occured while generating the SourcePawn code.");
    Ok(output)
}

fn format_string_language(input: &String, language: Language) -> anyhow::Result<String> {
    let mut parser = parser::sourcepawn(&language)?;
    let parsed = parser.parse(&input, None)?.unwrap();
    #[cfg(debug_assertions)]
    println!("{}", parsed.root_node().to_sexp());
    let mut cursor = parsed.walk();
    let mut writer = writers::Writer {
        output: String::new(),
        source: input.as_bytes(),
        language: &language,
        indent: 0,
        indent_string: "\t".to_string(),
        skip: 0,
    };
    for node in parsed.root_node().children(&mut cursor) {
        if writer.skip > 0 {
            writer.skip -= 1;
            continue;
        }
        match node.kind().borrow() {
            "global_variable_declaration" => writers::write_global_variable(node, &mut writer)?,
            "preproc_include" | "preproc_tryinclude" => {
                writers::write_preproc_include(node, &mut writer)?
            }
            "preproc_macro" | "preproc_define" => writers::write_preproc_define(node, &mut writer)?,
            "preproc_undefine" => writers::write_preproc_undefine(node, &mut writer)?,
            "preproc_if" | "preproc_endif" | "preproc_else" | "preproc_endinput"
            | "preproc_pragma" => writers::write_preproc_generic(node, &mut writer)?,
            "comment" => writers::write_comment(node, &mut writer)?,
            _ => writer
                .output
                .push_str(writers::utf8_text(node, writer.source)?.borrow()),
        };
    }
    Ok(writer.output)
}
