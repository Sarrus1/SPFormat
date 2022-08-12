use super::{
    expressions::write_function_call_arguments, preproc::insert_break, write_comment, write_node,
    Writer,
};

use std::{borrow::Borrow, str::Utf8Error};
use tree_sitter::Node;

pub fn write_assertion(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "assert" | "static_assert" => write_node(&child, writer)?,
            "function_call_arguments" => write_function_call_arguments(child, writer)?,
            "comment" => write_comment(child, writer)?,
            ";" => continue,
            _ => println!("Unexpected kind {} in write_assertion.", kind),
        }
    }
    writer.output.push(';');
    insert_break(&node, writer);

    Ok(())
}
