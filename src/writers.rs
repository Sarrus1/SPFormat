use std::{borrow::Borrow, str::Utf8Error};
use tree_sitter::{Language, Node};

pub struct Writer<'a> {
    pub output: String,
    pub source: &'a [u8],
    pub language: &'a Language,
    pub indent: usize,
    pub indent_string: String,
    pub skip: u8,
}

pub fn utf8_text<'a>(node: Node, source: &'a [u8]) -> Result<&'a str, Utf8Error> {
    std::str::from_utf8(&source[(node.start_byte() as usize)..(node.end_byte() as usize)])
}

pub fn write_preproc_include(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for sub_node in node.children(&mut cursor) {
        match sub_node.kind().borrow() {
            "string_literal" | "system_lib_string" => {
                writer
                    .output
                    .push_str(utf8_text(sub_node, writer.source)?.borrow());
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
            "dimension" => write_dimension(writer),
            ";" => writer.output.push(';'),
            _ => {
                println!("{}", sub_node.kind())
            }
        }
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
                    .push_str(utf8_text(sub_node, writer.source)?.borrow());
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
    let var_type = utf8_text(node.child_by_field_name("type").unwrap(), writer.source)?.borrow();
    let prev_var_type = utf8_text(
        prev_node.child_by_field_name("type").unwrap(),
        writer.source,
    )?
    .borrow();

    if var_type != prev_var_type {
        writer.output.push('\n');
        return Ok(());
    }
    Ok(())
}

fn write_variable_declaration(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let var_name = utf8_text(node.child_by_field_name("name").unwrap(), writer.source)?;
    writer.output.push_str(var_name.borrow());

    let mut cursor = node.walk();
    // Write the dimensions of a declaration, if they exist.
    for sub_child in node.named_children(&mut cursor) {
        match sub_child.kind().borrow() {
            "fixed_dimension" => write_fixed_dimension(sub_child, writer)?,
            "dimension" => write_dimension(writer),
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

fn write_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    match node.kind().borrow() {
        "symbol" | "null" | "this" | "int_literal " | "bool_literal" | "char_literal"
        | "float_literal" | "string_literal" => write_node(node, writer)?,
        "binary_expression" => write_binary_expression(node, writer)?,
        "unary_expression" => write_unary_expression(node, writer)?,
        "update_expression" => write_update_expression(node, writer)?,
        "parenthesized_expression" => write_parenthesized_expression(node, writer)?,
        "comma_expression" => write_comma_expression(node, writer)?,
        "scope_access" => write_scope_access(node, writer)?,
        "view_as" => write_view_as(node, writer)?,
        "old_type_cast" => write_old_type_cast(node, writer)?,
        "ternary_expression" => write_ternary_expression(node, writer)?,
        "concatenated_string" => write_concatenated_string(node, writer)?,
        "array_indexed_access" => write_array_indexed_access(node, writer)?,
        "field_access" => write_field_access(node, writer)?,
        "new_instance" => write_new_instance(node, writer)?,
        "function_call" => write_function_call(node, writer)?,
        "assignment_expression" => write_assignment_expression(node, writer)?,
        "array_literal" => write_array_literal(node, writer)?,
        "sizeof_expression" => write_sizeof_expression(node, writer)?,
        _ => write_node(node, writer)?,
    };
    Ok(())
}

fn write_binary_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("left").unwrap(), writer)?;
    writer.output.push(' ');
    write_node(node.child_by_field_name("operator").unwrap(), writer)?;
    writer.output.push(' ');
    write_expression(node.child_by_field_name("right").unwrap(), writer)?;
    Ok(())
}

fn write_assignment_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("left").unwrap(), writer)?;
    writer.output.push(' ');
    write_node(node.child_by_field_name("operator").unwrap(), writer)?;
    writer.output.push(' ');
    let right_node = node.child_by_field_name("right").unwrap();
    match right_node.kind().borrow() {
        "dynamic_array" => write_dynamic_array(right_node, writer)?,
        _ => write_expression(right_node, writer)?,
    }
    Ok(())
}

fn write_array_indexed_access(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let array_node = node.child_by_field_name("array").unwrap();
    match array_node.kind().borrow() {
        "array_indexed_access" => write_array_indexed_access(array_node, writer)?,
        // TODO: Handle "field_access" here.
        _ => write_node(array_node, writer)?,
    }
    writer.output.push('[');
    write_expression(node.child_by_field_name("index").unwrap(), writer)?;
    writer.output.push(']');
    Ok(())
}

fn write_field_access(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("target").unwrap(), writer)?;
    writer.output.push('.');
    write_node(node.child_by_field_name("field").unwrap(), writer)?;
    Ok(())
}

fn write_new_instance(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer.output.push_str("new ");
    write_node(node.child_by_field_name("class").unwrap(), writer)?;
    write_function_call_arguments(node.child_by_field_name("arguments").unwrap(), writer)?;
    Ok(())
}

fn write_function_call(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let function_node = node.child_by_field_name("function").unwrap();
    match function_node.kind().borrow() {
        "symbol" => write_node(function_node, writer)?,
        "field_access" => write_field_access(function_node, writer)?,
        _ => println!("Unexpected function node."),
    }
    write_function_call_arguments(node.child_by_field_name("arguments").unwrap(), writer)?;
    Ok(())
}

fn write_unary_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_node(node.child_by_field_name("operator").unwrap(), writer)?;
    write_expression(node.child_by_field_name("argument").unwrap(), writer)?;
    Ok(())
}

fn write_parenthesized_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    // TODO: Check for literals/symbols to remove unneeded parenthesis.
    writer.output.push('(');
    let expression_node = node.child_by_field_name("expression").unwrap();
    match expression_node.kind().borrow() {
        "comma_expression" => write_comma_expression(expression_node, writer)?,
        _ => write_expression(expression_node, writer)?,
    }
    writer.output.push(')');
    Ok(())
}

fn write_comma_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("left").unwrap(), writer)?;
    writer.output.push_str(", ");
    write_expression(node.child_by_field_name("right").unwrap(), writer)?;
    Ok(())
}

fn write_concatenated_string(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_node(node.child_by_field_name("left").unwrap(), writer)?;
    writer.output.push_str(" ... ");
    let right_node = node.child_by_field_name("right").unwrap();
    match right_node.kind().borrow() {
        "concatenated_string" => write_concatenated_string(right_node, writer)?,
        _ => write_node(right_node, writer)?,
    }
    Ok(())
}

fn write_update_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let argument_node = node.child_by_field_name("argument").unwrap();
    let operator_node = node.child_by_field_name("operator").unwrap();
    if operator_node.end_position() <= argument_node.start_position() {
        write_node(operator_node, writer)?;
        write_expression(argument_node, writer)?;
    } else {
        write_expression(argument_node, writer)?;
        write_node(operator_node, writer)?;
    }
    Ok(())
}

fn write_ternary_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("condition").unwrap(), writer)?;
    writer.output.push_str(" ? ");
    write_expression(node.child_by_field_name("consequence").unwrap(), writer)?;
    writer.output.push_str(" : ");
    write_expression(node.child_by_field_name("alternative").unwrap(), writer)?;
    Ok(())
}

fn write_scope_access(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("scope").unwrap(), writer)?;
    writer.output.push_str("::");
    write_expression(node.child_by_field_name("field").unwrap(), writer)?;
    Ok(())
}

fn write_view_as(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer.output.push_str("view_as<");
    write_node(node.child_by_field_name("type").unwrap(), writer)?;
    writer.output.push_str(">(");
    write_expression(node.child_by_field_name("value").unwrap(), writer)?;
    writer.output.push(')');
    Ok(())
}

fn write_array_literal(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    writer.output.push_str("{ ");
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "{" | "}" => continue,
            "," => writer.output.push_str(", "),
            _ => write_expression(child, writer)?,
        }
    }
    writer.output.push_str(" }");
    Ok(())
}

fn write_sizeof_expression(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    writer.output.push_str("sizeof ");
    for child in node.children_by_field_name("type", &mut cursor) {
        match child.kind().borrow() {
            "dimension" => write_dimension(writer),
            _ => write_expression(child, writer)?,
        }
    }
    Ok(())
}

fn write_dimension(writer: &mut Writer) {
    writer.output.push_str("[]");
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
        .push_str(utf8_text(node, writer.source)?.borrow());
    writer.output.push_str(": ");
    Ok(())
}

fn write_node(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer
        .output
        .push_str(utf8_text(node, writer.source)?.borrow());
    Ok(())
}

fn write_preproc_arg(node: Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    writer
        .output
        .push_str(utf8_text(node, writer.source)?.borrow().trim());
    Ok(())
}
