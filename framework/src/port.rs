use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PortData {
    Integer(i64),
    Real(f64),
    String(String),
    Vec(Vec<PortData>),
    Obj(HashMap<String, PortData>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PortType {
    Integer,
    Real,
    String,
    Vec,
    Obj,
}

impl From<&PortData> for PortType {
    fn from(value: &PortData) -> Self {
        match value {
            PortData::Integer(_) => PortType::Integer,
            PortData::Real(_) => PortType::Real,
            PortData::String(_) => PortType::String,
            PortData::Vec(_) => PortType::Vec,
            PortData::Obj(_) => PortType::Obj,
        }
    }
}
