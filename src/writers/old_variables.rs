use super::{
    expressions::{write_expression, write_old_type},
    next_sibling_kind, node_len,
    preproc::insert_break,
    write_comment, write_dimension, write_fixed_dimension, write_node, Writer,
};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

/// Write an old global variable declaration.
///
/// # Arguments
///
/// * `node`   - The old global variable declaration node to write.
/// * `writer` - The writer object.
pub fn write_old_global_variable_declaration(
    node: &Node,
    writer: &mut Writer,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    let should_break = should_break_declaration(&node)?;

    let mut declarator_length = 0;
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "variable_storage_class" | "variable_visibility" | "new" | "decl" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
                declarator_length += node_len(&child) + 1;
            }
            "comment" => {
                write_comment(&child, writer)?;
                if should_break {
                    writer
                        .output
                        .push_str(" ".repeat(declarator_length).as_str());
                }
            }
            "old_variable_declaration" => write_old_variable_declaration(child, writer)?,
            "," => {
                if should_break {
                    let next_kind = next_sibling_kind(&child);
                    if next_kind == "comment" {
                        writer.output.push_str(",");
                    } else {
                        writer.output.push_str(",\n");
                        writer
                            .output
                            .push_str(" ".repeat(declarator_length).as_str());
                    }
                } else {
                    writer.output.push_str(", ")
                }
            }
            ";" => continue,
            _ => println!(
                "Unexpected kind {} in write_old_global_variable_declaration.",
                kind
            ),
        }
    }
    writer.output.push(';');

    insert_break(&node, writer);

    Ok(())
}

/// Computes if we should break after each `,` in an old variable declaration.
///
/// # Arguments
///
/// * `node`   - The node which has the variable declarations.
fn should_break_declaration(node: &Node) -> Result<bool, Utf8Error> {
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
            "old_variable_declaration" => {
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
        return Ok(false);
    }

    Ok(true)
}

/// Write an old variable declaration statement.
///
/// # Arguments
///
/// * `node`   - The old variable declaration statement node to write.
/// * `writer` - The writer object.
pub fn write_old_variable_declaration_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    let should_break = should_break_declaration(&node)?;

    if do_indent {
        writer.write_indent();
    }

    let mut declarator_length = 0;

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "variable_storage_class" | "new" | "decl" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
                declarator_length += node_len(&child) + 1;
            }
            "old_variable_declaration" => write_old_variable_declaration(child, writer)?,
            "comment" => {
                write_comment(&child, writer)?;
                if should_break {
                    if do_indent {
                        writer.write_indent();
                    }
                    writer
                        .output
                        .push_str(" ".repeat(declarator_length).as_str());
                }
            }
            "," => {
                if should_break {
                    let next_kind = next_sibling_kind(&child);
                    if next_kind == "comment" {
                        writer.output.push_str(",");
                    } else {
                        writer.output.push_str(",\n");
                        if do_indent {
                            writer.write_indent();
                        }
                        writer
                            .output
                            .push_str(" ".repeat(declarator_length).as_str());
                    }
                } else {
                    writer.output.push_str(", ")
                }
            }
            ";" => continue,
            _ => write_node(&child, writer)?,
        }
    }

    if do_indent {
        writer.output.push(';');
        insert_break(&node, writer);
    }

    Ok(())
}

/// Write an old variable declaration.
///
/// # Arguments
///
/// * `node`   - The old variable declaration node to write.
/// * `writer` - The writer object.
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
