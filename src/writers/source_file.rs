use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    expressions::write_function_call_arguments,
    functions::{write_function_declaration, write_function_definition},
    preproc::{
        write_preproc_define, write_preproc_generic, write_preproc_include, write_preproc_undefine,
    },
    structs::{write_struct, write_struct_declaration},
    variables::write_global_variable,
    write_comment, write_node, Writer,
};

pub fn write_source_file(root_node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = root_node.walk();

    for node in root_node.children(&mut cursor) {
        if writer.skip > 0 {
            writer.skip -= 1;
            continue;
        }
        match node.kind().borrow() {
            "assertion" => write_assertion(node, writer)?,
            "global_variable_declaration" => write_global_variable(node, writer)?,
            "preproc_include" | "preproc_tryinclude" => write_preproc_include(node, writer)?,
            "preproc_macro" | "preproc_define" => write_preproc_define(node, writer)?,
            "preproc_undefine" => write_preproc_undefine(node, writer)?,
            "preproc_if" | "preproc_endif" | "preproc_else" | "preproc_endinput"
            | "preproc_pragma" => write_preproc_generic(node, writer)?,
            "struct_declaration" => write_struct_declaration(node, writer)?,
            "struct" => write_struct(node, writer)?,
            "comment" => write_comment(node, writer)?,
            "function_declaration" => write_function_declaration(node, writer)?,
            "function_definition" => write_function_definition(node, writer)?,
            _ => writer
                .output
                .push_str(node.utf8_text(writer.source)?.borrow()),
        };
    }

    Ok(())
}

pub fn write_assertion(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "assert" | "static_assert" => write_node(child, writer)?,
            "function_call_arguments" => write_function_call_arguments(child, writer)?,
            ";" => write_node(child, writer)?,
            "comment" => write_comment(child, writer)?,
            _ => println!("Unexpected kind {} in write_assertion.", kind),
        }
    }
    writer.breakl();

    Ok(())
}
