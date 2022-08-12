use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    functions::write_argument_declarations, next_sibling_kind, prev_sibling_kind,
    variables::write_type, write_comment, write_dimension, write_fixed_dimension, write_node,
    Writer,
};

pub fn write_typedef(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
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
            "typedef" => writer.output.push_str("typedef "),
            "symbol" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            "typedef_expression" => write_typedef_expression(child, writer)?,
            ";" => continue,
            _ => {
                println!("Unexpected kind {} in write_typedef.", kind);
            }
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

pub fn write_typeset(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
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
            "typeset" => writer.output.push_str("typeset "),
            "symbol" => write_node(&child, writer)?,
            "{" => {
                if writer.settings.brace_wrapping_before_typeset {
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
            "typedef_expression" => {
                let next_kind = next_sibling_kind(&child);
                write_typedef_expression(child, writer)?;
                writer.output.push(';');

                if next_kind != "" {
                    writer.breakl();
                }
            }
            "comment" => write_comment(child, writer)?,
            ";" => continue,
            _ => {
                println!("Unexpected kind {} in write_typeset.", kind);
            }
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

fn write_typedef_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "function" => writer.output.push_str("function "),
            "type" => write_type(child, writer)?,
            "dimension" => write_dimension(child, writer)?,
            "fixed_dimension" => {
                let next_kind = next_sibling_kind(&child);
                write_fixed_dimension(child, writer)?;
                if next_kind != "dimension" || next_kind != "fixed_dimension" {
                    writer.output.push(' ')
                };
            }
            "argument_declarations" => write_argument_declarations(child, writer)?,
            "(" | ")" => continue,
            _ => {
                println!("Unexpected kind {} in write_typedef_expression.", kind);
            }
        }
    }

    Ok(())
}
