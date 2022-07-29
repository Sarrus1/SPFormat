use self::expressions::write_expression;
use self::functions::{write_function_declaration, write_function_definition};
use self::preproc::{
    write_preproc_define, write_preproc_generic, write_preproc_include, write_preproc_undefine,
};
use self::structs::{write_struct, write_struct_declaration};

use std::{borrow::Borrow, collections::HashSet, str::Utf8Error};
use tree_sitter::{Language, Node};

pub mod expressions;
pub mod functions;
pub mod preproc;
pub mod statements;
pub mod structs;

pub struct Writer<'a> {
    pub output: String,
    pub source: &'a [u8],
    pub language: &'a Language,
    pub indent: usize,
    pub indent_string: String,
    pub skip: u8,
    pub _statement_kinds: HashSet<String>,
    pub _expression_kinds: HashSet<String>,
    pub _literal_kinds: HashSet<String>,
}

impl Writer<'_> {
    fn write_indent(&mut self) {
        self.output
            .push_str(self.indent_string.repeat(self.indent).as_str());
    }

    fn is_statement(&mut self, kind: String) -> bool {
        return self._statement_kinds.contains(&kind);
    }

    fn is_expression(&mut self, kind: String) -> bool {
        return self._expression_kinds.contains(&kind) || self.is_literal(kind);
    }

    fn is_literal(&mut self, kind: String) -> bool {
        return self._literal_kinds.contains(&kind);
    }
}

pub fn write_source_file(root_node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = root_node.walk();

    for node in root_node.children(&mut cursor) {
        if writer.skip > 0 {
            writer.skip -= 1;
            continue;
        }
        match node.kind().borrow() {
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

pub fn write_global_variable(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    global_variable_declaration_break(&node, writer)?;

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
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
            _ => println!("{}", sub_node.kind()),
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
        writer.output.push('\n');
        return Ok(());
    }
    let prev_node = prev_node.unwrap();
    if prev_node.kind() == "comment"
        && prev_node.end_position().row() == node.start_position().row() - 1
    {
        return Ok(());
    }
    if prev_node.kind() != "global_variable_declaration" {
        writer.output.push('\n');
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
        writer.output.push('\n');
        return Ok(());
    }

    Ok(())
}

fn write_type(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let next_kind = next_sibling_kind(&node);

    write_node(node, writer)?;
    if next_kind != "dimension" {
        writer.output.push(' ')
    };

    Ok(())
}

fn write_variable_declaration_statement(
    node: Node,
    writer: &mut Writer,
    is_for_loop: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    if !is_for_loop {
        writer.write_indent();
    }

    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "variable_storage_class" => write_variable_storage_class(child, writer)?,
            "type" => write_type(child, writer)?,
            "dimension" => write_dimension(child, writer)?,
            "variable_declaration" => write_variable_declaration(child, writer)?,
            "comment" => write_comment(child, writer)?,
            ";" => writer.output.push(';'),
            "," => writer.output.push_str(", "),
            _ => write_node(child, writer)?,
        }
    }

    if !is_for_loop {
        if !writer.output.ends_with(';') {
            writer.output.push(';');
        }
        writer.output.push('\n');
    }

    Ok(())
}

fn write_old_variable_declaration_statement(
    node: Node,
    writer: &mut Writer,
    is_for_loop: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    if !is_for_loop {
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

    if !is_for_loop {
        if !writer.output.ends_with(';') {
            writer.output.push(';');
        }
        writer.output.push('\n');
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

pub fn write_comment(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let prev_node = node.prev_named_sibling();
    if !prev_node.is_none() {
        let prev_node = prev_node.unwrap();
        match prev_node.kind().borrow() {
            "comment" => {
                if node.start_position().row() - 1 > prev_node.end_position().row() {
                    // Add a single break
                    writer.output.push('\n');
                }
            }
            _ => {}
        }
    }
    write_node(node, writer)?;
    writer.output.push('\n');

    Ok(())
}

fn write_dynamic_array(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer.output.push_str("new ");
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "type" => write_node(child, writer)?,
            // TODO: Handle different cases here.
            _ => write_node(child, writer)?,
        }
    }

    Ok(())
}

fn write_function_call_arguments(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    writer.output.push('(');
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "(" | ")" => continue,
            "symbol" | "ignore_argument" => write_node(child, writer)?,
            "named_arg" => write_named_arg(child, writer)?,
            _ => write_expression(child, writer)?,
        }
    }
    // Remove the last ", ".
    writer.output.pop();
    writer.output.pop();
    writer.output.push(')');

    Ok(())
}

fn write_named_arg(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer.output.push('.');
    write_node(node.child_by_field_name("name").unwrap(), writer)?;
    writer.output.push_str(" = ");
    // FIXME: Always write_node.
    write_node(node.child_by_field_name("value").unwrap(), writer)?;

    Ok(())
}

fn write_dimension(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let next_kind = next_sibling_kind(&node);
    writer.output.push_str("[]");
    if next_kind != "dimension" || next_kind != "fixed_dimension" {
        writer.output.push(' ')
    };

    Ok(())
}

fn write_fixed_dimension(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    writer.output.push('[');
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "[" | "]" => continue,
            _ => write_expression(child, writer)?,
        }
    }
    writer.output.push(']');

    Ok(())
}

fn write_old_type_cast(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_old_type(node.child_by_field_name("type").unwrap(), writer)?;
    write_expression(node.child_by_field_name("value").unwrap(), writer)?;

    Ok(())
}

fn write_old_type(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer
        .output
        .push_str(node.utf8_text(writer.source)?.borrow());
    writer.output.push(' ');

    Ok(())
}

fn write_node(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer
        .output
        .push_str(node.utf8_text(writer.source)?.borrow());

    Ok(())
}

fn next_sibling_kind(node: &Node) -> String {
    let next_node = node.next_sibling();
    if next_node.is_none() {
        return String::from("");
    }
    return String::from(next_node.unwrap().kind());
}
