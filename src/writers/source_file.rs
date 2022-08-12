use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use crate::writers::preproc::write_preproc_symbol;

use super::{
    alias::{write_alias_assignment, write_alias_declaration},
    assertions::write_assertion,
    enum_structs::write_enum_struct,
    enums::write_enum,
    functags::{write_funcenum, write_functag},
    functions::{write_function_declaration, write_function_definition},
    hardcoded_symbols::write_hardcoded_symbol,
    methodmaps::write_methodmap,
    preproc::{
        write_preproc_define, write_preproc_generic, write_preproc_include, write_preproc_undefine,
    },
    structs::{write_struct, write_struct_declaration},
    typedefs::{write_typedef, write_typeset},
    variables::{write_global_variable_declaration, write_old_global_variable_declaration},
    write_comment, Writer,
};

pub fn write_source_file(root_node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = root_node.walk();

    for node in root_node.children(&mut cursor) {
        if writer.skip > 0 {
            writer.skip -= 1;
            continue;
        }
        let kind = node.kind();
        match kind.borrow() {
            "assertion" => write_assertion(&node, writer)?,
            "function_declaration" => write_function_declaration(node, writer)?,
            "function_definition" => write_function_definition(node, writer)?,
            "enum" => write_enum(node, writer)?,
            "enum_struct" => write_enum_struct(node, writer)?,
            "typedef" => write_typedef(node, writer)?,
            "typeset" => write_typeset(node, writer)?,
            "functag" => write_functag(node, writer)?,
            "funcenum" => write_funcenum(node, writer)?,
            "methodmap" => write_methodmap(node, writer)?,
            "struct" => write_struct(node, writer)?,
            "struct_declaration" => write_struct_declaration(node, writer)?,
            "global_variable_declaration" => write_global_variable_declaration(&node, writer)?,
            "old_global_variable_declaration" => {
                write_old_global_variable_declaration(&node, writer)?
            }
            "preproc_include" | "preproc_tryinclude" => write_preproc_include(&node, writer)?,
            "preproc_macro" | "preproc_define" => write_preproc_define(&node, writer)?,
            "preproc_undefine" => write_preproc_undefine(&node, writer)?,
            "hardcoded_symbol" => write_hardcoded_symbol(&node, writer)?,
            "alias_declaration" => write_alias_declaration(node, writer)?,
            "alias_assignment" => write_alias_assignment(node, writer)?,
            "comment" => write_comment(&node, writer)?,
            "preproc_endif" | "preproc_else" | "preproc_endinput" => {
                write_preproc_symbol(&node, writer)?
            }
            "preproc_if" | "preproc_elseif" | "preproc_pragma" | "preproc_error"
            | "preproc_warning" | "preproc_assert" => write_preproc_generic(&node, writer)?,
            _ => {
                println!("Unexpected kind {} in write_source_file.", kind);
                writer
                    .output
                    .push_str(node.utf8_text(writer.source)?.borrow());
            }
        };
    }

    Ok(())
}
