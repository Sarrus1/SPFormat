mod formatter;
mod language;
mod parser;
pub mod settings;
mod writers;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use formatter::format_string_language;
use settings::Settings;

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
