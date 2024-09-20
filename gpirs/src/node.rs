use std::{collections::HashMap, error::Error, fs::File, io::Read, path::Path};

use anyhow::Context;
use pyo3::{FromPyObject, IntoPy, PyObject};
use serde::{Deserialize, Serialize};

use crate::{
    port::{PortType, PortValue},
    pyo::gpipy_config,
};

#[derive(Serialize, FromPyObject, PartialEq, Eq, Debug)]
pub struct NodeInputType(pub HashMap<String, PortType>);

#[derive(Serialize, FromPyObject, PartialEq, Eq, Debug)]
pub struct NodeOutputType(pub HashMap<String, PortType>);

#[derive(Serialize, Deserialize, FromPyObject, PartialEq, Debug)]
pub struct NodeInputValue(pub HashMap<String, PortValue>);

impl NodeInputValue {
    pub fn new(inputs: Vec<(String, PortValue)>) -> Self {
        NodeInputValue(inputs.into_iter().collect::<HashMap<String, PortValue>>())
    }
}
impl IntoPy<PyObject> for NodeInputValue {
    fn into_py(self, py: pyo3::Python<'_>) -> PyObject {
        self.0.into_py(py)
    }
}
#[derive(Serialize, FromPyObject, PartialEq, Debug)]
pub struct NodeOutputValue(pub HashMap<String, PortValue>);

impl NodeOutputValue {
    pub fn new(port_value: PortValue) -> Self {
        NodeOutputValue(HashMap::from([("out".into(), port_value)]))
    }
    pub fn get_first(&self) -> PortValue {
        let first_key = self.0.keys().next().unwrap();
        self.0.get(first_key).unwrap().clone()
    }
}

#[derive(Serialize)]
pub struct PythonNode {
    pub node_type: String,
    pub source: Source,
    pub config: HashMap<String, String>,
    pub input_types: NodeInputType,
    pub output_types: NodeOutputType,
}

impl PythonNode {
    //TODO: handle error internally
    pub fn new(module_path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut contents = String::new();
        let module_name = module_path.file_stem().unwrap().to_str().unwrap();
        dbg!(module_path);

        File::open(module_path)
            .with_context(|| format!("Tried to load file, but failed: {}", module_path.display()))?
            .read_to_string(&mut contents)
            .with_context(|| {
                format!(
                    "Encountered problem reading file: {}",
                    module_path.display()
                )
            })?;

        let (inputs, outputs) = gpipy_config(&module_name)?;

        Ok(PythonNode {
            node_type: module_name.to_string(),
            source: Source::Local(LocalNodeData {
                path: module_path.to_string_lossy().into(),
                file_contents: contents,
                // TODO: actually validate node
                validation_status: NodeValidation::Valid,
            }),
            config: HashMap::new(),
            input_types: inputs,
            output_types: outputs,
        })
    }
}

#[derive(serde::Serialize)]
pub enum Source {
    Local(LocalNodeData),
    Remote,
}

#[derive(serde::Serialize)]
pub struct LocalNodeData {
    pub path: String,
    pub file_contents: String,
    pub validation_status: NodeValidation,
}

#[derive(serde::Serialize)]
pub enum NodeValidation {
    Valid,
    Warning(String),
    Error(String),
}
