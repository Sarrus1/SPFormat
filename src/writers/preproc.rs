use super::{next_sibling_kind, next_sibling_start, write_comment, write_node, Writer};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

/// Check if there is an inline comment after the statement and don't
/// insert a line break if there is. Otherwise, insert a line break, and
/// an additional one if the following statement is there is more than one
/// empty row.
///
/// # Arguments
///
/// * `node`   - The node which was written.
/// * `writer` - The writer object.
pub fn insert_break(node: &Node, writer: &mut Writer) {
    let next_kind = next_sibling_kind(&node);
    if next_kind == "" {
        // No next sibling, add a break and return.
        writer.breakl();
        return;
    }

    let next_row = next_sibling_start(&node).unwrap().row();

    // If the next sibling is an inline comment, make sure it is on
    // the same line to avoid an unnecessary break.
    if next_kind == "comment" && next_row == node.end_position().row() {
        return;
    }

    // Insert a line break no matter what,
    // consecutive includes cannot be on the same line.
    writer.breakl();

    // Check if the next sibling is right after this node.
    // If it's not, limit the amount of empty rows to 1.
    if next_row - node.end_position().row() > 1 {
        writer.breakl();
    }
}

/// Write a preprocessor include.
///
/// # Arguments
///
/// * `node`   - The preprocessor include node to write.
/// * `writer` - The writer object.
pub fn write_preproc_include(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "#include" | "#tryinclude" => {
                write_node(&child, writer)?;
                writer.output.push(' ')
            }
            "string_literal" | "system_lib_string" => write_node(&child, writer)?,
            _ => println!("Unexpected kind {} in write_preproc_include.", kind),
        }
    }

    insert_break(&node, writer);

    Ok(())
}

/// Write a preprocessor define or macro.
///
/// # Arguments
///
/// * `node`   - The preprocessor define/macro node to write.
/// * `writer` - The writer object.
pub fn write_preproc_define(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "#define" => writer.output.push_str("#define "),
            "symbol" => write_node(&child, writer)?,
            "preproc_arg" => {
                writer.output.push(' ');
                write_preproc_arg(&child, writer)?;
            }
            "," => writer.output.push_str(", "),
            "(" | ")" => write_node(&child, writer)?,
            "macro_param" => write_node(&child, writer)?,
            _ => println!("Unexpected kind {} in write_preproc_define.", kind),
        }
    }

    insert_break(&node, writer);

    Ok(())
}

/// Write a preprocessor undef.
///
/// # Arguments
///
/// * `node`   - The preprocessor undef node to write.
/// * `writer` - The writer object.
pub fn write_preproc_undefine(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "symbol" => write_node(&child, writer)?,
            "#undef" => writer.output.push_str("#undef "),
            _ => println!("Unexpected kind {} in write_preproc_undefine.", kind),
        }
    }

    insert_break(&node, writer);

    Ok(())
}

/// Write a preprocessor generic:
/// * `#if`
/// * `#elseif`
/// * `#error`
/// * `#warning`
/// * `#pragma`
/// * `#assert`
///
/// # Arguments
///
/// * `node`   - The preprocessor generic node to write.
/// * `writer` - The writer object.
pub fn write_preproc_generic(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "#if" | "#elseif" | "#error" | "#warning" | "#pragma" | "#assert" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "preproc_arg" => write_preproc_arg(&child, writer)?,
            "comment" => write_comment(&child, writer)?,
            _ => println!("Unexpected kind {} in write_preproc_generic.", kind),
        }
    }

    insert_break(&node, writer);

    Ok(())
}

/// Write a preprocessor symbol:
/// * `#else`
/// * `#endif`
/// * `#endinput`
/// * `<symbol>`
///
/// # Arguments
///
/// * `node`   - The preprocessor symbol node to write.
/// * `writer` - The writer object.
pub fn write_preproc_symbol(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let kind = node.kind();
    match kind.borrow() {
        "preproc_endif" | "preproc_else" | "preproc_endinput" | "symbol" => {
            write_node(&node, writer)?
        }
        _ => println!("Unexpected kind {} in write_preproc_symbol.", kind),
    }

    insert_break(&node, writer);

    Ok(())
}

/// Write a preprocessor arguments node by trimming it.
///
/// # Arguments
///
/// * `node`   - The preprocessor symbol node to write.
/// * `writer` - The writer object.
fn write_preproc_arg(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let args = node.utf8_text(writer.source)?;
    writer.output.push_str(args.trim());

    Ok(())
}
