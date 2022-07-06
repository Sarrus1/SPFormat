use std::fs;
use tree_sitter::{Parser, Node};

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_sourcepawn::language())
        .expect("Error loading SourcePawn grammar");
    let filename = "test.sp";
    let source = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let parsed = parser.parse(&source, None).unwrap();
    let mut cursor = parsed.walk();
    let mut output = "".to_string();
    for node in parsed.root_node().children(&mut cursor) {
        let mut buf = String::new();
        match node.kind() {
            "global_variable_declaration" => write_global_variable(node, &mut buf, &source),
            _ => buf = node.utf8_text(source.as_ref()).unwrap().to_string(),
        };
        output.push_str(buf.as_str());
    }
    fs::write(filename, output).expect("Something went wrong writing the file.");
}

fn write_global_variable(node: Node, buf: &mut String, source: &String) {
    let var_type = node
        .child_by_field_name("type")
        .unwrap()
        .utf8_text(source.as_ref())
        .unwrap();
    buf.push_str(var_type);
    buf.push(' ');
    let mut cursor = node.walk();

    /*
     * Iterate over all declarations of this statement.
     * Handle cases such as:
     * int foo, bar;
     */
    for child in node.children(&mut cursor) {
        if !(child.kind() == "variable_declaration") {
            // TODO: Handle comments and preproc statements here.
            continue;
        }
        let var_name = child
            .child_by_field_name("name")
            .unwrap()
            .utf8_text(source.as_ref())
            .unwrap();
        buf.push_str(var_name);

        let mut cursor = child.walk();
        // Write the dimensions of a declaration, if they exist.
        for sub_child in child.named_children(&mut cursor) {
            if sub_child.kind() == "fixed_dimension" || sub_child.kind() == "dimension" {
                buf.push_str(sub_child.utf8_text(source.as_ref()).unwrap());
            }
        }

        // Write the default value of a declaration, if it exists.
        for sub_child in child.children_by_field_name("initialValue", &mut cursor) {
            if sub_child.kind() == "=" {
                buf.push_str(" = ");
                continue;
            }
            buf.push_str(sub_child.utf8_text(source.as_ref()).unwrap());
            break;
        }
        buf.push_str(", ");
    }
    // Remove the last ", "
    buf.pop();
    buf.pop();
    buf.push(';');
    buf.push('\n');
}
