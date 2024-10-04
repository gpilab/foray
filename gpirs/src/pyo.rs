use crate::node::{NodeInputType, NodeInputValue, NodeOutputType, NodeOutputValue};
use crate::port::PortValue;
use crate::python_node::gpirs as pyModule;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn initialize_gpipy() -> Result<(), Box<dyn std::error::Error>> {
    //"export" our API module to the python runtime
    //TODO: add python files in gpipy/ at compile time
    pyo3::append_to_inittab!(pyModule);
    //spawn runtime
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let sys = PyModule::import_bound(py, "sys")?;
        let syspath: &PyList = sys.getattr("path")?.extract()?;
        //syspath.insert(0, &path)?;
        syspath.insert(
            0,
            &Path::new(
                "/Users/jechristens3/projects/gpi_v2/gpirs/.venv/lib/python3.12/site-packages/",
            ),
        )?;

        //syspath.insert(0, &path)?;

        Ok(())
    })
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
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let node_src = &fs::read_to_string(path.join(format!("../nodes/{node_type}.py")))?;

    Python::with_gil(|py| {
        //        py.run_bound(
        //            r#"
        //import sys
        //print(sys.executable, sys.path)
        //"#,
        //            None,
        //            None,
        //        )
        //        .unwrap();

        let node_module = PyModule::from_code_bound(py, node_src, "gpi_node.py", "gpi_node")?;
        //println!("running py compute");

        //// COMPUTE
        let node_output = node_module
            .getattr("compute")?
            .call1((inputs,))?
            .extract::<PortValue>()?;

        Ok(NodeOutputValue(HashMap::from([(
            "out".into(),
            node_output,
        )])))
        //// TODO: VIEW
    })
}

/// Get a python node's configuration information
pub fn gpipy_config(
    node_type: &str,
) -> Result<(NodeInputType, NodeOutputType), Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let node_src = &fs::read_to_string(path.join(format!("../nodes/{node_type}.py")))?;

    Python::with_gil(|py| {
        dbg!("trying to load node");
        let node_module = PyModule::from_code_bound(py, node_src, "gpi_node.py", "gpi_node")?;

        dbg!("loaded node! now running..");
        //reload_node(node_type);
        //// get config
        let config = match node_module.getattr("config") {
            Ok(compute_fn) => match compute_fn.call0() {
                Ok(out_py) => match out_py.extract::<(NodeInputType, NodeOutputType)>() {
                    Ok(out_value) => out_value,
                    Err(e) => panic!("Failed to interperet  {node_type}'s `config`: {e}, {out_py}"),
                },
                Err(e) => panic!("Failed to run `config` in {node_type}: {e}"),
            },
            Err(e) => panic!("Failed to run `config` in {node_type}: {e}"),
        };
        Ok(config)
    })
}
