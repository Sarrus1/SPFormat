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
pub fn break_after_statement(node: &Node, writer: &mut Writer) {
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

    break_after_statement(&node, writer);

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

    break_after_statement(&node, writer);

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

    break_after_statement(&node, writer);

    Ok(())
}

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
            "#endif" | "#endinput" | "#else" | "symbol" => write_node(&child, writer)?,
            "comment" => write_comment(child, writer)?,
            _ => println!("Unexpected kind {} in write_preproc_generic.", kind),
        }
    }

    break_after_statement(&node, writer);

    Ok(())
}

pub fn write_preproc_symbol(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let kind = node.kind();
    match kind.borrow() {
        "preproc_endif" | "preproc_else" | "preproc_endinput" | "symbol" => {
            write_node(&node, writer)?
        }
        _ => println!("Unexpected kind {} in write_preproc_symbol.", kind),
    }

    break_after_statement(&node, writer);

    Ok(())
}

fn write_preproc_binary_expression(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_preproc_expression(&node.child_by_field_name("left").unwrap(), writer)?;
    writer.output.push(' ');
    write_node(&node.child_by_field_name("operator").unwrap(), writer)?;
    writer.output.push(' ');
    write_preproc_expression(&node.child_by_field_name("right").unwrap(), writer)?;

    Ok(())
}

fn write_preproc_unary_expression(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_node(&node.child_by_field_name("operator").unwrap(), writer)?;
    write_preproc_expression(&node.child_by_field_name("argument").unwrap(), writer)?;

    Ok(())
}

fn write_preproc_parenthesized_expression(
    node: &Node,
    writer: &mut Writer,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "(" | ")" => write_node(&child, writer)?,
            _ => {
                if writer.is_preproc_expression(kind.to_string()) {
                    write_preproc_expression(&child, writer)?;
                } else {
                    println!(
                        "Unexpected kind {} in write_preproc_parenthesized_expression.",
                        kind
                    );
                }
            }
        }
    }

    Ok(())
}

fn write_preproc_expression(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "preproc_binary_expression" => write_preproc_binary_expression(&child, writer)?,
            "preproc_unary_expression" => write_preproc_unary_expression(&child, writer)?,
            "preproc_parenthesized_expression" => {
                write_preproc_parenthesized_expression(&child, writer)?
            }
            "preproc_defined_condition" => write_preproc_defined_condition(&child, writer)?,
            "symbol" | "null" | "this" | "int_literal " | "bool_literal" | "char_literal"
            | "float_literal" | "string_literal" => write_node(&node, writer)?,
            _ => println!("Unexpected kind {} in write_preproc_expression.", kind),
        }
    }

    Ok(())
}

fn write_preproc_defined_condition(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "defined" => writer.output.push_str("defined "),
            "symbol" => write_node(&child, writer)?,
            _ => println!(
                "Unexpected kind {} in write_preproc_defined_condition.",
                kind
            ),
        }
    }

    Ok(())
}

fn write_preproc_arg(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let args = node.utf8_text(writer.source)?;
    writer.output.push_str(args.trim());

    Ok(())
}
