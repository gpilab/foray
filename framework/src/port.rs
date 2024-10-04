use std::collections::HashMap;

use serde::Serialize;

/// Temporarily Serializable, probably don't want to
/// serialize data in the long run
#[derive(Debug, Clone, Serialize)]
pub enum PortData {
    Integer(i64),
    Real(f64),
    String(String),
    Vec(Vec<PortData>),
    Struct(HashMap<String, PortData>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum PortType {
    Integer,
    Real,
    String,
    Vec,
    Struct,
}

impl From<&PortData> for PortType {
    fn from(value: &PortData) -> Self {
        match value {
            PortData::Integer(_) => PortType::Integer,
            PortData::Real(_) => PortType::Real,
            PortData::String(_) => PortType::String,
            PortData::Vec(_) => PortType::Vec,
            PortData::Struct(_) => PortType::Struct,
        }
    }
}
