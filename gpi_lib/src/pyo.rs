use gpipy::gpipy as pyModule;
pub use gpipy::Value;
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

pub fn initialize_default() {
    let mut path = std::env::current_dir().unwrap();
    path.push("python_plugin");
    let _ = initialize_gpipy(&path);
}

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

pub fn gpipy_compute(
    node_type: &str,
    inputs: Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        // This won't re-import the node, `reload_node` needs to be used
        let node_module = match PyModule::import_bound(py, node_type) {
            Ok(module) => module,
            Err(e) => panic!("Failed to import ${node_type}: ${e}"),
        };

        //        py_run!(
        //            py,
        //            inputs,
        //            &format!(
        //                "from {node_type} import In
        //print(In.__annotations__['a'].__name__)"
        //            )
        //        );
        //let input_types: String = match node_module.getattr("In") {
        //    Ok(in_enum) => in_enum
        //        .getattr("__annotations__['a'].__name__")
        //        .unwrap()
        //        .extract()
        //        .unwrap(),
        //    Err(e) => panic!("Failed to find `In` inside ${node_type}: {e}"),
        //};
        //println!("expected input type for 'a': {input_types}");

        //// INIT
        //let node_rs = GpiNode {
        //    inputs,
        //    out: Value::Integer(0),
        //    config: HashMap::new(),
        //};

        //// COMPUTE
        let compute_output: Value = match node_module.getattr("compute") {
            Ok(compute_fn) => match compute_fn.call1((inputs,)) {
                Ok(out_py) => match out_py.extract::<Value>() {
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
                    Err(e) => panic!("Failed to interperet  ${node_type}'s `compute` output: ${e}"),
                },
                Err(e) => panic!("Failed to run `compute` in ${node_type}: ${e}"),
            },
            Err(e) => panic!("Failed to run `compute` in ${node_type}: ${e}"),
        };
        Ok(config)
        //// TODO: VIEW
    })
}
