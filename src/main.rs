mod language;
mod parser;
mod writers;

use std::borrow::Borrow;
#[cfg(target_arch = "wasm32")]
use std::{borrow::Borrow, str::Utf8Error};
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
    let mut cursor = parsed.walk();
    let mut formatter = writers::Writer {
        output: String::new(),
        source: input.as_bytes(),
        language: &language,
        indent: 0,
        indent_string: "\t".to_string(),
    };
    for node in parsed.root_node().children(&mut cursor) {
        match node.kind().borrow() {
            "global_variable_declaration" => writers::write_global_variable(node, &mut formatter)?,
            _ => formatter
                .output
                .push_str(writers::utf8_text(node, formatter.source)?.borrow()),
        };
    }
    Ok(formatter.output)
}
