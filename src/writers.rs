use crate::settings::Settings;

use self::{expressions::write_expression, preproc::insert_break};

use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
    str::Utf8Error,
};
use tree_sitter::{Language, Node, Point};

pub mod alias;
pub mod assertions;
pub mod enum_structs;
pub mod enums;
pub mod expressions;
pub mod functags;
pub mod functions;
pub mod hardcoded_symbols;
pub mod methodmaps;
pub mod old_variables;
pub mod preproc;
pub mod source_file;
pub mod statements;
pub mod structs;
pub mod typedefs;
pub mod variables;

pub struct Writer<'a> {
    pub output: String,
    pub source: &'a [u8],
    pub language: &'a Language,
    pub indent: usize,
    pub indent_string: String,
    pub skip: u8,
    pub settings: &'a Settings,
    pub _statement_kinds: HashSet<String>,
    pub _expression_kinds: HashSet<String>,
    pub _literal_kinds: HashSet<String>,
}

impl Writer<'_> {
    fn write_indent(&mut self) {
        self.output
            .push_str(self.indent_string.repeat(self.indent).as_str());
    }

    fn breakl(&mut self) {
        self.output.push('\n');
    }

    fn is_statement(&mut self, kind: &Cow<str>) -> bool {
        return self._statement_kinds.contains(&kind.to_string());
    }

    fn is_expression(&mut self, kind: &Cow<str>) -> bool {
        return self._expression_kinds.contains(&kind.to_string()) || self.is_literal(kind);
    }

    fn is_literal(&mut self, kind: &Cow<str>) -> bool {
        return self._literal_kinds.contains(&kind.to_string());
    }
}

pub fn write_comment(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let prev_node = node.prev_named_sibling();
    if !prev_node.is_none() {
        let prev_node = prev_node.unwrap();
        if node.start_position().row() == prev_node.end_position().row() {
            // Previous node is on the same line, simply add a tab.
            writer.output.push_str(writer.indent_string.as_str());
        } else {
            // Previous node is on a different line, indent the comment.
            writer.write_indent();
        }
    }

    let text = node.utf8_text(writer.source)?;
    writer.output.push_str(&text.trim());

    insert_break(&node, writer);

    Ok(())
}

fn write_dynamic_array(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer.output.push_str("new ");
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "type" => write_node(&child, writer)?,
            // TODO: Handle different cases here.
            _ => write_node(&child, writer)?,
        }
    }

    Ok(())
}

fn write_dimension(node: Node, writer: &mut Writer, insert_space: bool) -> Result<(), Utf8Error> {
    let next_kind = next_sibling_kind(&node);
    writer.output.push_str("[]");

    if insert_space && next_kind != "dimension" && next_kind != "fixed_dimension" {
        writer.output.push(' ')
    };

    Ok(())
}

fn write_fixed_dimension(
    node: Node,
    writer: &mut Writer,
    insert_space: bool,
) -> Result<(), Utf8Error> {
    let next_kind = next_sibling_kind(&node);

    let mut cursor = node.walk();

    writer.output.push('[');

    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "[" | "]" => continue,
            _ => write_expression(child, writer)?,
        }
    }
    writer.output.push(']');

    if insert_space && next_kind != "dimension" && next_kind != "fixed_dimension" {
        writer.output.push(' ')
    };

    Ok(())
}

fn write_node(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
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

fn prev_sibling_kind(node: &Node) -> String {
    let prev_node = node.prev_sibling();
    if prev_node.is_none() {
        return String::from("");
    }
    return String::from(prev_node.unwrap().kind());
}

fn next_sibling_start(node: &Node) -> Option<Point> {
    let next_node = node.next_sibling();
    if next_node.is_none() {
        return None;
    }
    return Some(next_node.unwrap().start_position());
}

#[allow(dead_code)]
fn prev_sibling_end(node: &Node) -> Option<Point> {
    let prev_node = node.prev_sibling();
    if prev_node.is_none() {
        return None;
    }
    return Some(prev_node.unwrap().end_position());
}

/// Returns the length of a node.
///
/// # Arguments
///
/// * `node`   - The node to compute the length of.
pub fn node_len(node: &Node) -> usize {
    usize::try_from(node.end_byte() - node.start_byte()).unwrap()
}
