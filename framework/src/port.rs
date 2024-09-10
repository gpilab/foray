//use std::collections::HashMap;
//
//use serde::{Deserialize, Serialize};

//#[allow(dead_code)]
//#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
//pub enum PortValue<T> {
//    Vec1(Vec<T>),
//    Vec2(Vec<Vec<T>>),
//    Vec3(Vec<Vec<Vec<T>>>),
//    Vec4(Vec<Vec<Vec<Vec<T>>>>),
//    Struct(HashMap<String, T>),
//    Primitive(T),
//}
//impl From<Vec<PrimitiveValue>> for PortValue<PrimitiveValue> {
//    fn from(i: Vec<PrimitiveValue>) -> Self {
//        Self::Vec1(i)
//    }
//}
//impl From<Vec<Vec<PrimitiveValue>>> for PortValue<PrimitiveValue> {
//    fn from(i: Vec<Vec<PrimitiveValue>>) -> Self {
//        Self::Vec2(i)
//    }
//}
//impl From<PrimitiveValue> for PortValue<PrimitiveValue> {
//    fn from(i: PrimitiveValue) -> Self {
//        Self::Primitive(i.into())
//    }
//}
//impl From<i32> for PortValue<PrimitiveValue> {
//    fn from(i: i32) -> Self {
//        Self::Primitive(i.into())
//    }
//}
//impl From<f32> for PortValue<PrimitiveValue> {
//    fn from(i: f32) -> Self {
//        Self::Primitive(i.into())
//    }
//}
//
//#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
//pub enum PrimitiveValue {
//    Integer(i32),
//    Real(f32),
//    Complex((f32, f32)),
//    String(String),
//    Flag(bool),
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
//
///// Example Usage
//#[test]
//fn t1() {
//    let p: PortValue<PrimitiveValue> = PortValue::Vec1 {
//        0: (vec![1, 2, 3]).iter().map(|e| (*e).into()).collect(),
//    };
//    if let PortValue::Vec1(v) = &p {
//        println!("{v:?}")
//    }
//    let nested: PortValue<PrimitiveValue> = PortValue::Vec1(
//        vec![1.into(), 2.into(), 3.into()]
//            .iter()
//            .map(|e: &i32| (*e).into())
//            .collect(),
//    );
//    let mut hash: HashMap<String, PrimitiveValue> = HashMap::new();
//    hash.insert("r".into(), 0.5.into());
//    hash.insert("g".into(), 0.5.into());
//    hash.insert("b".into(), 0.5.into());
//    let my_struct = PortValue::Struct(hash);
//
//    println!("{p:?}\n{nested:?}\n{my_struct:?}");
//}
//
///// Just the port type, no data
//#[allow(dead_code)]
//#[derive(Debug, serde::Serialize, serde::Deserialize)]
//pub enum PortType {
//    Vec1,
//    Vec2,
//    Vec3,
//    Vec4,
//    Integer,
//    Real,
//    Complex,
//    String,
//    Flag,
//    Struct,
//}
//
//impl PortValue<PrimitiveValue> {
//    pub fn get_type(&self) -> PortType {
//        match self {
//            PortValue::Vec1(_) => PortType::Vec1,
//            PortValue::Vec2(_) => PortType::Vec2,
//            PortValue::Vec3(_) => PortType::Vec3,
//            PortValue::Vec4(_) => PortType::Vec4,
//            PortValue::Struct(_) => PortType::Struct,
//            PortValue::Primitive(primitive) => match primitive {
//                PrimitiveValue::Integer(_) => PortType::Integer,
//                PrimitiveValue::Real(_) => PortType::Real,
//                PrimitiveValue::Complex(_) => PortType::Complex,
//                PrimitiveValue::String(_) => PortType::String,
//                PrimitiveValue::Flag(_) => PortType::Flag,
//            },
//        }
//    }
//}
