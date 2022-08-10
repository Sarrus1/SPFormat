use super::parser;
use crate::{
    settings::Settings,
    writers::{self, source_file::write_source_file},
};
use std::collections::HashSet;
use tree_sitter::Language;

pub fn format_string_language(
    input: &String,
    language: Language,
    settings: &Settings,
) -> anyhow::Result<String> {
    let mut parser = parser::sourcepawn(&language)?;
    let parsed = parser.parse(&input, None)?.unwrap();
    if parsed.root_node().has_error() {
        // Do not try to format, there is an error in the syntax.
        return Ok("".to_string());
    }
    #[cfg(debug_assertions)]
    println!("{}", parsed.root_node().to_sexp());
    let mut writer = writers::Writer {
        output: String::new(),
        source: input.as_bytes(),
        language: &language,
        indent: 0,
        indent_string: "\t".to_string(),
        skip: 0,
        settings: settings,
        _statement_kinds: HashSet::new(),
        _expression_kinds: HashSet::new(),
        _literal_kinds: HashSet::new(),
    };
    build_writer(&mut writer);
    write_source_file(parsed.root_node(), &mut writer)?;
    Ok(writer.output)
}

fn build_writer(writer: &mut writers::Writer) {
    let _statement_kinds = vec![
        "block",
        "variable_declaration_statement",
        "old_variable_declaration_statement",
        "for_loop",
        "while_loop",
        "do_while_loop",
        "break_statement",
        "continue_statement",
        "condition_statement",
        "switch_statement",
        "return_statement",
        "delete_statement",
        "expression_statement",
    ];
    for kind in _statement_kinds {
        writer._statement_kinds.insert(kind.to_string());
    }
    let _expression_kinds = vec![
        "assignment_expression",
        "function_call",
        "array_indexed_access",
        "ternary_expression",
        "field_access",
        "scope_access",
        "binary_expression",
        "unary_expression",
        "update_expression",
        "sizeof_expression",
        "view_as",
        "old_type_cast",
        "symbol",
        "parenthesized_expression",
        "this",
        "new_instance",
    ];
    for kind in _expression_kinds {
        writer._expression_kinds.insert(kind.to_string());
    }

    let _literal_kinds = vec![
        "int_literal",
        "float_literal",
        "char_literal",
        "string_literal",
        "concatenated_string",
        "bool_literal",
        "array_literal",
        "null",
    ];
    for kind in _literal_kinds {
        writer._literal_kinds.insert(kind.to_string());
    }
}
