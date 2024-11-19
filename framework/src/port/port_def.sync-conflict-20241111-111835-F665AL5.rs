use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

use derive_more::derive::{From, Unwrap};
use serde::Serialize;

use super::{Bool, Complex, Integer, Real, Str};
use super::{Primitive, PrimitiveType};

/// Temporarily Serializable
///  TODO: probably don't want to serialize data in the long run
#[derive(From, PartialEq, Serialize)]
pub enum Port {
    Primitive(Primitive),
    Array(ArrayValue),
    Struct(HashMap<String, Port>),
}

impl Debug for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Port::Primitive(p) => write!(f, "{p}"),
            Port::Array(p) => write!(f, "{p}"),
            Port::Struct(p) => write!(f, "{:?}", p),
        }
    }
}

#[derive(From, Debug, PartialEq, Eq, Clone, Serialize)]
pub enum PortType {
    Primitive(PrimitiveType),
    Array(ShapeType, Box<PortType>),
    Struct(HashMap<String, PortType>),
}

/// The shape of an array. a 3x4, 2d array would have the shape
/// `vec![3,4]`
/// The length indicates the number of dimensions, the values indicate the lengths along each dimension
pub type Shape = Vec<usize>;
/// The shapes that a port will accept
/// None indicates that there is no required length
pub(crate) type ShapeType = Vec<Option<usize>>;

//pub(crate) type ArrayType = (Box<PortType>, ShapeType);

#[derive(From, PartialEq, Debug, Serialize, Unwrap)]
pub enum ArrayValue {
    Integer(Shape, Vec<Integer>),
    Real(Shape, Vec<Real>),
    Complex(Shape, Vec<Complex>),
    Str(Shape, Vec<Str>),
    Bool(Shape, Vec<Bool>),
    Array(Shape, Vec<ArrayValue>),
    Struct(Shape, Vec<HashMap<String, Port>>),
}
impl Display for ArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayValue::Integer(s, v) => {
                write!(f, "Shape:{s:?}, Values:[")?;
                for v in v {
                    write!(f, "{},", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            ArrayValue::Real(s, v) => {
                write!(f, "Shape:{s:?}, Values:[")?;
                for v in v {
                    write!(f, "{},", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            _ => todo!(),
        }
    }
}

//impl ArrayValue {
//    fn iter(&self) -> Iter {
//
//        match self{
//            ArrayValue::Integer(_, v) => v.iter(),
//            ArrayValue::Real(Shape, Vec<Real>),
//            ArrayValue::Complex(Shape, Vec<Complex>),
//            ArrayValue::Str(Shape, Vec<Str>),
//            ArrayValue::Bool(Shape, Vec<Bool>),
//            ArrayValue::Array(Shape, Vec<ArrayValue>),
//            ArrayValue::Struct(Shape, Vec<HashMap<String, Port>>),
//        }
//    }
//}

#[cfg(test)]
mod test {

    use super::{Integer, Real};

    use super::*;
    #[test]
    /// Just test that types can be created as expected and it still compiles
    fn port_type() {
        let integer_port = PortType::Primitive(PrimitiveType::Integer);
        let _pt2 = PortType::Primitive(PrimitiveType::Real);
        let _pt3 = PortType::Primitive(PrimitiveType::Str);
        let pt4 = PortType::Array(vec![Some(3), Some(4)], Box::new(integer_port.clone()));
        let _pt5 = PortType::Array(vec![Some(20)], Box::new(pt4));

        let _pt6 = PortType::Struct(HashMap::from([
            ("r".into(), integer_port.clone()),
            ("g".into(), integer_port.clone()),
            ("b".into(), integer_port.clone()),
            ("a".into(), integer_port.clone()),
        ]));
    }
    #[test]
    /// Construct port values
    fn primitive_eq() {
        // This looks excessively verbose, but I think it's necessary/worth it
        //  TODO: create abstraction for port creation
        let pv1 = Port::Primitive(Primitive::Integer(Integer(1)));
        let pv2 = Port::Primitive(Primitive::Integer(Integer(3)));
        let pv3 = Port::Primitive(Primitive::Integer(Integer(1)));
        assert_ne!(pv1, pv2);
        assert_eq!(pv1, pv3);
    }
    #[test]
    fn struct_eq() {
        // This looks excessively verbose, but I think it's necessary/worth it
        //  TODO: create abstraction for port creation
        let struct1 = Port::Struct(HashMap::from([
            ("r".into(), Port::Primitive(Primitive::Integer(Integer(1)))),
            ("g".into(), Port::Primitive(Primitive::Integer(1.into()))),
            ("b".into(), Port::Primitive(Integer(1).into())),
            ("a".into(), Port::Primitive(Integer(1).into())),
        ]));
        let struct2 = Port::Struct(HashMap::from([
            ("r".into(), Port::Primitive(1.into())),
            ("g".into(), Port::Primitive(2.into())),
            ("b".into(), Port::Primitive(3.into())),
            ("a".into(), Port::Primitive(4.into())),
        ]));
        let struct3 = Port::Struct(HashMap::from([
            ("r".into(), Port::Primitive(1.into())),
            ("g".into(), Port::Primitive(1.into())),
            ("b".into(), Port::Primitive(1.into())),
            ("a".into(), Port::Primitive(1.into())),
        ]));
        assert_ne!(struct1, struct2);
        assert_eq!(struct1, struct3);
    }
    #[test]
    fn array_eq() {
        let flat_array = vec![Real(1.), Real(2.), Real(3.), Real(4.)];
        let a1 = Port::Array(ArrayValue::Real(vec![2, 2], flat_array.clone()));
        let a2 = Port::Array(ArrayValue::Real(vec![4], flat_array.clone()));
        let a3 = Port::Array(ArrayValue::Real(vec![2, 2], flat_array));
        assert_ne!(a1, a2);
        assert_eq!(a1, a3);
    }
}
