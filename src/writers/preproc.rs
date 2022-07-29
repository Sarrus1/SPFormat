use super::{write_comment, write_node, Writer};
use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

pub fn write_preproc_include(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "string_literal" | "system_lib_string" => {
                writer
                    .output
                    .push_str(sub_node.utf8_text(writer.source)?.borrow());
            }
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "#include" => writer.output.push_str("#include "),
            "#tryinclude" => writer.output.push_str("#tryinclude "),
            "\n" | _ => {}
        }
    }
    if !writer.output.ends_with('\n') {
        writer.output.push('\n');
    }

    Ok(())
}

pub fn write_preproc_define(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "symbol" => write_node(sub_node, writer)?,
            "preproc_arg" => {
                writer.output.push(' ');
                write_preproc_arg(sub_node, writer)?;
            }
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "#define" => writer.output.push_str("#define "),
            "\n" | _ => {}
        }
    }
    if !writer.output.ends_with('\n') {
        writer.output.push('\n');
    }

    Ok(())
}

pub fn write_preproc_undefine(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "symbol" => write_node(sub_node, writer)?,
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "#undef" => writer.output.push_str("#undef "),
            "\n" | _ => {}
        }
    }
    if !writer.output.ends_with('\n') {
        writer.output.push('\n');
    }

    Ok(())
}

pub fn write_preproc_generic(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "symbol" => write_node(sub_node, writer)?,
            "comment" => {
                writer.output.push('\t');
                write_comment(sub_node, writer)?;
            }
            "preproc_defined_condition" => write_node(sub_node, writer)?,
            "#if" => writer.output.push_str("#if "),
            "#endif" => writer.output.push_str("#endif"),
            "#else" => writer.output.push_str("#else"),
            "#endinput" => writer.output.push_str("#else"),
            "#pragma" => writer.output.push_str("#pragma "),
            "\n" | _ => {}
        }
    }
    if !writer.output.ends_with('\n') {
        writer.output.push('\n');
    }

    Ok(())
}

fn write_preproc_arg(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let args = node.utf8_text(writer.source)?;
    writer.output.push_str(args.trim());

    Ok(())
}
