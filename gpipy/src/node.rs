use std::{collections::HashMap, error::Error, fs::File, io::Read, path::Path};

use anyhow::Context;

use crate::pyo::gpipy_config;

#[derive(serde::Serialize)]
pub struct PythonNode {
    pub source: Source,
    pub config: HashMap<String, String>,
    pub input_types: Vec<String>,
    pub output_types: Vec<String>,
}

impl PythonNode {
    pub fn new(module_path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut contents = String::new();
        let module_name = module_path.file_stem().unwrap();

        File::open(module_path)
            .with_context(|| format!("Tried to load file, but failed: {}", module_path.display()))?
            .read_to_string(&mut contents)
            .with_context(|| {
                format!(
                    "Encountered problem reading file: {}",
                    module_path.display()
                )
            })?;

        let (inputs, outputs) = gpipy_config(&module_name.to_str().unwrap())?;

        Ok(PythonNode {
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
