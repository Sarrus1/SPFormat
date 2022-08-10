use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    expressions::write_old_type,
    functions::{write_argument_declarations, write_function_visibility},
    prev_sibling_kind,
    statements::{write_block, write_statement},
    variables::write_type,
    write_dimension, write_node, Writer,
};

pub fn write_alias_declaration(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_decl).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_")
        && prev_kind != ""
        && prev_kind != "comment"
        && prev_kind != "alias_declaration"
    {
        // Insert new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "function_visibility" => write_function_visibility(child, writer)?,
            "type" => write_type(child, writer, true)?,
            "old_type" => write_old_type(child, writer)?,
            "dimension" => write_dimension(child, writer)?,
            "alias_operator" | "operator" => write_node(&child, writer)?,
            "argument_declarations" => write_argument_declarations(child, writer)?,
            "block" => {
                if writer.settings.brace_wrapping_before_function {
                    writer.breakl();
                    write_block(child, writer, true)?;
                } else {
                    writer.output.push(' ');
                    write_block(child, writer, false)?;
                }
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    write_statement(child, writer, false, false)?;
                } else {
                    println!("Unexpected kind {} in write_alias_declaration.", kind);
                }
            }
        }
    }
    writer.breakl();

    Ok(())
}

pub fn write_alias_assignment(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_")
        && prev_kind != ""
        && prev_kind != "comment"
        && prev_kind != "alias_assignment"
    {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "function_definition_type" => write_function_visibility(child, writer)?,
            "type" => write_type(child, writer, true)?,
            "old_type" => write_old_type(child, writer)?,
            "symbol" => write_node(&child, writer)?,
            "dimension" => write_dimension(child, writer)?,
            "=" => writer.output.push_str(" = "),
            "alias_operator" | "operator" => write_node(&child, writer)?,
            "argument_declarations" => write_argument_declarations(child, writer)?,
            ";" => writer.output.push(';'),
            _ => {
                if writer.is_statement(kind.to_string()) {
                    write_statement(child, writer, false, false)?;
                } else {
                    println!("Unexpected kind {} in write_alias_declaration.", kind);
                }
            }
        }
    }
    writer.breakl();

    Ok(())
}
