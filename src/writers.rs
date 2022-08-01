use crate::settings::Settings;

use self::expressions::write_expression;

use std::{borrow::Borrow, collections::HashSet, str::Utf8Error};
use tree_sitter::{Language, Node};

pub mod expressions;
pub mod functions;
pub mod preproc;
pub mod source_file;
pub mod statements;
pub mod structs;
pub mod variables;

pub struct Writer<'a> {
    pub output: String,
    pub source: &'a [u8],
    pub language: &'a Language,
    pub indent: usize,
    pub indent_string: String,
    pub skip: u8,
    pub settings: Settings,
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
