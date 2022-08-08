use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    functions::write_argument_declarations, prev_sibling_kind, statements::write_block,
    variables::write_type, write_comment, write_fixed_dimension, write_node, Writer,
};

pub fn write_enum_struct(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
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
            "enum" | "struct" => {
                write_node(child, writer)?;
                writer.output.push(' ')
            }
            "symbol" => write_node(child, writer)?,
            "{" => {
                if writer.settings.brace_wrapping_before_enum_struct {
                    writer.breakl();
                } else {
                    writer.output.push(' ');
                }
                writer.output.push_str("{\n");
                writer.indent += 1;
            }
            "}" => {
                writer.output.push_str("}\n");
                writer.indent -= 1;
            }
            "comment" => write_comment(child, writer)?,
            "enum_struct_field" => write_enum_struct_field(child, writer)?,
            "enum_struct_method" => write_enum_struct_method(child, writer)?,
            _ => {
                println!("Unexpected kind {} in write_enum_struct.", kind);
            }
        }
    }
    writer.breakl();

    Ok(())
}

fn write_enum_struct_field(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_decl).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_")
        && prev_kind != "{"
        && prev_kind != "comment"
        && prev_kind != "enum_struct_field"
    {
        // Insert new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    writer.write_indent();

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "type" => write_type(child, writer)?,
            "symbol" => write_node(child, writer)?,
            "fixed_dimension" => write_fixed_dimension(child, writer)?,
            ";" => write_node(child, writer)?,
            _ => {
                println!("Unexpected kind {} in write_enum_struct_field.", kind);
            }
        }
    }
    writer.breakl();

    Ok(())
}

fn write_enum_struct_method(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_decl).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "{" && prev_kind != "comment" {
        // Insert new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    writer.write_indent();

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "type" => write_type(child, writer)?,
            "symbol" => write_node(child, writer)?,
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
                println!("Unexpected kind {} in write_enum_struct_method.", kind);
            }
        }
    }
    writer.breakl();

    Ok(())
}
