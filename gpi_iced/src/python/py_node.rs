use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use derive_more::derive::Debug;
use numpy::{PyArrayMethods, ToPyArray};
use pyo3::{types::PyAnyMethods, FromPyObject, PyObject, Python};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use crate::{
    nodes::{PortData, PortType},
    OrderMap,
};

use super::{gpipy_compute, gpipy_config};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PyNode {
    pub path: PathBuf,
    pub ports: PortDef,
}

impl Default for PyNode {
    fn default() -> Self {
        Self::new("null").unwrap()
    }
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
        }
    }
}

impl PyNode {
    pub fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        gpipy_config(name)
        //Self {
        //    path: Self::py_node_path(name),
        //    //TODO: initialize using gpipy_config
        //    ports: gpipy_config(node_name)
        //}
    }

    pub fn compute(
        &self,
        //populated_inputs: OrderMap<String, PortData>,
        inputs: OrderMap<String, &std::cell::RefCell<PortData>>,
    ) -> OrderMap<String, PortData> {
        // convert inputs to python arrays/objects

        Python::with_gil(|py| {
            let py_inputs = inputs
                .into_iter()
                .map(|(k, v)| (k.clone(), v.borrow().to_py(py)))
                .collect();
            let out = gpipy_compute(
                self.path.file_stem().unwrap().to_str().unwrap(),
                &py_inputs,
                py,
            )
            .unwrap();

            self.ports
                .outputs
                .iter()
                .map(|(k, port_type)| (k.clone(), Self::extract_py_data(port_type, &out[k], py)))
                .collect()
        })
    }

    pub fn py_node_path(node_name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("nodes/{node_name}.py"))
    }
    pub fn extract_py_data(port_type: &PortType, py_object: &PyObject, py: Python) -> PortData {
        unsafe {
            match port_type {
                PortType::Integer => {
                    PortData::Integer(py_object.bind(py).downcast().unwrap().as_array().to_owned())
                }
                PortType::Real => {
                    PortData::Real(py_object.bind(py).downcast().unwrap().as_array().to_owned())
                }
                PortType::Complex => {
                    PortData::Complex(py_object.bind(py).downcast().unwrap().as_array().to_owned())
                }
                PortType::Real2d => {
                    PortData::Real2d(py_object.bind(py).downcast().unwrap().as_array().to_owned())
                }
                PortType::Complex2d => PortData::Complex2d(
                    py_object.bind(py).downcast().unwrap().as_array().to_owned(),
                ),
            }
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
    fn try_from(value: PyFacingPortDef) -> Result<Self, Self::Error> {
        Ok(PortDef {
            inputs: value
                .clone()
                .inputs
                .into_iter()
                .map(|(key, value)| PortType::from_str(&value).map(|v| (key, v)))
                .collect::<Result<OrderMap<_, _>, _>>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Expected port values of {:?}, found {:?}",
                        PortType::VARIANTS,
                        value.inputs
                    )
                }),
            outputs: value
                .outputs
                .into_iter()
                .map(|(key, value)| PortType::from_str(&value).map(|v| (key, v)))
                .collect::<Result<OrderMap<_, _>, _>>()
                .unwrap_or_else(|_| {
                    panic!(
                        "Expected port values of {:?}, found {:?}",
                        PortType::VARIANTS,
                        value.inputs
                    )
                }),
        })
    }
    type Error = strum::ParseError;
}
