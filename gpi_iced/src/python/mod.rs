use log::trace;
use py_node::{PortDef, PyFacingPortDef, PyNode};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CString;
use std::{collections::HashMap, fs, path::Path};

use crate::nodes::status::NodeError;
use crate::OrderMap;
pub mod ndarray;
pub mod py_node;

#[allow(clippy::complexity)]
/// Run a python node's compute function
pub fn gpipy_compute<'a>(
    node_type: &str,
    inputs: &OrderMap<String, PyObject>,
    py: Python<'a>,
) -> Result<OrderMap<String, PyObject>, NodeError> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    //PERF: cache this in the PyNode
    let node_src = fs::read_to_string(path.join(format!("nodes/{node_type}.py")))
        .map_err(|e| NodeError::FileSys(e.to_string()))?;
    trace!("Running '{node_type}' compute:\n{node_src}");
    //PERF: test if caching this is a big performance win
    //This would be more of a pain to cache becaues of the associated python lifetime, but could
    //potentially be worth it
    //Update: It may be possible to package the py type without a lifetime? pyo3 docs

    let node_module = PyModule::from_code(
        py,
        CString::new(node_src)
            .map_err(|e| NodeError::FileSys(e.to_string()))?
            .as_c_str(),
        &CString::new(format!("{}.py", node_type))
            .expect("Node names should not contain invalid characters"),
        c_str!("gpi_node"),
    )
    .map_err(|e| NodeError::Syntax(e.to_string()))?;

    //// COMPUTE
    let node_output = node_module
        .getattr("compute")
        .map_err(|_| NodeError::MissingCompute("Could not find compute function".to_string()))?
        .call1((inputs.iter().collect::<HashMap<_, _>>(),))
        .map_err(|e| NodeError::Runtime(e.to_string()))?;

    Ok(OrderMap::from([("out".into(), node_output.into())]))
}

/// Get a python node's configuration information
pub fn gpipy_config(node_name: &str) -> PyNode {
    let path = PyNode::py_node_path(node_name);
    let node_src = match fs::read_to_string(&path).map_err(|e| NodeError::FileSys(e.to_string())) {
        Ok(src) => src,
        Err(_) => {
            let py_node = PyNode {
                name: node_name.to_string(),
                path,
                ports: Err(NodeError::FileSys("could not find src file".into())),
            };
            log::error!("Failed to load node {node_name} {py_node:?}");
            return py_node;
        }
    };

    let ports = gpipy_read_config(node_name, &node_src);

    if ports.is_err() {
        log::error!("Error reading port configuration {ports:?}")
    }

    PyNode {
        name: node_name.to_string(),
        path: path.to_path_buf(),
        ports,
    }
}

pub fn gpipy_read_config(node_type: &str, node_src: &str) -> Result<PortDef, NodeError> {
    Python::with_gil(|py| {
        trace!("Reading node config '{node_type}'");

        //TODO Clean up error handling
        let node_module = PyModule::from_code(
            py,
            CString::new(node_src)
                .map_err(|e| {
                    NodeError::Syntax(format!("Error in node '{node_type}' source text {e}"))
                })?
                .as_c_str(),
            CString::new(format!("{node_type}.py"))
                .map_err(|e| NodeError::Syntax(format!("Error with node name {node_type}{e}")))?
                .as_c_str(),
            CString::new(node_type)
                .map_err(|e| NodeError::Syntax(format!("Error with node name {node_type}{e}")))?
                .as_c_str(),
        )
        .map_err(|e| NodeError::Syntax(format!("Error in node '{node_type}' source text {e}")))?;

        //// get config
        match node_module
            .getattr("config")
            .map_err(|_e| {
                NodeError::Config(format!("Error getting node '{node_type}' configuration"))
            })?
            .call0()
        {
            Ok(out_py) => match out_py.extract::<PyFacingPortDef>() {
                Ok(out_value) => out_value.try_into(),
                Err(e) => panic!("Failed to interperet  {node_type}'s `config`: {e}, {out_py}"),
            },
            Err(e) => panic!("Failed to run `config` in {node_type}: {e}"),
        }
    })
}

#[cfg(test)]
mod test {
    use pyo3::prepare_freethreaded_python;

    use crate::nodes::port::PortType;

    use super::*;
    #[test]
    fn simple_config() {
        prepare_freethreaded_python();

        let port_def = gpipy_read_config(
            "test",
            r#"
def config():
    class out:
        inputs = {"p1": "Real", "p2": "Real2d"}
        outputs = {"out": "Complex"}
    return out
            "#,
        );

        assert_eq!(
            PortDef {
                inputs: [
                    ("p1".to_string(), PortType::Real),
                    ("p2".to_string(), PortType::Real2d)
                ]
                .into(),
                outputs: [("out".to_string(), PortType::Complex),].into()
            },
            port_def.expect("valid config")
        );
    }
}
