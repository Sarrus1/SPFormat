use super::expressions::write_expression;
use super::{write_comment, write_dimension, write_fixed_dimension, write_node, Writer};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

pub fn write_struct_declaration(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "public" | "symbol" => {
                write_node(sub_node, writer)?;
                writer.output.push(' ');
            }
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "=" => {
                writer.output.push_str("=\n");
            }
            "struct_constructor" => write_struct_constructor(sub_node, writer)?,
            "\n" | _ => {}
        }
    }

    Ok(())
}

fn write_struct_constructor(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "struct_field_value" => write_struct_field_value(sub_node, writer)?,
            "{" => {
                writer.indent += 1;
                writer.output.push_str("{\n");
            }
            "}" => {
                writer.indent -= 1;
                writer.output.push('}');
            }
            ";" => writer.output.push(';'),
            _ => println!("{}", sub_node.kind()),
        }
    }
    if !writer.output.ends_with(';') {
        writer.output.push_str(";\n");
    }

    Ok(())
}

fn write_struct_field_value(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    let mut key = true;
    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "symbol" => {
                if key {
                    key = false;
                    writer
                        .output
                        .push_str(writer.indent_string.repeat(writer.indent).as_str());
                    write_node(sub_node, writer)?;
                } else {
                    key = true;
                    write_node(sub_node, writer)?;
                    writer.output.push_str(",\n");
                }
            }
            "=" => writer.output.push_str(" = "),
            _ => {
                write_expression(sub_node, writer)?;
                writer.output.push_str(",\n")
            }
        }
    }

    Ok(())
}

pub fn write_struct(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "struct" => writer.output.push_str("struct "),
            "symbol" => write_node(sub_node, writer)?,
            "{" => {
                writer.indent += 1;
                writer.output.push_str("\n{\n");
            }
            "}" => {
                writer.indent -= 1;
                writer.output.push('}');
            }
            "struct_field" => write_struct_field(sub_node, writer)?,
            _ => writer.output.push_str(";\n"),
        }
    }

    Ok(())
}

fn write_struct_field(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer
        .output
        .push_str(writer.indent_string.repeat(writer.indent).as_str());

    let mut cursor = node.walk();
    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "public" => writer.output.push_str("public "),
            "const" => writer.output.push_str("const "),
            "type" => write_node(sub_node, writer)?,
            "symbol" => {
                writer.output.push(' ');
                write_node(sub_node, writer)?;
            }
            "fixed_dimension" => write_fixed_dimension(sub_node, writer)?,
            "dimension" => write_dimension(sub_node, writer)?,
            ";" => writer.output.push(';'),
            _ => {
                println!("{}", sub_node.kind())
            }
        }
    }

    Ok(())
}
