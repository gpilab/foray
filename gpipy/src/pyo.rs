use crate::node::{NodeInputType, NodeInputValue, NodeOutputType, NodeOutputValue};
use crate::port::PortValue;
use crate::python_node::gpipy as pyModule;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;
use std::path::Path;

pub fn initialize_gpipy(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    //"export" our API module to the python runtime
    pyo3::append_to_inittab!(pyModule);
    //spawn runtime
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        //add the current directory to import path of Python
        //#[allow(deprecated)]
        let syspath: &PyList = py.import_bound("sys")?.getattr("path")?.extract()?;
        syspath.insert(0, &path)?;
        Ok(())
    })
}
//TODO: Use proper path (from user config)
pub fn initialize_default() {
    let mut path = std::env::current_dir().unwrap();
    path.push("../nodes/");
    let _ = initialize_gpipy(&path);
}

//TODO: Do this automatically when python source files change
pub fn reload_node(node_type: &str) {
    Python::with_gil(|py| {
        let _ = py.run_bound(
            &format!(
                r#"
import importlib
import {0}
importlib.reload({0})
"#,
                node_type
            ),
            None,
            None,
        );
    });
}

/// Run a python node's compute function
pub fn gpipy_compute(
    node_type: &str,
    inputs: NodeInputValue,
) -> Result<NodeOutputValue, Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        // This won't re-import the node, `reload_node` needs to be used
        let node_module = match PyModule::import_bound(py, node_type) {
            Ok(module) => module,
            Err(e) => panic!("Failed to import ${node_type}: ${e}"),
        };
        println!("running py compute");

        //// COMPUTE
        let compute_output: PortValue = match node_module.getattr("compute") {
            Ok(compute_fn) => match compute_fn.call1((inputs,)) {
                Ok(out_py) => match out_py.extract::<PortValue>() {
                    Ok(out_value) => out_value,
                    Err(e) => panic!("Failed to interperet  ${node_type}'s `compute` output: ${e}"),
                },
                Err(e) => panic!("Failed to run `compute` in ${node_type}: ${e}"),
            },
            Err(e) => panic!("Failed to run `compute` in ${node_type}: ${e}"),
        };
        println!("finished compute");
        Ok(NodeOutputValue(HashMap::from([(
            "out".into(),
            compute_output,
        )])))
        //// TODO: VIEW
    })
}

/// Get a python node's configuration information
pub fn gpipy_config(
    node_type: &str,
) -> Result<(NodeInputType, NodeOutputType), Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        // This won't re-import the node, `reload_node` needs to be used
        let node_module = match PyModule::import_bound(py, node_type) {
            Ok(module) => module,
            Err(e) => panic!("Failed to import ${node_type}: ${e}"),
        };

        //// get config
        let config = match node_module.getattr("config") {
            Ok(compute_fn) => match compute_fn.call0() {
                Ok(out_py) => match out_py.extract::<(NodeInputType, NodeOutputType)>() {
                    Ok(out_value) => out_value,
                    Err(e) => panic!("Failed to interperet  ${node_type}'s `config`: ${e}"),
                },
                Err(e) => panic!("Failed to run `config` in ${node_type}: ${e}"),
            },
            Err(e) => panic!("Failed to run `config` in ${node_type}: ${e}"),
        };
        Ok(config)
    })
}
