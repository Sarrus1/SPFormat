use std::fs;
use tree_sitter;

fn main() {
    let mut parser = tree_sitter::Parser::new();
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

fn write_global_variable(node: tree_sitter::Node, buf: &mut String, source: &String) {
    let var_type = node
        .child_by_field_name("type")
        .unwrap()
        .utf8_text(source.as_ref())
        .unwrap();
    buf.push_str(var_type);
    buf.push(' ');
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if !(child.kind() == "variable_declaration") {
            continue;
        }
        let var_name = child
            .child_by_field_name("name")
            .unwrap()
            .utf8_text(source.as_ref())
            .unwrap();
        buf.push_str(var_name);
        let mut cursor = child.walk();
        let var_init_node = child.children_by_field_name("initialValue", &mut cursor);
        for sub_child in var_init_node {
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
