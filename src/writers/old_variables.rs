use super::{
    expressions::{write_expression, write_old_type},
    variables::write_variable_storage_class,
    write_comment, write_dimension, write_fixed_dimension, write_node, Writer,
};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

pub fn write_old_global_variable_declaration(
    node: &Node,
    writer: &mut Writer,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "variable_storage_class" | "variable_visibility" | "new" | "decl" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "comment" => {
                write_comment(&child, writer)?;
            }
            "old_variable_declaration" => write_old_variable_declaration(child, writer)?,
            "," => writer.output.push_str(", "),
            ";" => continue,
            _ => println!(
                "Unexpected kind {} in write_old_global_variable_declaration.",
                kind
            ),
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

pub fn write_old_variable_declaration_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    if do_indent {
        writer.write_indent();
    }

    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "variable_storage_class" => write_variable_storage_class(child, writer)?,
            "new" | "decl" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "old_variable_declaration" => write_old_variable_declaration(child, writer)?,
            "comment" => write_comment(&child, writer)?,
            ";" => writer.output.push(';'),
            "," => writer.output.push_str(", "),
            _ => write_node(&child, writer)?,
        }
    }

    if do_indent {
        if !writer.output.ends_with(';') {
            writer.output.push(';');
        }
        writer.breakl();
    }

    Ok(())
}

fn write_old_variable_declaration(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "old_type" => write_old_type(child, writer)?,
            "dimension" => write_dimension(child, writer, false)?,
            "fixed_dimension" => write_fixed_dimension(child, writer, false)?,
            "symbol" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            _ => {
                if writer.is_expression(&kind) {
                    write_expression(child, writer)?;
                } else {
                    println!(
                        "Unexpected kind {} in write_old_variable_declaration.",
                        kind
                    )
                }
            }
        }
    }

    Ok(())
}
