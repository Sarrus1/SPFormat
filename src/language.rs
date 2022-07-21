#[cfg(target_arch = "wasm32")]
pub async fn sourcepawn() -> anyhow::Result<tree_sitter::Language> {
    let bytes: &[u8] = include_bytes!("../bins/tree-sitter-sourcepawn.wasm");
    let result = web_tree_sitter_sys::Language::load_bytes(&bytes.into())
        .await
        .map(Into::into)
        .map_err(Into::<tree_sitter::LanguageError>::into)?;
    Ok(result)
}
