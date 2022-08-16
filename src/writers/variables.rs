use super::{
    expressions::{write_expression, write_old_type},
    next_sibling_kind, node_len,
    preproc::insert_break,
    write_comment, write_dimension, write_dynamic_array, write_fixed_dimension, write_node, Writer,
};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

/// Write a global variable declaration.
///
/// # Arguments
///
/// * `node`   - The global variable declaration node to write.
/// * `writer` - The writer object.
pub fn write_global_variable_declaration(
    node: &Node,
    writer: &mut Writer,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    let max_name_length = get_max_name_length(&node)?;

    // Keep track of the type's length (as well as the storage class and visibility)
    // to properly indent line break variables.
    // Start at `1` to account for the ` ` between the type and the declaration.
    let mut type_length = 1;
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "type" => {
                write_type(&child, writer)?;
                type_length += node_len(&child);
            }
            "variable_storage_class" | "variable_visibility" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
                type_length += node_len(&child) + 1;
            }
            "comment" => {
                write_comment(&child, writer)?;
                if max_name_length > 0 {
                    writer.output.push_str(" ".repeat(type_length).as_str());
                }
            }
            "variable_declaration" => write_variable_declaration(&child, writer, max_name_length)?,
            "," => {
                if max_name_length > 0 {
                    let next_kind = next_sibling_kind(&child);
                    if next_kind == "comment" {
                        writer.output.push_str(",");
                    } else {
                        writer.output.push_str(",\n");
                        writer.output.push_str(" ".repeat(type_length).as_str());
                    }
                } else {
                    writer.output.push_str(", ")
                }
            }
            ";" => continue,
            _ => println!("Unexpected kind {} in write_global_variable.", kind),
        }
    }
    writer.output.push(';');

    insert_break(&node, writer);

    Ok(())
}

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

pub fn write_type(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let next_kind = next_sibling_kind(&node);

    write_node(&node, writer)?;
    if next_kind != "dimension" && next_kind != "fixed_dimension" {
        writer.output.push(' ')
    };

    Ok(())
}

/// Get the max name length from variable declarations in the same node.
/// Returns 0 if the line should not be broken after the `,`.
///
/// # Arguments
///
/// * `node`   - The node which has the variable declarations.
fn get_max_name_length(node: &Node) -> Result<usize, Utf8Error> {
    let mut cursor = node.walk();

    // Compute an estimated length of the declarations.
    // If it's longer than an threshold, we break the declarations.
    let mut length = 0;
    let mut max_name_length = 0;
    let mut nb_declarations = 0;
    let mut nested_comment = false;
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "variable_declaration" => {
                // FIXME: This includes whitespaces, and might yield incorrect results.
                // Sum the length of each node once formatted instead.
                length += node_len(&child);
                if nb_declarations > 0 {
                    // Take the `, ` into account.
                    length += 2
                }
                nb_declarations += 1;
                let mut name_length = 0;
                let mut sub_cursor = child.walk();
                for sub_child in child.children(&mut sub_cursor) {
                    let sub_kind = sub_child.kind();
                    match sub_kind.borrow() {
                        "symbol" => name_length += node_len(&sub_child),
                        "dimension" => name_length += 2,
                        "fixed_dimension" => name_length += node_len(&sub_child),
                        _ => continue,
                    }
                }
                if name_length > max_name_length {
                    max_name_length = name_length;
                }
            }
            // If a nested comment is present, break, even if the line
            // is too long.
            "comment" => nested_comment = true,
            _ => continue,
        }
    }

    if length <= 80 && !nested_comment {
        max_name_length = 0;
    }

    Ok(max_name_length)
}

/// Write a variable declaration statement.
///
/// # Arguments
///
/// * `node`   - The variable declaration statement node to write.
/// * `writer` - The writer object.
pub fn write_variable_declaration_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    let max_name_length = get_max_name_length(&node)?;

    // Keep track of the type's length (as well as the storage class and visibility)
    // to properly indent line break variables.
    // Start at `1` to account for the ` ` between the type and the declaration.
    let mut type_length = 1;
    if do_indent {
        writer.write_indent();
    }

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "type" => {
                write_type(&child, writer)?;
                type_length += node_len(&child);
            }
            "variable_storage_class" | "variable_visibility" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
                type_length += node_len(&child) + 1;
            }
            "comment" => {
                write_comment(&child, writer)?;
                if max_name_length > 0 {
                    if do_indent {
                        writer.write_indent();
                    }
                    writer.output.push_str(" ".repeat(type_length).as_str());
                }
            }
            "dimension" => write_dimension(child, writer, true)?,
            "variable_declaration" => write_variable_declaration(&child, writer, max_name_length)?,
            "," => {
                if max_name_length > 0 {
                    let next_kind = next_sibling_kind(&child);
                    if next_kind == "comment" {
                        writer.output.push_str(",");
                    } else {
                        writer.output.push_str(",\n");
                        if do_indent {
                            writer.write_indent();
                        }
                        writer.output.push_str(" ".repeat(type_length).as_str());
                    }
                } else {
                    writer.output.push_str(", ")
                }
            }
            ";" => continue,
            _ => println!("Unexpected kind {} in write_global_variable.", kind),
        }
    }

    if do_indent {
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

/// Write a variable storage class.
///
/// # Arguments
///
/// * `node`   - The variable storage class node to write.
/// * `writer` - The writer object.
fn write_variable_storage_class(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = node.kind();
        match kind.borrow() {
            "const" | "static" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            _ => println!("Unexpected kind {} in write_variable_storage_class.", kind),
        }
    }

    Ok(())
}

/// Write a variable declaration.
///
/// # Arguments
///
/// * `node`   - The variable declaration node to write.
/// * `writer` - The writer object.
/// * `writer` - The writer object.
fn write_variable_declaration(
    node: &Node,
    writer: &mut Writer,
    max_name_length: usize,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    // Keep track of the name length (including dimensions) in order
    // to properly align the `=` sign.
    let mut name_length = 0;

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "symbol" => {
                write_node(&child, writer)?;
                name_length += node_len(&child);
            }
            "fixed_dimension" => {
                name_length += node_len(&child);
                write_fixed_dimension(child, writer, false)?;
            }
            "dimension" => {
                write_dimension(child, writer, false)?;
                name_length += 2;
            }
            "=" => {
                if max_name_length > 0 {
                    writer
                        .output
                        .push_str(" ".repeat(max_name_length - name_length).as_str());
                }
                writer.output.push_str(" = ");
            }
            "dynamic_array" => write_dynamic_array(child, writer)?,
            _ => {
                if writer.is_expression(&kind) {
                    write_expression(child, writer)?
                } else {
                    println!("Unexpected kind {} in write_global_variable.", kind);
                }
            }
        }
    }

    Ok(())
}
