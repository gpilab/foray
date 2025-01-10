use py_node::{PortDef, PyFacingPortDef, PyNode};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CString;
use std::{collections::HashMap, fs, path::Path};

use crate::OrderMap;
pub mod ndarray;
pub mod py_node;

#[allow(clippy::complexity)]
/// Run a python node's compute function
pub fn gpipy_compute<'a>(
    node_type: &str,
    inputs: &OrderMap<String, PyObject>,
    py: Python<'a>,
) -> Result<OrderMap<String, PyObject>, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let node_src = fs::read_to_string(path.join(format!("nodes/{node_type}.py")))?;

    let node_module = PyModule::from_code(
        py,
        CString::new(node_src)?.as_c_str(),
        c_str!("gpi_node.py"),
        c_str!("gpi_node"),
    )?;

    //// COMPUTE
    let node_output = match node_module
        .getattr("compute")?
        .call1((inputs.iter().collect::<HashMap<_, _>>(),))
        //.downcast_into::<PyArrayDyn<f64>>()
    {
        Ok(out) => out,
        Err(err) => {
            panic!("{:?}", err);
        }
    };

    Ok(OrderMap::from([("out".into(), node_output.into())]))
}

/// Get a python node's configuration information
pub fn gpipy_config(node_name: &str) -> Result<PyNode, Box<dyn std::error::Error>> {
    let path = PyNode::py_node_path(node_name);
    let node_src = fs::read_to_string(&path)?;
    let ports = gpipy_read_config(node_name, &node_src)?;

    Ok(PyNode {
        path: path.to_path_buf(),
        ports,
    })
}

pub fn gpipy_read_config(
    node_type: &str,
    node_src: &str,
) -> Result<PortDef, Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        dbg!("trying to load node");
        let node_module = PyModule::from_code(
            py,
            CString::new(dbg!(node_src))?.as_c_str(),
            c_str!("gpi_node.py"),
            c_str!("gpi_node"),
        )?;

        dbg!("loaded node! now running..");
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

    use crate::nodes::PortType;

    use super::*;
    #[test]
    fn simple_config() {
        prepare_freethreaded_python();

        let port_def = gpipy_read_config(
            "test",
            r#"
def config():
    class out:
        inputs = {"a": "Real", "b": "Real2d"}
        outputs = {"out": "Complex"}
    return out
            "#,
        );

        assert_eq!(
            PortDef {
                inputs: [
                    ("a".to_string(), PortType::Real),
                    ("b".to_string(), PortType::Real2d)
                ]
                .into(),
                outputs: [("out".to_string(), PortType::Complex),].into()
            },
            port_def.expect("valid config")
        );
    }
}
