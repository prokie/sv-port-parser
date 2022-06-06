
// use pyo3::prelude::*;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn sv_port_parser(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }



use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use sv_parser::{parse_sv, unwrap_node, Locate, RefNode};

fn main() {

    // The path of SystemVerilog source file
    let path = PathBuf::from(r"C:\repositories\sv-port-parser\my_mux.sv");
    // The list of defined macros
    let defines = HashMap::new();
    // The list of include paths
    let includes: Vec<PathBuf> = Vec::new();

    // Parse
    let result = parse_sv(&path, &defines, &includes, false);

    if let Ok((syntax_tree, _)) = result {
        // &SyntaxTree is iterable
        for node in &syntax_tree {
            // The type of each node is RefNode
            match node {
                RefNode::ModuleDeclarationNonansi(x) => {
                    // unwrap_node! gets the nearest ModuleIdentifier from x
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();

                    let id = get_identifier(id).unwrap();

                    // Original string can be got by SyntaxTree::get_str(self, locate: &Locate)
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);
                }
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    println!("module: {}", id);
                }
                _ => (),
            }
        }
    } else {
        println!("Parse failed");
    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}