use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumString)]
pub enum PortType {
    Integer,
    #[default]
    Real,
    Complex,
    Real2d,
}
#[derive(Clone, Debug)]
pub enum PortData {
    Integer(Array1<i64>),
    Real(Array1<f64>),
    Complex(Array1<(f64, f64)>),
    Real2d(Array2<f64>),
}

impl From<&PortData> for PortType {
    fn from(value: &PortData) -> Self {
        match value {
            PortData::Integer(_) => PortType::Integer,
            PortData::Real(_) => PortType::Real,
            PortData::Complex(_) => PortType::Complex,
            PortData::Real2d(_) => PortType::Real2d,
        }
    }
}

impl std::fmt::Display for PortData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(data) => write!(f, "{:?}", data.dim()),
            Self::Real(data) => write!(f, "{:.2?}", data.dim()),
            Self::Complex(data) => write!(f, "{:.2?}", data.dim()),
            Self::Real2d(data) => write!(f, "{:.2?}", data.dim()),
        }
    }
}
