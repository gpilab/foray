use std::{collections::HashMap, fs, path::Path};

use numpy::PyArray1;
use pyo3::{
    types::{PyAnyMethods, PyModule},
    Bound, Python,
};

use crate::port::Port;
mod py_ndarray;

/// Run a python node's compute function
pub fn gpipy_compute<'a>(
    node_type: &str,
    inputs: HashMap<String, &Bound<'a, PyArray1<i32>>>, //HashMap<String, PyRef<Port>>, //HashMap<String, impl IntoPy<PyAny>>,
    py: Python<'a>,
) -> Result<HashMap<String, Bound<'a, PyArray1<i32>>>, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let node_src = &fs::read_to_string(path.join(format!("../nodes/{node_type}.py")))?;

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
    //let input_borrow = input.borrow(py);
    //// COMPUTE
    let node_output: Bound<PyArray1<i32>> = node_module
        .getattr("compute")?
        .call1((inputs,))
        .unwrap()
        .downcast_into::<PyArray1<i32>>()
        .unwrap();

    Ok(HashMap::from([("out".into(), node_output)]))
}
//// TODO: VIEW
