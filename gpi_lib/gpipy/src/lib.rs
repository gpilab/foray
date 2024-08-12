use std::collections::HashMap;

use pyo3::prelude::*;

///this is our Gadget that python plugin code can create, and rust app can then access natively.
#[pyclass]
pub struct Gadget {
    #[pyo3(get, set)]
    pub prop: usize,
    //this field will only be accessible to rust code
    pub rustonly: Vec<usize>,
}

#[pyclass]
pub struct GpiNode {
    #[pyo3(get)]
    pub a: i32,

    #[pyo3(get)]
    pub b: i32,

    #[pyo3(get, set)]
    pub out: i32,

    #[pyo3(get, set)]
    pub config: HashMap<String, String>,
}

#[pymethods]
impl GpiNode {
    #[new]
    fn new() -> Self {
        GpiNode {
            a: 13,
            b: 17,
            out: 19,
            config: HashMap::<String, String>::new(),
        }
    }
}

#[pymethods]
impl Gadget {
    #[new]
    fn new() -> Self {
        Gadget {
            prop: 777,
            rustonly: Vec::new(),
        }
    }

    fn push(&mut self, v: usize) {
        self.rustonly.push(v);
    }
}

/// A Python module for interfacing with GPI
#[pymodule]
pub fn gpipy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Gadget>()?;
    m.add_class::<GpiNode>()?;
    Ok(())
}
