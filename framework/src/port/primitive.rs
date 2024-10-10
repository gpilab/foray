use derive_more::derive::{Add, Display, From, Sum};
use serde::Serialize;

#[derive(From, PartialEq, Debug, Serialize, Clone, Copy, Display)]
#[from(i8, i16, i32, i64, u8, u16, u32)]
pub struct Integer(pub i64);

#[derive(From, PartialEq, Debug, Serialize, Clone, Add, Copy, Sum, Display)]
#[from(f32, f64)]
pub struct Real(pub f64);

#[derive(From, PartialEq, Debug, Serialize, Clone, Display)]
#[display("({},{}i)", 0, 1)]
pub struct Complex(pub f64, pub f64);

#[derive(From, PartialEq, Debug, Serialize, Clone, Display)]
pub struct Str(pub String);

#[derive(From, PartialEq, Debug, Serialize, Clone, Display)]
pub struct Bool(pub bool);

#[derive(From, PartialEq, Debug, Serialize, Clone, Display)]
pub enum Primitive {
    #[from(Integer, i64, i32)]
    Integer(Integer),
    #[from(Real, f32, f64)]
    Real(Real),
    Complex(Complex),
    Str(Str),
    Bool(Bool),
}

#[derive(From, Debug, PartialEq, Eq, Clone, Serialize, Display)]
pub enum PrimitiveType {
    Integer,
    Real,
    Complex,
    Str,
    Bool,
}

impl From<&Primitive> for PrimitiveType {
    fn from(value: &Primitive) -> Self {
        match value {
            Primitive::Integer(_) => PrimitiveType::Integer,
            Primitive::Real(_) => PrimitiveType::Real,
            Primitive::Complex(_) => PrimitiveType::Complex,
            Primitive::Str(_) => PrimitiveType::Str,
            Primitive::Bool(_) => PrimitiveType::Bool,
        }
    }
}
