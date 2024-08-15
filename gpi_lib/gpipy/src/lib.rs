use std::collections::HashMap;

use pyo3::prelude::*;

#[derive(FromPyObject, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Integer(i32),
    Vec1(Vec<f32>),
    String(String),
    #[pyo3(transparent)]
    Other(String),
}

//type GpiPortData = Option<Box<PortData>>;
impl ToPyObject for Value {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Self::Integer(val) => val.into_py(py),
            Self::String(val) => val.into_py(py),
            Self::Vec1(val) => val.clone().into_py(py),
            Self::Other(_) => py.None(),
        }
    }
}
impl IntoPy<PyObject> for Value {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Integer(val) => val.into_py(py),
            Self::String(val) => val.into_py(py),
            Self::Vec1(val) => val.into_py(py),
            Self::Other(_) => py.None(),
        }
    }
}

#[pyclass]
pub struct GpiNode {
    #[pyo3(get)]
    pub inputs: Vec<Value>,

    #[pyo3(get, set)]
    pub out: Value,

    #[pyo3(get, set)]
    pub config: HashMap<String, String>,
}

/// A Python module for interfacing with GPI
#[pymodule]
pub fn gpipy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GpiNode>()?;
    Ok(())
}
