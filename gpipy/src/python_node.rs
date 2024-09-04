use std::collections::HashMap;

use gpi_framework::port::{PortValue, PrimitiveValue};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
//use serde::{Deserialize, Serialize};

//#[derive(FromPyObject, Clone, Debug, serde::Serialize, serde::Deserialize)]
//pub enum Value {
//    Integer(i32),
//    Vec1(Vec<f32>),
//    Vec2(Vec<Vec<f32>>),
//    Vec3(Vec<Vec<Vec<f32>>>),
//    Vec4(Vec<Vec<Vec<Vec<f32>>>>),
//    String(String),
//    #[pyo3(transparent)]
//    Other(String),
//}
//
/// `PortValue`s always have associated data.
/// use `PortType` when only concerened with the type, and not the data
///
//#[allow(dead_code)]
//#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, FromPyObject, PartialEq)]
//pub enum PortValue {
//    Vec1(Vec<PrimitiveValue>),
//    Vec2(Vec<Vec<PrimitiveValue>>),
//    Vec3(Vec<Vec<Vec<PrimitiveValue>>>),
//    Vec4(Vec<Vec<Vec<Vec<PrimitiveValue>>>>),
//    Primitive(PrimitiveValue),
//}
//impl From<Vec<PrimitiveValue>> for PortValue {
//    fn from(i: Vec<PrimitiveValue>) -> Self {
//        Self::Vec1(i)
//    }
//}
//impl From<Vec<Vec<PrimitiveValue>>> for PortValue {
//    fn from(i: Vec<Vec<PrimitiveValue>>) -> Self {
//        Self::Vec2(i)
//    }
//}
//impl From<PrimitiveValue> for PortValue {
//    fn from(i: PrimitiveValue) -> Self {
//        Self::Primitive(i.into())
//    }
//}
//impl From<i32> for PortValue {
//    fn from(i: i32) -> Self {
//        Self::Primitive(i.into())
//    }
//}
//impl From<f32> for PortValue {
//    fn from(i: f32) -> Self {
//        Self::Primitive(i.into())
//    }
//}
//
//#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, FromPyObject)]
//pub enum PrimitiveValue {
//    Integer(i32),
//    Real(f32),
//    Complex((f32, f32)),
//    String(String),
//    Flag(bool),
//    Struct(HashMap<String, PortValue>),
//}
//
//impl From<i32> for PrimitiveValue {
//    fn from(i: i32) -> Self {
//        Self::Integer(i)
//    }
//}
//impl From<f32> for PrimitiveValue {
//    fn from(f: f32) -> Self {
//        Self::Real(f)
//    }
//}
//impl From<String> for PrimitiveValue {
//    fn from(s: String) -> Self {
//        Self::String(s)
//    }
//}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct PyPrimitiveValue(pub PrimitiveValue);

//impl FromPyObject<'_> for PyPrimitiveValue {
//    fn extract<'py>(ob: &'py PyAny) -> PyResult<Self> {
//        Self::extract_bound(&ob.as_borrowed())
//    }
//
//    fn extract_bound<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
//        Self::extract(ob.clone().into_gil_ref())
//    }
//}

impl ToPyObject for PyPrimitiveValue {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        println!("calling to_object on {:?}", self);
        match &self.0 {
            PrimitiveValue::Integer(val) => val.into_py(py),
            PrimitiveValue::Real(val) => val.into_py(py),
            PrimitiveValue::Complex(val) => val.into_py(py),
            PrimitiveValue::String(val) => val.into_py(py),
            PrimitiveValue::Flag(val) => val.into_py(py),
        }
    }
}
impl IntoPy<PyObject> for PyPrimitiveValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        println!("calling into_py on {:?}", self);
        match self.0 {
            PrimitiveValue::Integer(val) => val.into_py(py),
            PrimitiveValue::Real(val) => val.into_py(py),
            PrimitiveValue::Complex(val) => val.into_py(py),
            PrimitiveValue::String(val) => val.into_py(py),
            PrimitiveValue::Flag(val) => val.into_py(py),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PyPortValue(pub PortValue<PyPrimitiveValue>);

impl From<PortValue<PrimitiveValue>> for PyPortValue {
    fn from(p: PortValue<PrimitiveValue>) -> PyPortValue {
        PyPortValue(p.into())
    }
}

impl FromPyObject<'_> for PyPortValue {
    fn extract<'py>(ob: &'py PyAny) -> PyResult<Self> {
        Self::extract_bound(&ob.as_borrowed())
    }

    fn extract_bound<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::extract(ob.clone().into_gil_ref())
    }
}
//type GpiPortData = Option<Box<PortData>>;
impl ToPyObject for PyPortValue {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        println!("calling to_object on {:?}", self);
        match &self.0 {
            // TODO: Can these clones be avoided?
            PortValue::Vec1(val) => val.clone().into_py(py),
            PortValue::Vec2(val) => val.clone().into_py(py),
            PortValue::Vec3(val) => val.clone().into_py(py),
            PortValue::Vec4(val) => val.clone().into_py(py),
            PortValue::Struct(val) => val.clone().into_py(py),
            PortValue::Primitive(val) => val.clone().into_py(py),
        }
    }
}

impl IntoPy<PyObject> for PyPortValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        println!("calling into_py on {:?}", self);
        match self.0 {
            PortValue::Vec1(val) => val.into_py(py),
            PortValue::Vec2(val) => val.into_py(py),
            PortValue::Vec3(val) => val.into_py(py),
            PortValue::Vec4(val) => val.into_py(py),
            PortValue::Struct(val) => val.into_py(py),
            PortValue::Primitive(val) => val.into_py(py),
        }
    }
}

#[pyclass]
pub struct GpiNode {
    #[pyo3(get)]
    pub inputs: Vec<PyPortValue>,

    #[pyo3(get)]
    pub out: PyPortValue,

    #[pyo3(get, set)]
    pub config: HashMap<String, String>,
}

///// A Python module for interfacing with GPI
#[pymodule]
pub fn gpipy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GpiNode>()?;
    Ok(())
}
