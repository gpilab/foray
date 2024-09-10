use std::{collections::HashMap, f64};

use ordered_float::OrderedFloat;
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyclass_enum};
use serde::{Deserialize, Serialize};

/// `PortValue`s always have associated data.
/// use `PortType` when only concerened with the type, and not the data
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromPyObject)]
pub enum PortValue {
    Vec(Vec<RealFloat>),
    Vec2(Vec<Vec<RealFloat>>),
    Struct(HashMap<String, i32>),
    Integer(i32),
    Real(RealFloat),
    Complex((RealFloat, RealFloat)),
    String(String),
    Flag(bool),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct RealFloat(OrderedFloat<f64>);

impl From<f32> for RealFloat {
    fn from(f: f32) -> Self {
        println!("from {:?}", f);
        RealFloat(OrderedFloat(f.into()))
    }
}
impl From<f64> for RealFloat {
    fn from(f: f64) -> Self {
        println!("from {:?}", f);
        RealFloat(OrderedFloat(f))
    }
}
impl IntoPy<PyObject> for RealFloat {
    fn into_py(self, py: Python<'_>) -> PyObject {
        println!("intopy {:?}", self);
        (self.0 .0).into_py(py)
    }
}

impl FromPyObject<'_> for RealFloat {
    fn extract_bound<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(RealFloat(OrderedFloat(ob.extract::<f64>()?)))
    }
}

/// Holds no data, just serves as an indication of a type of data
#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PortType {
    Vec,
    Struct,
    Integer,
    Real,
    Complex,
    String,
    Flag,
}

#[test]
fn enum_works() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let x = Py::new(py, PortType::Flag).unwrap();
        let y = Py::new(py, PortType::Real).unwrap();
        let z = Py::new(py, PortType::Vec).unwrap();
        let cls = py.get_type_bound::<PortType>();
        pyo3::py_run!(py, x y  z cls, r#"
        assert x == cls.Flag
        assert y == cls.Real
        assert z != cls.Struct
        assert x != y
    "#)
    })
}

//impl FromPyObject<'_> for PortType {
//    fn extract<'py>(ob: &'py PyAny) -> PyResult<Self> {
//        //Self::extract_bound(&ob.as_borrowed())
//        //
//    }
//
//    fn extract_bound<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
//        Self::extract(ob.clone().into_gil_ref())
//    }
//}

//impl PortValue {
//    pub fn dim(self) -> (usize, usize, usize, usize) {
//        match self {
//            Self::Vec(v1) => match &v1[0] {
//                Self::Vec(v2) => match &v2[0] {
//                    Self::Vec(v3) => match &v3[0] {
//                        Self::Vec(v4) => (v1.len(), v2.len(), v3.len(), v4.len()),
//                        _ => (v1.len(), v2.len(), v3.len(), 0),
//                    },
//                    _ => (v1.len(), v2.len(), 0, 0),
//                },
//                _ => (v1.len(), 0, 0, 0),
//            },
//            _ => (0, 0, 0, 0),
//        }
//    }
//}
impl From<i32> for PortValue {
    fn from(i: i32) -> Self {
        println!("from {:?}", i);
        Self::Integer(i.into())
    }
}
impl From<f64> for PortValue {
    fn from(i: f64) -> Self {
        println!("from {:?}", i);
        Self::Real(i.into())
    }
}
impl From<Vec<f32>> for PortValue {
    fn from(v: Vec<f32>) -> Self {
        println!("from {:?}", v);
        Self::Vec(v.into_iter().map(|x| x.into()).collect())
    }
}
impl From<Vec<f64>> for PortValue {
    fn from(v: Vec<f64>) -> Self {
        println!("from {:?}", v);
        Self::Vec(v.into_iter().map(|x| x.into()).collect())
    }
}

impl ToPyObject for PortValue {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        println!("calling to_object on {:?}", self);
        match &self {
            // TODO: Can these clones be avoided?
            PortValue::Vec(val) => val.clone().into_py(py),
            PortValue::Vec2(val) => val.clone().into_py(py),
            //PortValue::Vec2(val) => val.clone().into_py(py),
            //PortValue::Vec3(val) => val.clone().into_py(py),
            //PortValue::Vec4(val) => val.clone().into_py(py),
            PortValue::Struct(val) => val.clone().into_py(py),
            PortValue::Integer(val) => val.into_py(py),
            PortValue::Real(val) => val.into_py(py),
            PortValue::Complex(val) => val.into_py(py),
            PortValue::String(val) => val.into_py(py),
            PortValue::Flag(val) => val.into_py(py),
            //PortValue::Primitive(val) => val.clone().into_py(py),
        }
    }
}

impl IntoPy<PyObject> for PortValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        println!("calling into_py on {:?}", self);
        match self {
            PortValue::Vec(val) => val.into_py(py),
            PortValue::Vec2(val) => val.into_py(py),
            //PortValue::Vec2(val) => val.into_py(py),
            //PortValue::Vec3(val) => val.into_py(py),
            //PortValue::Vec4(val) => val.into_py(py),
            PortValue::Struct(val) => val.into_py(py),
            PortValue::Integer(val) => val.into_py(py),
            PortValue::Real(val) => val.into_py(py),
            PortValue::Complex(val) => val.into_py(py),
            PortValue::String(val) => val.into_py(py),
            PortValue::Flag(val) => val.into_py(py),
        }
    }
}
define_stub_info_gatherer!(stub_info);
