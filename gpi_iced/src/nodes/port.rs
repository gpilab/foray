use ndarray::{Array1, Array2};
use numpy::Complex64;
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumString, VariantNames)]
pub enum PortType {
    Integer,
    #[default]
    Real,
    Complex,
    Complex2d,
    Real2d,
}
//PERF: consider ArcArray
#[derive(Clone, Debug)]
pub enum PortData {
    Integer(Array1<i64>),
    Real(Array1<f64>),
    Complex(Array1<Complex64>),
    Complex2d(Array2<Complex64>),
    Real2d(Array2<f64>),
}

impl From<&PortData> for PortType {
    fn from(value: &PortData) -> Self {
        match value {
            PortData::Integer(_) => PortType::Integer,
            PortData::Real(_) => PortType::Real,
            PortData::Complex(_) => PortType::Complex,
            PortData::Complex2d(_) => PortType::Complex2d,
            PortData::Real2d(_) => PortType::Real2d,
        }
    }
}

impl std::fmt::Display for PortData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(data) => {
                write!(
                    f,
                    "dim: {:?} {:.2?}",
                    data.dim(),
                    data.iter().take(10).collect::<Vec<_>>()
                )
            }
            Self::Real(data) => write!(
                f,
                "dim: {:?} {:.2?}",
                data.dim(),
                data.iter().take(10).collect::<Vec<_>>()
            ),
            Self::Complex(data) => write!(
                f,
                "dim: {:?} {:.2?}",
                data.dim(),
                data.iter().take(10).collect::<Vec<_>>()
            ),
            Self::Complex2d(data) => write!(
                f,
                "dim: {:?} {:.2?}",
                data.dim(),
                data.iter().take(10).collect::<Vec<_>>()
            ),
            Self::Real2d(data) => write!(
                f,
                "dim: {:?} {:.2?}",
                data.dim(),
                data.iter().take(10).collect::<Vec<_>>()
            ),
        }
    }
}
