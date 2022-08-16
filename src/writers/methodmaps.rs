use std::{borrow::Borrow, str::Utf8Error};

use tree_sitter::Node;

use super::{
    functions::write_argument_declarations, prev_sibling_kind, statements::write_block,
    variables::write_type, write_comment, write_node, Writer,
};

pub fn write_methodmap(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_methodmap).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_")
        && prev_kind != ""
        && prev_kind != "comment"
        && prev_kind != "alias_declaration"
    {
        // Insert new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "methodmap" => writer.output.push_str("methodmap "),
            "symbol" => write_node(&child, writer)?,
            "<" => writer.output.push_str(" < "),
            "__nullable__" => writer.output.push_str(" __nullable__ "),
            "{" => {
                if writer.settings.brace_wrapping_before_methodmap {
                    writer.breakl();
                } else {
                    writer.output.push(' ');
                }
                writer.output.push_str("{\n");
                writer.indent += 1;
            }
            "}" => {
                writer.output.push_str("}");
                writer.indent -= 1;
            }
            "methodmap_alias" => write_methodmap_alias(child, writer)?,
            "methodmap_native" | "methodmap_native_destructor" | "methodmap_native_constructor" => {
                write_methodmap_native(child, writer)?
            }
            "methodmap_method" | "methodmap_method_destructor" | "methodmap_method_constructor" => {
                write_methodmap_method(child, writer)?
            }
            "methodmap_property" => write_methodmap_property(child, writer)?,
            "comment" => write_comment(&child, writer)?,
            ";" => continue,
            _ => {
                println!("Unexpected kind {} in write_methodmap.", kind);
            }
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

fn write_methodmap_alias(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "public" => writer.output.push_str("public "),
            "~" | "(" | ")" | "symbol" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            ";" => continue,
            _ => println!("Unexpected kind {} in write_alias_declaration.", kind),
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

fn write_methodmap_native(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "public" => writer.output.push_str("public "),
            "static" | "native" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "type" => write_type(&child, writer)?,
            "(" | ")" | "symbol" | "~" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            "argument_declarations" => write_argument_declarations(child, writer)?,
            ";" => continue,
            _ => println!("Unexpected kind {} in write_methodmap_native.", kind),
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

fn write_methodmap_method(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "public" => writer.output.push_str("public "),
            "static" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "type" => write_type(&child, writer)?,
            "(" | ")" | "symbol" | "~" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            "argument_declarations" => write_argument_declarations(child, writer)?,
            "block" => {
                if writer.settings.brace_wrapping_before_function {
                    writer.breakl();
                    write_block(child, writer, true)?;
                } else {
                    writer.output.push(' ');
                    write_block(child, writer, false)?;
                }
            }
            _ => println!("Unexpected kind {} in write_methodmap_method.", kind),
        }
    }
    writer.breakl();

    Ok(())
}

fn write_methodmap_property(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "property" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "type" => write_type(&child, writer)?,
            "(" | ")" | "symbol" | "~" => write_node(&child, writer)?,
            "{" => {
                if writer.settings.brace_wrapping_before_methodmap_property {
                    writer.breakl();
                } else {
                    writer.output.push(' ');
                }
                writer.output.push_str("{\n");
                writer.indent += 1;
            }
            "}" => {
                writer.output.push_str("}");
                writer.indent -= 1;
            }
            "=" => writer.output.push_str(" = "),
            "argument_declarations" => write_argument_declarations(child, writer)?,
            "methodmap_property_alias" => write_methodmap_property_alias(child, writer)?,
            "methodmap_property_method" | "methodmap_property_native" => {
                write_methodmap_property_method(child, writer)?
            }
            ";" => continue,
            _ => println!("Unexpected kind {} in write_methodmap_property.", kind),
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

fn write_methodmap_property_alias(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "public" => writer.output.push_str("public "),
            "methodmap_property_getter" => writer.output.push_str("get()"),
            "symbol" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            ";" => continue,
            _ => println!(
                "Unexpected kind {} in write_methodmap_property_alias.",
                kind
            ),
        }
    }
    writer.output.push(';');
    writer.breakl();

    Ok(())
}

fn write_methodmap_property_method(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let nb_lines: usize = usize::try_from(writer.settings.breaks_before_function_def).unwrap();
    let prev_kind = prev_sibling_kind(&node);

    if !prev_kind.starts_with("preproc_") && prev_kind != "" && prev_kind != "comment" {
        // Insert two new lines automatically
        writer.output.push_str("\n".repeat(nb_lines).as_str());
    }

    let mut cursor = node.walk();

    writer.write_indent();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "public" => writer.output.push_str("public "),
            "native" => {
                write_node(&child, writer)?;
                writer.output.push(' ');
            }
            "methodmap_property_getter" => writer.output.push_str("get()"),
            "methodmap_property_setter" => write_methodmap_property_setter(child, writer)?,
            "symbol" => write_node(&child, writer)?,
            "=" => writer.output.push_str(" = "),
            "block" => {
                if writer.settings.brace_wrapping_before_function {
                    writer.breakl();
                    write_block(child, writer, true)?;
                } else {
                    writer.output.push(' ');
                    write_block(child, writer, false)?;
                }
            }
            ";" => writer.output.push(';'),
            _ => println!(
                "Unexpected kind {} in write_methodmap_property_method.",
                kind
            ),
        }
    }
    writer.breakl();

    Ok(())
}

fn write_methodmap_property_setter(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "set" => writer.output.push_str("set"),
            "symbol" | "(" | ")" => write_node(&child, writer)?,
            "type" => write_type(&child, writer)?,
            ";" => continue,
            _ => println!(
                "Unexpected kind {} in write_methodmap_property_setter.",
                kind
            ),
        }
    }

    Ok(())
}
