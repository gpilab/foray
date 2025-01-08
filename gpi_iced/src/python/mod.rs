use numpy::PyArray1;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use std::ffi::CString;
use std::{collections::HashMap, fs, path::Path};
pub mod ndarray;

#[allow(clippy::complexity)]
/// Run a python node's compute function
pub fn gpipy_compute<'a>(
    node_type: &str,
    inputs: HashMap<String, &Bound<'a, PyArray1<f64>>>,
    py: Python<'a>,
) -> Result<HashMap<String, Bound<'a, PyArray1<f64>>>, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let node_src = fs::read_to_string(path.join(format!("nodes/{node_type}.py")))?;

    let node_module = PyModule::from_code(
        py,
        CString::new(node_src)?.as_c_str(),
        c_str!("gpi_node.py"),
        c_str!("gpi_node"),
    )?;

    //// COMPUTE
    let node_output: Bound<PyArray1<f64>> = node_module
        .getattr("compute")?
        .call1((inputs,))
        .unwrap()
        .downcast_into::<PyArray1<f64>>()
        .unwrap();

    Ok(HashMap::from([("out".into(), node_output)]))
}
