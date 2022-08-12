use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    expressions::write_expression,
    variables::{write_old_variable_declaration_statement, write_variable_declaration_statement},
    write_comment, write_node, Writer,
};

pub fn write_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
    do_break: bool,
) -> Result<(), Utf8Error> {
    let sp = node.end_position().row();
    let next_sibling = node.next_sibling();

    match node.kind().borrow() {
        "block" => write_block(node, writer, do_indent)?,
        "variable_declaration_statement" => {
            write_variable_declaration_statement(node, writer, do_indent)?
        }
        "old_variable_declaration_statement" => {
            write_old_variable_declaration_statement(node, writer, do_indent)?
        }
        "for_loop" => write_for_loop(node, writer, do_indent)?,
        "while_loop" => write_while_loop(node, writer, do_indent)?,
        "do_while_loop" => write_do_while_loop(node, writer, do_indent)?,
        "break_statement" => {
            if do_indent {
                writer.write_indent();
            }
            writer.output.push_str("break");
            writer.output.push(';');
        }
        "continue_statement" => {
            if do_indent {
                writer.write_indent();
            }
            writer.output.push_str("continue");
            writer.output.push(';');
        }
        "condition_statement" => write_condition_statement(node, writer, do_indent)?,
        "switch_statement" => write_switch_statement(node, writer, do_indent)?,
        "return_statement" => write_return_statement(node, writer, do_indent)?,
        "delete_statement" => write_delete_statement(node, writer, do_indent)?,
        "expression_statement" => {
            if do_indent {
                writer.write_indent();
            }
            write_expression_statement(node, writer)?
        }
        _ => write_node(&node, writer)?,
    }
    if do_break {
        // Add another break if the next sibling is not right below/next
        // to the current sibling.
        if !next_sibling.is_none() {
            let st = next_sibling.unwrap().start_position().row();
            if st - sp > 1 {
                writer.breakl();
            }
        }
        writer.breakl();
    }

    Ok(())
}

fn write_for_loop(node: Node, writer: &mut Writer, do_indent: bool) -> Result<(), Utf8Error> {
    let mut end_condition_reached = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "for" => {
                if do_indent {
                    writer.write_indent();
                }
                write_node(&child, writer)?;
            }
            "(" => write_node(&child, writer)?,
            ")" => {
                end_condition_reached = true;
                write_node(&child, writer)?;
            }
            "assignment_expression" => write_expression(child, writer)?,
            ";" => writer.output.push(';'),
            "," => writer.output.push_str(", "),
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if !end_condition_reached {
                        if writer.output.ends_with(";") {
                            writer.output.push(' ');
                        }
                        write_statement(child, writer, false, false)?;
                        continue;
                    }
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
                        writer.indent += 1;
                        write_statement(child, writer, true, false)?;
                        writer.indent -= 1;
                    }
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_while_loop(node: Node, writer: &mut Writer, do_indent: bool) -> Result<(), Utf8Error> {
    let mut end_condition_reached = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "while" => {
                if do_indent {
                    writer.write_indent();
                }
                write_node(&child, writer)?;
            }
            "(" => write_node(&child, writer)?,
            ")" => {
                end_condition_reached = true;
                write_node(&child, writer)?;
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
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
                            writer.indent += 1;
                            write_statement(child, writer, true, false)?;
                            writer.indent -= 1;
                        }
                    } else {
                        write_statement(child, writer, false, false)?;
                    }
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_do_while_loop(node: Node, writer: &mut Writer, do_indent: bool) -> Result<(), Utf8Error> {
    let mut in_condition = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "do" => {
                if do_indent {
                    writer.write_indent();
                }
                writer.output.push_str("do");
            }
            "while" => {
                in_condition = true;
                writer.write_indent();
                writer.output.push_str("while");
            }
            "(" => write_node(&child, writer)?,
            ")" => {
                write_node(&child, writer)?;
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if in_condition {
                        write_statement(child, writer, false, false)?;
                        continue;
                    }
                    if kind == "block" {
                        if writer.settings.brace_wrapping_before_loop {
                            writer.breakl();
                            write_block(child, writer, true)?;
                        } else {
                            writer.output.push(' ');
                            write_block(child, writer, false)?;
                        }
                        writer.breakl();
                    } else {
                        writer.breakl();
                        writer.indent += 1;
                        write_statement(child, writer, true, false)?;
                        writer.indent -= 1;
                    }
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_switch_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "switch" => {
                if do_indent {
                    writer.write_indent();
                }
                writer.output.push_str("switch");
            }
            "(" => write_node(&child, writer)?,
            ")" => {
                write_node(&child, writer)?;
            }
            "{" => {
                if writer.settings.brace_wrapping_before_condition {
                    writer.breakl();
                    writer.write_indent();
                } else {
                    writer.output.push(' ');
                }
                writer.output.push('{');
                writer.breakl();
                writer.indent += 1;
            }
            "}" => {
                writer.indent -= 1;
                writer.write_indent();
                writer.output.push('}');
            }
            "switch_case" => write_switch_case(child, writer)?,
            "switch_default_case" => write_switch_default_case(child, writer)?,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?;
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
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            ":" => {
                writer.output.push_str(":\n");
            }
            "switch_case_values" => write_switch_case_values(child, writer)?,
            "comment" => write_comment(&child, writer)?,
            _ => {
                if kind == "block" {
                    write_statement(child, writer, true, true)?;
                    continue;
                }
                if writer.is_statement(kind.to_string()) {
                    writer.indent += 1;
                    write_statement(child, writer, true, true)?;
                    writer.indent -= 1;
                } else {
                    write_node(&child, writer)?;
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
                write_node(&child, writer)?;
            }
            ":" => {
                writer.output.push_str(":\n");
            }
            _ => {
                if kind == "block" {
                    write_statement(child, writer, true, true)?;
                    continue;
                }
                if writer.is_statement(kind.to_string()) {
                    writer.indent += 1;
                    write_statement(child, writer, true, true)?;
                    writer.indent -= 1;
                } else {
                    write_node(&child, writer)?
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
            "comment" => write_comment(&child, writer)?,
            "symbol" => write_node(&child, writer)?,
            "," => writer.output.push_str(", "),
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?;
                }
            }
        }
    }

    Ok(())
}

fn write_return_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "comment" => write_comment(&child, writer)?,
            "return" => {
                if do_indent {
                    writer.write_indent();
                }
                writer.output.push_str("return ");
            }
            ";" => writer.output.push(';'),
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?
                }
            }
        }
    }

    Ok(())
}

fn write_delete_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "comment" => write_comment(&child, writer)?,
            "delete" => {
                if do_indent {
                    writer.write_indent();
                }
                writer.output.push_str("delete ");
            }
            ";" => writer.output.push(';'),
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?
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
            "comment" => write_comment(&child, writer)?,
            _ => {
                if writer.is_expression(kind.to_string()) {
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?
                }
            }
        }
    }

    Ok(())
}

fn write_condition_statement(
    node: Node,
    writer: &mut Writer,
    do_indent: bool,
) -> Result<(), Utf8Error> {
    let mut out_of_condition = false;
    let mut else_statement = false;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "if" => {
                if writer.output.ends_with("else") {
                    writer.output.push(' ');
                } else if do_indent {
                    writer.write_indent();
                }
                write_node(&child, writer)?;
            }
            "else" => {
                writer.breakl();
                writer.write_indent();
                write_node(&child, writer)?;
                out_of_condition = true;
                else_statement = true;
            }
            "(" => write_node(&child, writer)?,
            ")" => {
                write_node(&child, writer)?;
                out_of_condition = true;
            }
            _ => {
                if writer.is_statement(kind.to_string()) {
                    if out_of_condition {
                        if kind == "block" {
                            if writer.settings.brace_wrapping_before_condition {
                                writer.breakl();
                                write_block(child, writer, true)?;
                            } else {
                                writer.output.push(' ');
                                write_block(child, writer, false)?;
                            }
                        } else {
                            if else_statement && kind == "condition_statement" {
                                write_statement(child, writer, true, false)?;
                                continue;
                            }
                            writer.breakl();
                            writer.indent += 1;
                            write_statement(child, writer, true, false)?;
                            writer.indent -= 1;
                        }
                    } else {
                        write_statement(child, writer, false, false)?;
                    }
                } else if writer.is_expression(kind.to_string()) {
                    if writer.output.ends_with(';') {
                        writer.output.push(' ');
                    }
                    write_expression(child, writer)?;
                } else {
                    write_node(&child, writer)?;
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
                write_node(&child, writer)?;
                writer.breakl();
                writer.indent += 1;
            }
            "}" => {
                writer.indent -= 1;
                writer.write_indent();
                write_node(&child, writer)?;
            }
            "comment" => write_comment(&child, writer)?,
            _ => {
                if writer.is_statement(kind.to_string()) {
                    write_statement(child, writer, true, true)?
                } else {
                    write_node(&child, writer)?
                }
            }
        }
    }

    Ok(())
}
