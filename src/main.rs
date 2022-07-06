use std::{fs, str::Utf8Error};
use tree_sitter::{Node, Parser};

fn main() -> Result<(), Utf8Error> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_sourcepawn::language())
        .expect("Error loading SourcePawn grammar");
    let filename = "test.sp";
    let source =
        fs::read_to_string(filename).expect("Something went wrong while reading the file.");
    let source = source.as_bytes();

    let parsed = parser.parse(&source, None).unwrap();
    let mut cursor = parsed.walk();
    let mut output = "".to_string();
    for node in parsed.root_node().children(&mut cursor) {
        let mut buf = String::new();
        match node.kind() {
            "global_variable_declaration" => write_global_variable(node, &mut buf, &source)?,
            _ => buf = node.utf8_text(source)?.to_string(),
        };
        output.push_str(buf.as_str());
    }
    fs::write(filename, output).expect("Something went wrong writing the file.");
    Ok(())
}

fn write_global_variable(node: Node, buf: &mut String, source: &[u8]) -> Result<(), Utf8Error> {
    let mut cursor = node.walk();
    let mut variable_declarations: Vec<Node> = Vec::new();

    // Get the type, storage class, and visibility of the declaration(s).
    for sub_node in node.named_children(&mut cursor) {
        match sub_node.kind() {
            "variable_storage_class" | "variable_visibility" | "type" => {
                buf.push_str(sub_node.utf8_text(source)?);
                buf.push(' ');
            }
            "variable_declaration" => variable_declarations.push(sub_node),
            _ => println!("{}", sub_node.kind()),
        }
    }

    // Iterate over all declarations of this statement.
    // Handle cases such as:
    // `int foo, bar;`
    for child in variable_declarations {
        if !(child.kind() == "variable_declaration") {
            // TODO: Handle comments and preproc statements here.
            continue;
        }
        let var_name = child
            .child_by_field_name("name")
            .unwrap()
            .utf8_text(source)?;
        buf.push_str(var_name);

        let mut cursor = child.walk();
        // Write the dimensions of a declaration, if they exist.
        for sub_child in child.named_children(&mut cursor) {
            if sub_child.kind() == "fixed_dimension" || sub_child.kind() == "dimension" {
                buf.push_str(sub_child.utf8_text(source)?);
            }
        }

        // Write the default value of a declaration, if it exists.
        for sub_child in child.children_by_field_name("initialValue", &mut cursor) {
            if sub_child.kind() == "=" {
                buf.push_str(" = ");
                continue;
            }
            buf.push_str(sub_child.utf8_text(source)?);
            break;
        }
        buf.push_str(", ");
    }

    // Remove the last ", "
    buf.pop();
    buf.pop();
    buf.push_str(";\n");
    Ok(())
}
