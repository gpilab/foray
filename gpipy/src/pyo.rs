use crate::python_node::{gpipy as pyModule, PyPortValue, PyPrimitiveValue};
use gpi_framework::port::{PortValue, PrimitiveValue};
use pyo3::prelude::*;
use pyo3::types::PyList;
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
    path.push("python_plugin");
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
    inputs: Vec<PortValue<PrimitiveValue>>,
) -> Result<PyPortValue, Box<dyn std::error::Error>> {
    let py_inputs = inputs.into_iter().map(|input| match input {
        PortValue::Vec1(val) => PyPortValue(val),
    });
    Python::with_gil(|py| {
        // This won't re-import the node, `reload_node` needs to be used
        let node_module = match PyModule::import_bound(py, node_type) {
            Ok(module) => module,
            Err(e) => panic!("Failed to import ${node_type}: ${e}"),
        };

        //// COMPUTE
        let compute_output: PyPortValue = match node_module.getattr("compute") {
            Ok(compute_fn) => match compute_fn.call1((inputs,)) {
                Ok(out_py) => match out_py.extract::<PyPortValue>() {
                    Ok(out_value) => out_value,
                    Err(e) => panic!("Failed to interperet  ${node_type}'s `compute` output: ${e}"),
                },
                Err(e) => panic!("Failed to run `compute` in ${node_type}: ${e}"),
            },
            Err(e) => panic!("Failed to run `compute` in ${node_type}: ${e}"),
        };
        Ok(compute_output)
        //// TODO: VIEW
    })
}

/// Get a python node's configuration information
pub fn gpipy_config(
    node_type: &str,
) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        // This won't re-import the node, `reload_node` needs to be used
        let node_module = match PyModule::import_bound(py, node_type) {
            Ok(module) => module,
            Err(e) => panic!("Failed to import ${node_type}: ${e}"),
        };

        //// get config
        let config = match node_module.getattr("config") {
            Ok(compute_fn) => match compute_fn.call0() {
                Ok(out_py) => match out_py.extract::<(Vec<String>, Vec<String>)>() {
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
