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
    trace!("running '{node_type}' compute:\n{node_src}");
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
            return PyNode {
                name: node_name.to_string(),
                path,
                ports: Err(NodeError::FileSys("could find src file".into())),
            }
        }
    };

    let ports =
        gpipy_read_config(node_name, &node_src).map_err(|e| NodeError::Runtime(e.to_string()));

    PyNode {
        name: node_name.to_string(),
        path: path.to_path_buf(),
        ports,
    }
}

pub fn gpipy_read_config(
    node_type: &str,
    node_src: &str,
) -> Result<PortDef, Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        trace!("reading node '{node_type}' config with src:\n{node_src}");
        let node_module = PyModule::from_code(
            py,
            CString::new(node_src)?.as_c_str(),
            c_str!("gpi_node.py"),
            c_str!("gpi_node"),
        )?;

        //reload_node(node_type);
        //// get config
        Ok(match node_module.getattr("config") {
            Ok(compute_fn) => match compute_fn.call0() {
                Ok(out_py) => match out_py.extract::<PyFacingPortDef>() {
                    Ok(out_value) => out_value.try_into()?,
                    Err(e) => panic!("Failed to interperet  {node_type}'s `config`: {e}, {out_py}"),
                },
                Err(e) => panic!("Failed to run `config` in {node_type}: {e}"),
            },
            Err(e) => panic!("Failed to run `config` in {node_type}: {e}"),
        })
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
