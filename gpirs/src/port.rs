use std::{collections::HashMap, f64, fmt::Debug};

use ordered_float::OrderedFloat;
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyclass_enum};
use serde::{Deserialize, Serialize};

/// `PortValue`s always have associated data.
/// use `PortType` when only concerened with the type, and not the data
/// PERF: Array performance could be improved by switching to direct Vec<i64>,Vec<f64>, etc.
/// This adds some complexity, so in the vein of avoiding premature optimization, let's tackle
/// this when/if it becomes a bottleneck
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, FromPyObject)]
pub enum PortValue {
    Integer(i64),
    Real(f64),
    Complex((f64, f64)),
    String(String),
    Flag(bool),
    Image(Vec<Vec<u8>>),
    Array(Vec<PortValue>),
    Struct(HashMap<String, PortValue>),
}

/// Holds no data, just serves as an indication of a type of data
#[gen_stub_pyclass_enum]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum PortType {
    Integer,
    Real,
    Complex,
    String,
    Flag,
    Image,
    Array(Box<PortType>),
    Struct(HashMap<String, PortType>),
}

#[derive(Debug, Clone, FromPyObject)]
pub enum SimplePortType {
    Primitive(String),
    Array(Vec<SimplePortType>),
    Struct(HashMap<String, SimplePortType>),
}

impl FromPyObject<'_> for PortType {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        println!("Unextracted: {ob}");
        let simple_port_type = ob.extract::<SimplePortType>()?;
        println!("extracted: {simple_port_type:?}");
        match simple_port_type.try_into() {
            Ok(pt) => Ok(pt),
            Err(_) => PyResult::Err(PyErr::from_value_bound(ob.clone())),
        }
    }
}
////TODO: make tests, and correctly parse nested structurs
impl TryFrom<SimplePortType> for PortType {
    type Error = String;
    fn try_from(value: SimplePortType) -> Result<Self, Self::Error> {
        Ok(match value {
            SimplePortType::Primitive(port_type) => match port_type.as_str() {
                "Integer" => PortType::Integer,
                "Real" => PortType::Real,
                "String" => PortType::String,
                "Flag" => PortType::Flag,
                "Image" => PortType::Image,
                e => return Err(e.to_string()),
            },
            SimplePortType::Array(simple_port_vec) => {
                PortType::Array(Box::new(PortType::try_from(simple_port_vec[0].clone())?))
            }
            SimplePortType::Struct(structure) => {
                let hash_port: HashMap<String, PortType> = structure
                    .into_iter()
                    .map(|(label, value)| (label, PortType::try_from(value).unwrap()))
                    .collect();
                PortType::Struct(hash_port)
            }
        })
    }
}

impl FromPyObject<'_> for Box<PortType> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Box::new(ob.extract::<PortType>()?))
    }
}
impl FromPyObject<'_> for Box<SimplePortType> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Box::new(ob.extract::<SimplePortType>()?))
    }
}

impl Into<PortValue> for Vec<i64> {
    fn into(self) -> PortValue {
        PortValue::Array(self.iter().map(|f| PortValue::Integer(*f)).collect())
    }
}
impl Into<PortValue> for Vec<f64> {
    fn into(self) -> PortValue {
        PortValue::Array(self.iter().map(|f| PortValue::Real(*f)).collect())
    }
}

impl IntoPy<PyObject> for PortValue {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            PortValue::Integer(v) => v.into_py(py),
            PortValue::Real(v) => v.into_py(py),
            PortValue::Complex(v) => v.into_py(py),
            PortValue::String(v) => v.into_py(py),
            PortValue::Flag(v) => v.into_py(py),
            PortValue::Image(v) => v.into_py(py),
            PortValue::Array(v) => v.into_py(py),
            PortValue::Struct(v) => v.into_py(py),
        }
    }
}

impl PortValue {
    pub fn dim(&self) -> Vec<usize> {
        self.dim_help(Vec::new())
    }

    fn dim_help(&self, mut current_dimensions: Vec<usize>) -> Vec<usize> {
        println!("dim:{:?}", current_dimensions);
        match self {
            PortValue::Array(array) => {
                current_dimensions.push(array.len());
                array[0].dim_help(current_dimensions)
            }
            _ => current_dimensions,
        }
    }
}

//pub enum NodeInterface {
//    Input(HashMap<String, PortValue>),
//    Output(HashMap<String, PortValue>),
//}

//// This could be used as the type for PortValue which could reduce some conversion overhead
//enum PortValueArray {
//    IntegerArray(Vec<i64>),
//    RealArray(Vec<f64>),
//    ComplexArray(Vec<(f64, f64)>),
//    StringArray(Vec<String>),
//    FlagArray(Vec<bool>),
//}

//#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
//pub enum GpiItemLabel {
//    Struct,
//    Integer,
//    Real,
//    Complex,
//    String,
//    Flag,
//}

//impl FromPyObject<'_> for GpiItemLabel {
//    fn extract_bound<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
//        match ob.extract::<&str>()? {
//            "Struct" => Ok(GpiItemLabel::Struct),
//            "Integer" => Ok(GpiItemLabel::Integer),
//            "Real" => Ok(GpiItemLabel::Integer),
//            "Complex" => Ok(GpiItemLabel::Integer),
//            "String" => Ok(GpiItemLabel::Integer),
//            "Flag" => Ok(GpiItemLabel::Integer),
//            _ => Err(PyErr::new::<PyTypeError, _>(
//                "Could not Read in vector type from python!",
//            )),
//        }
//    }
//}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromPyObject)]
pub enum MyVec<T> {
    Vec1(Vec<T>),
    Vec2(Vec<Vec<T>>),
    Vec3(Vec<Vec<Vec<T>>>),
    Vec4(Vec<Vec<Vec<Vec<T>>>>),
}

impl IntoPy<PyObject> for MyVec<i32> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            MyVec::Vec1(val) => val.into_py(py),
            MyVec::Vec2(val) => val.into_py(py),
            MyVec::Vec3(val) => val.into_py(py),
            MyVec::Vec4(val) => val.into_py(py),
        }
    }
}
impl IntoPy<PyObject> for MyVec<f32> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            MyVec::Vec1(val) => val.into_py(py),
            MyVec::Vec2(val) => val.into_py(py),
            MyVec::Vec3(val) => val.into_py(py),
            MyVec::Vec4(val) => val.into_py(py),
        }
    }
}
impl IntoPy<PyObject> for MyVec<String> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            MyVec::Vec1(val) => val.into_py(py),
            MyVec::Vec2(val) => val.into_py(py),
            MyVec::Vec3(val) => val.into_py(py),
            MyVec::Vec4(val) => val.into_py(py),
        }
    }
}
impl IntoPy<PyObject> for MyVec<(f32, f32)> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            MyVec::Vec1(val) => val.into_py(py),
            MyVec::Vec2(val) => val.into_py(py),
            MyVec::Vec3(val) => val.into_py(py),
            MyVec::Vec4(val) => val.into_py(py),
        }
    }
}
impl IntoPy<PyObject> for MyVec<bool> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            MyVec::Vec1(val) => val.into_py(py),
            MyVec::Vec2(val) => val.into_py(py),
            MyVec::Vec3(val) => val.into_py(py),
            MyVec::Vec4(val) => val.into_py(py),
        }
    }
}

//impl IntoPy<PyObject> for MyVec<HashMap<String, GpiItem>> {
//    fn into_py(self, py: Python<'_>) -> PyObject {
//        match self {
//            MyVec::Vec1(val) => val.into_py(py),
//            MyVec::Vec2(val) => val.into_py(py),
//            MyVec::Vec3(val) => val.into_py(py),
//            MyVec::Vec4(val) => val.into_py(py),
//        }
//    }
//}

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

//#[test]
//fn enum_works() {
//    pyo3::prepare_freethreaded_python();
//    Python::with_gil(|py| {
//        let x = Py::new(py, PortType::Integer).unwrap();
//        //let y = Py::new(py, PortType::Real {}).unwrap();
//        let cls = py.get_type_bound::<PortType>();
//        pyo3::py_run!(py, x  cls, r#"
//        y = cls.Integer
//        print(x)
//        print(y)
//        assert x == cls.Integer
//    "#)
//    })
//    //fn nested_enum() {
//    //        let z = Py::new(py, PortType::Array {}).unwrap();
//    //    }
//}

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
//impl From<i32> for PortValue {
//    fn from(i: i32) -> Self {
//        println!("from {:?}", i);
//        Self::Integer(i.into())
//    }
//}
//impl From<f64> for PortValue {
//    fn from(i: f64) -> Self {
//        println!("from {:?}", i);
//        Self::Real(i.into())
//    }
//}
//impl From<Vec<f32>> for PortValue {
//    fn from(v: Vec<f32>) -> Self {
//        println!("from {:?}", v);
//        Self::Vec1(GpiItemLabel::Real, v.into_iter().map(|x| x).collect())
//    }
//}
//impl From<Vec<f64>> for PortValue {
//    fn from(v: Vec<f64>) -> Self {
//        println!("from {:?}", v);
//        Self::Vec1(v.into_iter().map(|x| GpiItem::Real(x.into())).collect())
//    }
//}

//impl ToPyObject for PortValue {
//    fn to_object(&self, py: Python<'_>) -> PyObject {
//        println!("calling ToPyObject on {:?}", self);
//        match &self {
//            // TODO: Can these clones be avoided?
//            //PortValue::AVec(val) => val.clone().into_py(py),
//            PortValue::IntegerVec(val) => val.clone().into_py(py),
//            PortValue::RealVec(val) => val.clone().into_py(py),
//            PortValue::ComplexVec(val) => val.clone().into_py(py),
//            PortValue::StringVec(val) => val.clone().into_py(py),
//            PortValue::StructVec(val) => val.clone().into_py(py),
//            PortValue::FlagVec(val) => val.clone().into_py(py),
//            PortValue::Primitive(val) => val.clone().into_py(py),
//            //PortValue::Vec1(val) => val.clone().into_py(py),
//            //PortValue::Vec2(val) => val.clone().into_py(py),
//            ////PortValue::Vec2(val) => val.clone().into_py(py),
//            //PortValue::Vec3(val) => val.clone().into_py(py),
//            //PortValue::Vec4(val) => val.clone().into_py(py),
//            //PortValue::Struct(val) => val.clone().into_py(py),
//            //PortValue::Integer(val) => val.into_py(py),
//            //PortValue::Real(val) => val.into_py(py),
//            //PortValue::Complex(val) => val.into_py(py),
//            //PortValue::String(val) => val.into_py(py),
//            //PortValue::Flag(val) => val.into_py(py),
//            //PortValue::Primitive(val) => val.clone().into_py(py),
//        }
//    }
//}

//impl IntoPy<PyObject> for GpiItem {
//    fn into_py(self, py: Python<'_>) -> PyObject {
//        println!("calling IntoPy on {:?}", self);
//        match self {
//            GpiItem::Struct(val) => val.into_py(py),
//            GpiItem::Integer(val) => val.into_py(py),
//            GpiItem::Real(val) => val.into_py(py),
//            GpiItem::Complex(val) => val.into_py(py),
//            GpiItem::String(val) => val.into_py(py),
//            GpiItem::Flag(val) => val.into_py(py),
//        }
//    }
//}
//
//impl IntoPy<PyObject> for PortValue {
//    fn into_py(self, py: Python<'_>) -> PyObject {
//        println!("calling IntoPy on {:?}", self);
//        match self {
//            PortValue::IntegerVec(val) => val.into_py(py),
//            PortValue::RealVec(val) => val.clone().into_py(py),
//            PortValue::ComplexVec(val) => val.clone().into_py(py),
//            PortValue::StringVec(val) => val.clone().into_py(py),
//            PortValue::StructVec(val) => val.clone().into_py(py),
//            PortValue::FlagVec(val) => val.clone().into_py(py),
//            PortValue::Primitive(val) => val.clone().into_py(py),
//            ////PortValue::AVec(val) => val.into_py(py),
//            //PortValue::Vec1(val) => val.into_py(py),
//            //PortValue::Vec2(val) => val.into_py(py),
//            ////PortValue::Vec2(val) => val.into_py(py),
//            //PortValue::Vec3(val) => val.into_py(py),
//            //PortValue::Vec4(val) => val.into_py(py),
//            //PortValue::Struct(val) => val.into_py(py),
//            //PortValue::Integer(val) => val.into_py(py),
//            //PortValue::Real(val) => val.into_py(py),
//            //PortValue::Complex(val) => val.into_py(py),
//            //PortValue::String(val) => val.into_py(py),
//            //PortValue::Flag(val) => val.into_py(py),
//        }
//    }
//}
define_stub_info_gatherer!(stub_info);
