use std::collections::HashMap;

use ndarray::{ArrayD, IxDyn};
use numpy::Complex64;
use pyo3::FromPyObject;

#[derive(Debug, Clone)]
enum DataVariant {
    Integer,
    Real,
    Boolean,
    String,
    Complex,

    IntegerArray,
    RealArray,
    StringArray,
    ComplexArray,

    Array(Box<DataVariant>),
    Object(HashMap<String, DataVariant>),
}

#[derive(Debug, Clone)]
enum Data {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    String(String),
    //Complex(Complex64),
    IntegerArray(ArrayD<i64>),
    RealArray(ArrayD<f64>),
    StringArray(ArrayD<String>),
    ComplexArray(ArrayD<Complex64>),

    Array(ArrayD<Data>),
    Object(HashMap<String, Data>),
}

//impl FromPyObject for Data {
//    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {}
//}

fn main() {
    let a = Data::Integer(5);
    let b = Data::RealArray(ArrayD::from_shape_vec(IxDyn(&[2]), vec![4.0, 1.0]).unwrap());
    let c = Data::Object(
        [
            ("a".into(), Data::Integer(5)),
            ("b".into(), Data::Real(5.0)),
            (
                "c".into(),
                Data::RealArray(ArrayD::from_shape_vec(IxDyn(&[2]), vec![4.0, 1.0]).unwrap()),
            ),
            (
                "d".into(),
                Data::Object([("a".into(), Data::String("a_value".into()))].into()),
            ),
        ]
        .into(),
    );
    dbg!(a, b, c);
}

