use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    expressions::write_expression, prev_sibling_kind, write_fixed_dimension, write_node, Writer,
};

pub fn write_enum(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_decl).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "enum" => writer.output.push_str("enum "),
            "symbol" | ":" | "(" | ";" => write_node(child, writer)?,
            ")" => writer.output.push_str(") "),
            "enum_entries" => write_enum_entries(child, writer)?,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else if kind.to_string().ends_with('=') {
                    write_node(child, writer)?;
                } else {
                    println!("Unexpected kind {} in write_enum.", kind);
                }
            }
        }
    }
    writer.breakl();

    Ok(())
}

fn write_enum_entries(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "{" => {
                if writer.settings.brace_wrapping_before_enum {
                    writer.breakl();
                } else {
                    writer.output.push(' ');
                }
                writer.output.push_str("{\n");
                writer.indent += 1;
            }
            "}" => {
                writer.output.push_str("}");
                writer.indent -= 1;
            }
            "enum_entry" => write_enum_entry(child, writer)?,
            "," => continue,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else if kind.to_string().ends_with('=') {
                    // Match all in place operators, write it, and add a space
                    // to respect the rest of the styling.
                    write_node(child, writer)?;
                    writer.output.push(' ');
                } else {
                    println!("Unexpected kind {} in write_enum_entries.", kind);
                }
            }
        }
    }

    Ok(())
}

fn write_enum_entry(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "builtin_type" | "symbol" => write_node(child, writer)?,
            ":" => writer.output.push_str(": "),
            "fixed_dimension" => write_fixed_dimension(child, writer)?,
            "=" => writer.output.push_str(" = "),
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    println!("Unexpected kind {} in write_enum_entry.", kind);
                }
            }
        }
    }
    writer.output.push(',');
    writer.breakl();

    Ok(())
}
