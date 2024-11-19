use numpy::{PyArray1, PyArrayMethods};
use pyo3::types::{IntoPyDict, PyAnyMethods, PyList};
use pyo3::PyErr;
use pyo3::{types::PyModule, PyRef, Python};
use std::path::Path;

use crate::port::Port;

pub fn initialize_gpipy() -> Result<(), Box<dyn std::error::Error>> {
    //"export" our API module to the python runtime
    //TODO: add python files in gpipy/ at compile time
    //pyo3::append_to_inittab!(pyModule);
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
/// Run a python node's compute function
//pub fn gpipy_compute(
//    node_type: &str,
//    inputs: PyRef<Port>, //HashMap<String, PyRef<Port>>, //HashMap<String, impl IntoPy<PyAny>>,
//) -> Result<HashMap<String, Port>, Box<dyn std::error::Error>> {
//    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
//    let node_src = &fs::read_to_string(path.join(format!("../nodes/{node_type}.py")))?;
//
//    Python::with_gil(|py| {
//        //        py.run_bound(
//        //            r#"
//        //import sys
//        //print(sys.executable, sys.path)
//        //"#,
//        //            None,
//        //            None,
//        //        )
//        //        .unwrap();
//        let node_module = PyModule::from_code_bound(py, node_src, "gpi_node.py", "gpi_node")?;
//        //println!("running py compute");
//        //let input_borrow = input.borrow(py);
//        //// COMPUTE
//        let node_output: Port = node_module
//            .getattr("compute")?
//            .call1((inputs,))?
//            .extract()?;
//
//        Ok(HashMap::from([("out".into(), node_output)]))
//        //// TODO: VIEW
//    })
//}
pub fn create_numpy_array() {
    Python::with_gil(|py| -> Result<(), PyErr> {
        let np = py.import_bound("numpy")?;
        let locals = [("np", np)].into_py_dict_bound(py);

        let pyarray = py
            .eval_bound(
                "np.absolute(np.array([-1, -2, -3], dtype='int32'))",
                Some(&locals),
                None,
            )?
            .downcast_into::<PyArray1<i32>>()?;

        let readonly = pyarray.readonly();
        let slice = readonly.as_slice()?;
        assert_eq!(slice, &[1, 2, 3]);

        Ok(())
    })
    .unwrap();
}
#[cfg(test)]
mod test {
    use super::create_numpy_array;

    #[test]
    fn numpy_simple() {
        create_numpy_array();
    }
}
