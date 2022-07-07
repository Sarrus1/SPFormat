mod language;
mod parser;

use std::{borrow::Borrow, fs, str::Utf8Error};
use tree_sitter::Node;
use wasm_bindgen::prelude::*;

struct Formatter<'a> {
    output: String,
    source: &'a [u8],
}

async fn main() -> Result<(), Utf8Error> {
    let filename = "test.sp";
    let source =
        fs::read_to_string(filename).expect("Something went wrong while reading the file.");
    fs::write(filename, format_string(&source).await.unwrap())
        .expect("Something went wrong writing the file.");
    Ok(())
}

#[wasm_bindgen]
pub async fn sp_format(input: String) -> Result<String, JsValue> {
    let output = format_string(&input)
        .await
        .expect("An error has occured while generating the SourcePawn code.");
    Ok(output)
}

pub async fn format_string(input: &String) -> anyhow::Result<String> {
    let language = language::sourcepawn().await.unwrap();
    let mut parser = parser::sourcepawn(&language)?;
    let parsed = parser.parse(&input, None)?.unwrap();
    let mut cursor = parsed.walk();
    let mut formatter = Formatter {
        output: String::new(),
        source: input.as_bytes(),
    };
    for node in parsed.root_node().children(&mut cursor) {
        match node.kind().borrow() {
            "global_variable_declaration" => write_global_variable(node, &mut formatter)?,
            _ => formatter
                .output
                .push_str(node.utf8_text(formatter.source)?.borrow()),
        };
    }
    Ok(formatter.output)
}

fn write_global_variable(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    let mut variable_declarations: Vec<Node> = Vec::new();

    // Get the type, storage class, and visibility of the declaration(s).
    for sub_node in node.named_children(&mut cursor) {
        match sub_node.kind().borrow() {
            "variable_storage_class" | "variable_visibility" | "type" => {
                formatter
                    .output
                    .push_str(sub_node.utf8_text(formatter.source)?.borrow());
                formatter.output.push(' ');
            }
            "variable_declaration" => variable_declarations.push(sub_node),
            _ => println!("{}", sub_node.kind()),
        }
    }

    // Iterate over all declarations of this statement.
    // Handle cases such as:
    // `int foo, bar;`
    for child in variable_declarations {
        if !(child.kind().to_string() == "variable_declaration") {
            // TODO: Handle comments and preproc statements here.
            continue;
        }
        let var_name = child
            .child_by_field_name("name")
            .unwrap()
            .utf8_text(formatter.source)?;
        formatter.output.push_str(var_name.borrow());

        let mut cursor = child.walk();
        // Write the dimensions of a declaration, if they exist.
        for sub_child in child.named_children(&mut cursor) {
            match sub_child.kind().borrow() {
                "fixed_dimension" => write_fixed_dimension(sub_child, formatter)?,
                "dimension" => write_dimension(formatter),
                _ => continue,
            }
        }

        // Write the default value of a declaration, if it exists.
        for sub_child in child.children_by_field_name("initialValue", &mut cursor) {
            if sub_child.kind().to_string() == "=" {
                formatter.output.push_str(" = ");
                continue;
            } else if sub_child.kind().to_string() == "dynamic_array" {
                write_dynamic_array(sub_child, formatter)?;
                continue;
            }
            write_expression(sub_child, formatter)?;
            break;
        }
        formatter.output.push_str(", ");
    }

    // Remove the last ", "
    formatter.output.pop();
    formatter.output.pop();
    formatter.output.push_str(";\n");
    Ok(())
}

fn write_dynamic_array(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    formatter.output.push_str("new ");
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "type" => write_node(child, formatter)?,
            // TODO: Handle different cases here.
            _ => write_node(child, formatter)?,
        }
    }
    Ok(())
}

fn write_function_call_arguments(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    formatter.output.push('(');
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "(" | ")" => continue,
            "symbol" | "ignore_argument" => write_node(child, formatter)?,
            "named_arg" => write_named_arg(child, formatter)?,
            _ => write_expression(child, formatter)?,
        }
    }
    // Remove the last ", ".
    formatter.output.pop();
    formatter.output.pop();
    formatter.output.push(')');
    Ok(())
}

fn write_named_arg(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    formatter.output.push('.');
    write_node(node.child_by_field_name("name").unwrap(), formatter)?;
    formatter.output.push_str(" = ");
    // FIXME: Always write_node.
    write_node(node.child_by_field_name("value").unwrap(), formatter)?;
    Ok(())
}

fn write_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    match node.kind().borrow() {
        "symbol" | "null" | "this" | "int_literal " | "bool_literal" | "char_literal"
        | "float_literal" | "string_literal" => write_node(node, formatter)?,
        "binary_expression" => write_binary_expression(node, formatter)?,
        "unary_expression" => write_unary_expression(node, formatter)?,
        "update_expression" => write_update_expression(node, formatter)?,
        "parenthesized_expression" => write_parenthesized_expression(node, formatter)?,
        "comma_expression" => write_comma_expression(node, formatter)?,
        "scope_access" => write_scope_access(node, formatter)?,
        "view_as" => write_view_as(node, formatter)?,
        "old_type_cast" => write_old_type_cast(node, formatter)?,
        "ternary_expression" => write_ternary_expression(node, formatter)?,
        "concatenated_string" => write_concatenated_string(node, formatter)?,
        "array_indexed_access" => write_array_indexed_access(node, formatter)?,
        "field_access" => write_field_access(node, formatter)?,
        "new_instance" => write_new_instance(node, formatter)?,
        "function_call" => write_function_call(node, formatter)?,
        "assignment_expression" => write_assignment_expression(node, formatter)?,
        "array_literal" => write_array_literal(node, formatter)?,
        "sizeof_expression" => write_sizeof_expression(node, formatter)?,
        _ => write_node(node, formatter)?,
    };
    Ok(())
}

fn write_binary_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("left").unwrap(), formatter)?;
    formatter.output.push(' ');
    write_node(node.child_by_field_name("operator").unwrap(), formatter)?;
    formatter.output.push(' ');
    write_expression(node.child_by_field_name("right").unwrap(), formatter)?;
    Ok(())
}

fn write_assignment_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("left").unwrap(), formatter)?;
    formatter.output.push(' ');
    write_node(node.child_by_field_name("operator").unwrap(), formatter)?;
    formatter.output.push(' ');
    let right_node = node.child_by_field_name("right").unwrap();
    match right_node.kind().borrow() {
        "dynamic_array" => write_dynamic_array(right_node, formatter)?,
        _ => write_expression(right_node, formatter)?,
    }
    Ok(())
}

fn write_array_indexed_access(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let array_node = node.child_by_field_name("array").unwrap();
    match array_node.kind().borrow() {
        "array_indexed_access" => write_array_indexed_access(array_node, formatter)?,
        // TODO: Handle "field_access" here.
        _ => write_node(array_node, formatter)?,
    }
    formatter.output.push('[');
    write_expression(node.child_by_field_name("index").unwrap(), formatter)?;
    formatter.output.push(']');
    Ok(())
}

fn write_field_access(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("target").unwrap(), formatter)?;
    formatter.output.push('.');
    write_node(node.child_by_field_name("field").unwrap(), formatter)?;
    Ok(())
}

fn write_new_instance(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    formatter.output.push_str("new ");
    write_node(node.child_by_field_name("class").unwrap(), formatter)?;
    write_function_call_arguments(node.child_by_field_name("arguments").unwrap(), formatter)?;
    Ok(())
}

fn write_function_call(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let function_node = node.child_by_field_name("function").unwrap();
    match function_node.kind().borrow() {
        "symbol" => write_node(function_node, formatter)?,
        "field_access" => write_field_access(function_node, formatter)?,
        _ => println!("Unexpected function node."),
    }
    write_function_call_arguments(node.child_by_field_name("arguments").unwrap(), formatter)?;
    Ok(())
}

fn write_unary_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_node(node.child_by_field_name("operator").unwrap(), formatter)?;
    write_expression(node.child_by_field_name("argument").unwrap(), formatter)?;
    Ok(())
}

fn write_parenthesized_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    // TODO: Check for literals/symbols to remove unneeded parenthesis.
    formatter.output.push('(');
    let expression_node = node.child_by_field_name("expression").unwrap();
    match expression_node.kind().borrow() {
        "comma_expression" => write_comma_expression(expression_node, formatter)?,
        _ => write_expression(expression_node, formatter)?,
    }
    formatter.output.push(')');
    Ok(())
}

fn write_comma_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("left").unwrap(), formatter)?;
    formatter.output.push_str(", ");
    write_expression(node.child_by_field_name("right").unwrap(), formatter)?;
    Ok(())
}

fn write_concatenated_string(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_node(node.child_by_field_name("left").unwrap(), formatter)?;
    formatter.output.push_str(" ... ");
    let right_node = node.child_by_field_name("right").unwrap();
    match right_node.kind().borrow() {
        "concatenated_string" => write_concatenated_string(right_node, formatter)?,
        _ => write_node(right_node, formatter)?,
    }
    Ok(())
}

fn write_update_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let argument_node = node.child_by_field_name("argument").unwrap();
    let operator_node = node.child_by_field_name("operator").unwrap();
    if operator_node.end_position() <= argument_node.start_position() {
        write_node(operator_node, formatter)?;
        write_expression(argument_node, formatter)?;
    } else {
        write_expression(argument_node, formatter)?;
        write_node(operator_node, formatter)?;
    }
    Ok(())
}

fn write_ternary_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("condition").unwrap(), formatter)?;
    formatter.output.push_str(" ? ");
    write_expression(node.child_by_field_name("consequence").unwrap(), formatter)?;
    formatter.output.push_str(" : ");
    write_expression(node.child_by_field_name("alternative").unwrap(), formatter)?;
    Ok(())
}

fn write_scope_access(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_expression(node.child_by_field_name("scope").unwrap(), formatter)?;
    formatter.output.push_str("::");
    write_expression(node.child_by_field_name("field").unwrap(), formatter)?;
    Ok(())
}

fn write_view_as(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    formatter.output.push_str("view_as<");
    write_node(node.child_by_field_name("type").unwrap(), formatter)?;
    formatter.output.push_str(">(");
    write_expression(node.child_by_field_name("value").unwrap(), formatter)?;
    formatter.output.push(')');
    Ok(())
}

fn write_array_literal(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    formatter.output.push_str("{ ");
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "{" | "}" => continue,
            "," => formatter.output.push_str(", "),
            _ => write_expression(child, formatter)?,
        }
    }
    formatter.output.push_str(" }");
    Ok(())
}

fn write_sizeof_expression(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    formatter.output.push_str("sizeof ");
    for child in node.children_by_field_name("type", &mut cursor) {
        match child.kind().borrow() {
            "dimension" => write_dimension(formatter),
            _ => write_expression(child, formatter)?,
        }
    }
    Ok(())
}

fn write_dimension(formatter: &mut Formatter) {
    formatter.output.push_str("[]");
}

fn write_fixed_dimension(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    formatter.output.push('[');
    for child in node.children(&mut cursor) {
        match child.kind().borrow() {
            "[" | "]" => continue,
            _ => write_expression(child, formatter)?,
        }
    }
    formatter.output.push(']');
    Ok(())
}

fn write_old_type_cast(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    write_old_type(node.child_by_field_name("type").unwrap(), formatter)?;
    write_expression(node.child_by_field_name("value").unwrap(), formatter)?;
    Ok(())
}

fn write_old_type(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    formatter
        .output
        .push_str(node.utf8_text(formatter.source)?.borrow());
    formatter.output.push_str(": ");
    Ok(())
}

fn write_node(node: Node, formatter: &mut Formatter) -> Result<(), Utf8Error> {
    formatter
        .output
        .push_str(node.utf8_text(formatter.source)?.borrow());
    Ok(())
}
