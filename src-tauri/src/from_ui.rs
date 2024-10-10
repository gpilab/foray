use std::path::PathBuf;

use gpirs::{
    node::{NodeInputValue, NodeOutputValue, PythonNode},
    pyo::gpipy_compute,
};

use serde::Deserialize;
use tauri::api::dir::read_dir;

#[derive(Deserialize)]
pub struct RunNodeMessage<'a> {
    /// name of the python node module. i.e. "add_int"
    node_type: &'a str,
    /// list of input `Values` to pass to `node_type`'s python "compute" function
    inputs: NodeInputValue, //output: Vec<Value>,
}

#[tauri::command]
pub fn run_node(message: RunNodeMessage) -> Result<NodeOutputValue, String> {
    //println!(
    //    "node type: {}, inputs: {:?}",
    //    message.node_type, message.inputs
    //);
    //match res {
    //    Value::Other(v) => panic!("Unexpected return value from python: {}", v),
    //    _ => res,
    //}
    //TODO: Handle errors more discretely/descriptevly
    gpipy_compute(message.node_type, message.inputs).map_err(|op| op.to_string())
}

#[tauri::command]
pub fn get_python_nodes() -> Vec<PythonNode> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../nodes/");

    read_dir(d.clone(), true)
        .unwrap()
        .into_iter()
        .filter_map(|disk_entry| match disk_entry.path.extension() {
            Some(ext) => {
                if ext == "py" {
                    Some(PythonNode::new(&disk_entry.path))
                } else {
                    None
                }
            }
            None => None,
        })
        .filter_map(|r| {
            r.map_err(|e| println!("Failed to load python file {}", e))
                .ok()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use gpirs::{port::PortValue, pyo::initialize_gpipy};

    use super::*;
    #[test]
    fn add_int() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("../nodes/");

        let inputs = NodeInputValue(HashMap::from([
            ("a".into(), PortValue::Integer(1)),
            ("b".into(), PortValue::Integer(3)),
        ]));

        let _ = initialize_gpipy();
        let _result = match gpipy_compute("add_int", inputs) {
            Ok(res) => res,
            //Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output =
            NodeOutputValue(HashMap::from([("out".into(), PortValue::Integer(4))]));
        assert_eq!(expected_output, _result);
    }
}
