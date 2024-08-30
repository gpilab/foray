use std::path::PathBuf;

use gpipy::{node::PythonNode, pyo::gpipy_compute, python_node::Value};

use serde::Deserialize;
use tauri::api::dir::read_dir;

#[derive(Deserialize)]
pub struct RunNodeMessage<'a> {
    /// name of the python node module. i.e. "add_int"
    node_type: &'a str,
    /// list of input `Values` to pass to `node_type`'s python "compute" function
    inputs: Vec<Value>,
    //output: Vec<Value>,
}

#[tauri::command]
pub fn run_node(message: RunNodeMessage) -> Value {
    println!(
        "node type: {}, inputs: {:?}",
        message.node_type, message.inputs
    );
    let res = gpipy_compute(message.node_type, message.inputs).unwrap();
    match res {
        Value::Other(v) => panic!("Unexpected return value from python: {}", v),
        _ => res,
    }
}

#[tauri::command]
pub fn get_python_nodes() -> Vec<PythonNode> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../gpipy/python_plugin/");

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
        .into_iter()
        .filter_map(|r| {
            r.map_err(|e| println!("Failed to load python file {}", e))
                .ok()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use gpipy::pyo::initialize_gpipy;

    use super::*;
    #[test]
    fn add_int() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("../gpipy/python_plugin/");
        let _ = initialize_gpipy(&d);
        let _result = match gpipy_compute("add_int", vec![Value::Integer(1), Value::Integer(3)]) {
            Ok(Value::Integer(res)) => res,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert_eq!(4, _result);
    }
}
