use super::{
    expressions::{write_expression, write_old_type},
    next_sibling_kind, write_comment, write_dimension, write_dynamic_array, write_fixed_dimension,
    write_node, Writer,
};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

pub fn write_global_variable(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    global_variable_declaration_break(&node, writer)?;

    for sub_node in node.children(&mut cursor) {
        let kind = sub_node.kind();
        match kind.borrow() {
            "variable_storage_class" | "variable_visibility" | "type" => {
                writer
                    .output
                    .push_str(sub_node.utf8_text(writer.source)?.borrow());
                writer.output.push(' ');
            }
            "comment" => {
                write_comment(sub_node, writer)?;
            }
            "variable_declaration" => write_variable_declaration(sub_node, writer)?,
            "," => writer.output.push_str(", "),
            ";" => continue,
            _ => println!("Unexpected kind {} in write_global_variable.", kind),
        }
    }
    let next_node = node.next_sibling();
    if next_node.is_none() {
        writer.output.push_str(";");
        return Ok(());
    }
    let next_node = next_node.unwrap();
    if next_node.kind() == "comment" {
        if next_node.start_position().row() == node.end_position().row() {
            writer.output.push_str(";\t");
            write_comment(next_node, writer)?;
            writer.skip += 1;
        } else {
            writer.output.push_str(";\n\n");
        }
    } else {
        writer.output.push_str(";\n");
    }

    Ok(())
}

fn global_variable_declaration_break(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let prev_node = node.prev_sibling();

    if prev_node.is_none() {
        return Ok(());
    }
    let prev_node = prev_node.unwrap();
    if prev_node.kind() == "comment"
        && prev_node.end_position().row() == node.start_position().row() - 1
    {
        return Ok(());
    }
    if prev_node.kind() != "global_variable_declaration" {
        writer.breakl();
        return Ok(());
    }
    // Don't double next line if same type.
    let var_type = node
        .child_by_field_name("type")
        .unwrap()
        .utf8_text(writer.source)?;
    let prev_var_type = prev_node
        .child_by_field_name("type")
        .unwrap()
        .utf8_text(writer.source)?;

    if var_type != prev_var_type {
        writer.breakl();
        return Ok(());
    }

    Ok(())
}

pub fn write_type(node: Node, writer: &mut Writer, insert_space: bool) -> Result<(), Utf8Error> {
    let next_kind = next_sibling_kind(&node);

    write_node(node, writer)?;
    if insert_space && next_kind != "dimension" && next_kind != "fixed_dimension" {
        writer.output.push(' ')
    };

    Ok(())
}

pub fn write_variable_declaration_statement(
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
            "type" => write_type(child, writer, true)?,
            "dimension" => write_dimension(child, writer)?,
            "variable_declaration" => write_variable_declaration(child, writer)?,
            "comment" => write_comment(child, writer)?,
            ";" => writer.output.push(';'),
            "," => writer.output.push_str(", "),
            _ => write_node(child, writer)?,
        }
    }

    if do_indent && !writer.output.ends_with(';') {
        writer.output.push(';');
    }

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
                write_node(child, writer)?;
                writer.output.push(' ');
            }
            "old_variable_declaration" => write_old_variable_declaration(child, writer)?,
            "comment" => write_comment(child, writer)?,
            ";" => writer.output.push(';'),
            "," => writer.output.push_str(", "),
            _ => write_node(child, writer)?,
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
    // Write the dimensions of a declaration, if they exist.
    for child in node.named_children(&mut cursor) {
        match child.kind().borrow() {
            "old_type" => write_old_type(child, writer)?,
            "dimension" => write_dimension(child, writer)?,
            "fixed_dimension" => write_fixed_dimension(child, writer)?,
            "symbol" => write_node(child, writer)?,
            _ => continue,
        }
    }

    // Write the default value of a declaration, if it exists.
    for child in node.children_by_field_name("initialValue", &mut cursor) {
        if child.kind().to_string() == "=" {
            writer.output.push_str(" = ");
            continue;
        }
        write_expression(child, writer)?;
        break;
    }

    Ok(())
}

fn write_variable_storage_class(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "const" | "static" => {
                write_node(sub_node, writer)?;
                writer.output.push(' ');
            }
            _ => write_node(sub_node, writer)?,
        }
    }

    Ok(())
}

fn write_variable_declaration(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let var_name = node
        .child_by_field_name("name")
        .unwrap()
        .utf8_text(writer.source)?;
    writer.output.push_str(var_name.borrow());

    let mut cursor = node.walk();
    // Write the dimensions of a declaration, if they exist.
    for sub_child in node.named_children(&mut cursor) {
        match sub_child.kind().borrow() {
            "fixed_dimension" => write_fixed_dimension(sub_child, writer)?,
            "dimension" => write_dimension(sub_child, writer)?,
            _ => continue,
        }
    }
    // Write the default value of a declaration, if it exists.
    for sub_child in node.children_by_field_name("initialValue", &mut cursor) {
        if sub_child.kind().to_string() == "=" {
            writer.output.push_str(" = ");
            continue;
        } else if sub_child.kind().to_string() == "dynamic_array" {
            write_dynamic_array(sub_child, writer)?;
            continue;
        }
        write_expression(sub_child, writer)?;
        break;
    }

    Ok(())
}
