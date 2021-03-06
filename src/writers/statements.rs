use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    expressions::write_expression,
    next_sibling_kind,
    variables::{write_old_variable_declaration_statement, write_variable_declaration_statement},
    write_comment, write_node, Writer,
};

pub fn write_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
    do_break: bool,
) -> Result<(), Utf8Error> {
    if do_indent {
        writer.indent += 1;
    }
    match node.kind().borrow() {
        "block" => write_block(node, writer, do_indent)?,
        "variable_declaration_statement" => {
            write_variable_declaration_statement(node, writer, false)?
        }
        "old_variable_declaration_statement" => {
            write_old_variable_declaration_statement(node, writer, false)?
        }
        "for_loop" => write_for_loop(node, writer)?,
        "while_loop" => write_while_loop(node, writer)?,
        "do_while_loop" => write_do_while_loop(node, writer)?,
        "break_statement" => {
            writer.write_indent();
            writer.output.push_str("break");
            writer.output.push(';');
        }
        "continue_statement" => {
            writer.write_indent();
            writer.output.push_str("continue");
            writer.output.push(';');
        }
        "condition_statement" => write_condition_statement(node, writer)?,
        "switch_statement" => write_switch_statement(node, writer)?,
        "return_statement" => write_return_statement(node, writer)?,
        "delete_statement" => write_delete_statement(node, writer)?,
        "expression_statement" => write_expression_statement(node, writer)?,
        _ => write_node(node, writer)?,
    }
    if do_indent {
        writer.indent -= 1;
    }
    if do_break {
        writer.breakl();
    }

    Ok(())
}

fn write_for_loop(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut end_condition_reached = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "for" => {
                writer.write_indent();
                write_node(child, writer)?;
            }
            "(" => write_node(child, writer)?,
            ")" => {
                end_condition_reached = true;
                write_node(child, writer)?;
            }
            "assignment_expression" => write_expression(child, writer)?,
            ";" => writer.output.push(';'),
            "," => writer.output.push_str(", "),
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                        write_statement(child, writer, false, false)?;
                        continue;
                    }

                    if end_condition_reached {
                        if kind == "block" {
                            if writer.settings.brace_wrapping_before_loop {
                                writer.breakl();
                                write_block(child, writer, true)?;
                            } else {
                                writer.output.push(' ');
                                write_block(child, writer, false)?;
                            }
                        } else {
                            writer.breakl();
                            write_statement(child, writer, true, false)?;
                        }
                    } else {
                        match kind.borrow() {
                            "variable_declaration_statement" => {
                                write_variable_declaration_statement(child, writer, true)?
                            }
                            "old_variable_declaration_statement" => {
                                write_old_variable_declaration_statement(child, writer, true)?
                            }
                            _ => write_node(child, writer)?,
                        }
                    }
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_while_loop(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "while" => {
                writer.write_indent();
                write_node(child, writer)?;
            }
            "(" => write_node(child, writer)?,
            ")" => {
                write_node(child, writer)?;
                writer.breakl()
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_statement(child, writer, false, false)?;
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_do_while_loop(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "do" => {
                writer.write_indent();
                write_node(child, writer)?;
                writer.breakl();
            }
            "while" => {
                writer.write_indent();
                write_node(child, writer)?;
            }
            "(" => write_node(child, writer)?,
            ")" => {
                write_node(child, writer)?;
                writer.breakl()
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_statement(child, writer, false, false)?;
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_switch_statement(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "switch" => {
                writer.write_indent();
                write_node(child, writer)?;
                writer.output.push(' ');
            }
            "(" => write_node(child, writer)?,
            ")" => {
                write_node(child, writer)?;
                writer.breakl()
            }
            "{" => {
                writer.write_indent();
                writer.output.push_str("{\n");
                writer.indent += 1;
            }
            "}" => {
                writer.indent -= 1;
                writer.write_indent();
                writer.output.push_str("}\n");
            }
            "switch_case" => write_switch_case(child, writer)?,
            "switch_default_case" => write_switch_default_case(child, writer)?,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_switch_case(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "case" => {
                writer.write_indent();
                write_node(child, writer)?;
                writer.output.push(' ');
            }
            ":" => {
                writer.output.push_str(":\n");
            }
            "switch_case_values" => write_switch_case_values(child, writer)?,
            "comment" => write_comment(child, writer)?,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else if writer.is_statement(kind.to_string()) {
                    let is_block = kind == "block";
                    write_statement(child, writer, !is_block, true)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_switch_default_case(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "default" => {
                writer.write_indent();
                write_node(child, writer)?;
            }
            ":" => {
                writer.output.push_str(":\n");
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    write_statement(child, writer, false, true)?;
                } else {
                    write_node(child, writer)?
                }
            }
        }
    }

    Ok(())
}

fn write_switch_case_values(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "comment" => write_comment(child, writer)?,
            "symbol" => write_node(child, writer)?,
            "," => writer.output.push_str(", "),
            _ => {
                if writer.is_literal(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_return_statement(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "comment" => write_comment(child, writer)?,
            "return" => {
                writer.write_indent();
                writer.output.push_str("return ");
            }
            ";" => writer.output.push(';'),
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?
                }
            }
        }
    }

    Ok(())
}

fn write_delete_statement(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "comment" => write_comment(child, writer)?,
            "delete" => {
                writer.write_indent();
                writer.output.push_str("delete ");
            }
            ";" => writer.output.push(';'),
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?
                }
            }
        }
    }

    Ok(())
}

fn write_expression_statement(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "comment" => write_comment(child, writer)?,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?
                }
            }
        }
    }

    Ok(())
}

fn write_condition_statement(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "if" => {
                if writer.output.ends_with("else") {
                    writer.output.push(' ');
                } else {
                    writer.write_indent();
                }
                write_node(child, writer)?;
            }
            "else" => {
                let next_sibling_kind = next_sibling_kind(&child);
                writer.write_indent();
                write_node(child, writer)?;
                if next_sibling_kind != "condition_statement" {
                    writer.breakl();
                }
            }
            "(" => write_node(child, writer)?,
            ")" => {
                write_node(child, writer)?;
                writer.breakl()
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_statement(child, writer, false, true)?;
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(child, writer)?;
                }
            }
        }
    }

    Ok(())
}

pub fn write_block(node: Node, writer: &mut Writer, do_indent: bool) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "{" => {
                if do_indent {
                    writer.write_indent();
                }
                write_node(child, writer)?;
                writer.breakl();
                writer.indent += 1;
            }
            "}" => {
                writer.indent -= 1;
                writer.write_indent();
                write_node(child, writer)?;
            }
            "comment" => write_comment(child, writer)?,
            _ => {
                if writer.is_statement(kind.to_string()) {
                    write_statement(child, writer, false, true)?
                } else {
                    write_node(child, writer)?
                }
            }
        }
    }

    Ok(())
}
