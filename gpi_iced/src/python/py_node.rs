use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use derive_more::derive::{Debug, Display};
use numpy::{PyArrayMethods, ToPyArray};
use pyo3::{types::PyAnyMethods, FromPyObject, PyObject, Python};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use crate::OrderMap;
use crate::{
    gui_node::PortDataReference,
    nodes::{
        port::{PortData, PortType},
        status::NodeError,
    },
};

use super::{gpipy_compute, gpipy_config};

#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq)]
#[display("{}", self.name)]
pub struct PyNode {
    pub name: String,
    pub path: PathBuf,
    pub ports: Result<PortDef, NodeError>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PortDef {
    pub inputs: OrderMap<String, PortType>,
    pub outputs: OrderMap<String, PortType>,
}
impl PortData {
    pub fn to_py(&self, py: Python) -> PyObject {
        match self {
            PortData::Integer(array_base) => array_base.to_pyarray(py).into_any().into(),
            PortData::Real(array_base) => array_base.to_pyarray(py).into_any().into(),
            PortData::Real2d(array_base) => array_base.to_pyarray(py).into_any().into(),
            PortData::Complex(array_base) => array_base.to_pyarray(py).into_any().into(),
            PortData::Complex2d(array_base) => array_base.to_pyarray(py).into_any().into(),
            PortData::Real3d(array_base) => array_base.to_pyarray(py).into_any().into(),
        }
    }
}

impl PyNode {
    pub fn new(name: &str) -> Self {
        gpipy_config(name)
    }

    pub fn compute(
        &self,
        inputs: OrderMap<String, PortDataReference>,
    ) -> Result<OrderMap<String, PortData>, NodeError> {
        match &self.ports {
            Ok(ports) => {
                // Convert inputs to python arrays/objects
                Python::with_gil(|py| {
                    let py_inputs = inputs
                        .into_iter()
                        .map(|(k, v)| (k.clone(), v.to_py(py)))
                        .collect();
                    let out = gpipy_compute(
                        self.path.file_stem().unwrap().to_str().unwrap(),
                        &py_inputs,
                        py,
                    )
                    .map_err(|err| NodeError::Runtime(err.to_string()))?;

                    ports
                        .outputs
                        .iter()
                        .map(|(k, port_type)| {
                            Self::extract_py_data(port_type, &out[k], py).map(|p| (k.clone(), p))
                        })
                        .collect()
                })
            }
            // If the ports are not valid, don't bother running. Just surface the error
            Err(e) => Err(e.clone()),
        }
    }

    pub fn py_node_path(node_name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("nodes/{node_name}.py"))
    }
    pub fn extract_py_data(
        port_type: &PortType,
        py_object: &PyObject,
        py: Python,
    ) -> Result<PortData, NodeError> {
        // unsure how to make the repetion bellow more generic, while still
        // automatically converting to the PortType
        unsafe {
            Ok(match port_type {
                PortType::Integer => PortData::Integer(
                    py_object
                        .bind(py)
                        .downcast()
                        .map_err(|_e| output_error(port_type, py_object))?
                        .as_array()
                        .to_owned(),
                ),
                PortType::Real => PortData::Real(
                    py_object
                        .bind(py)
                        .downcast()
                        .map_err(|_e| output_error(port_type, py_object))?
                        .as_array()
                        .to_owned(),
                ),
                PortType::Complex => PortData::Complex(
                    py_object
                        .bind(py)
                        .downcast()
                        .map_err(|_e| output_error(port_type, py_object))?
                        .as_array()
                        .to_owned(),
                ),
                PortType::Real2d => PortData::Real2d(
                    py_object
                        .bind(py)
                        .downcast()
                        .map_err(|_e| output_error(port_type, py_object))?
                        .as_array()
                        .to_owned(),
                ),
                PortType::Real3d => PortData::Real3d(
                    py_object
                        .bind(py)
                        .downcast()
                        .map_err(|_e| output_error(port_type, py_object))?
                        .as_array()
                        .to_owned(),
                ),
                PortType::Complex2d => PortData::Complex2d(
                    py_object
                        .bind(py)
                        .downcast()
                        .map_err(|_e| output_error(port_type, py_object))?
                        .as_array()
                        .to_owned(),
                ),
            })
        }
    }
}

/// Port to receive port def from python
#[derive(Clone, FromPyObject, Default, Debug, Serialize, Deserialize)]
pub struct PyFacingPortDef {
    inputs: HashMap<String, String>,
    outputs: HashMap<String, String>,
}

impl TryFrom<PyFacingPortDef> for PortDef {
    type Error = NodeError;
    fn try_from(value: PyFacingPortDef) -> Result<Self, Self::Error> {
        Ok(PortDef {
            inputs: value
                .clone()
                .inputs
                .into_iter()
                .map(|(key, value)| PortType::from_str(&value).map(|v| (key, v)))
                .collect::<Result<OrderMap<_, _>, _>>()
                .map_err(|e| {
                    NodeError::Output(format!(
                        "{:#?}\nExpected one of {:#?}, found {:#?}",
                        e,
                        PortType::VARIANTS,
                        value.inputs,
                    ))
                })?,
            outputs: value
                .clone()
                .outputs
                .into_iter()
                .map(|(key, value)| PortType::from_str(&value).map(|v| (key, v)))
                .collect::<Result<OrderMap<_, _>, _>>()
                .unwrap_or_else(|_| {
                    //TODO: pass the error up? not sure why this currently just panics
                    panic!(
                        "Expected one of {:#?}, found {:#?}",
                        PortType::VARIANTS,
                        value.outputs
                    )
                }),
        })
    }
}

fn output_error(port_type: &PortType, py_object: &PyObject) -> NodeError {
    NodeError::Output(format!(
        "Received unexpected output from node. Expected one of {port_type:#?}, found {py_object:#?}"
    ))
}
