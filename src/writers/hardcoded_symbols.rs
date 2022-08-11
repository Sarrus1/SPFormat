use super::{preproc::break_after_statement, write_node, Writer};

use std::{borrow::Borrow, str::Utf8Error};
use tree_sitter::Node;

pub fn write_hardcoded_symbol(node: &Node, writer: &mut Writer) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        let kind = child.kind();
        match kind.borrow() {
            "using __intrinsics__.Handle" => write_node(&child, writer)?,
            ";" => continue,
            _ => println!("Unexpected kind {} in write_hardcoded_symbol.", kind),
        }
    }
    writer.output.push(';');
    break_after_statement(&node, writer);

    Ok(())
}
