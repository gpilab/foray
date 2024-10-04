use std::collections::HashMap;

use serde::Serialize;

pub type NodeIndex = petgraph::graph::NodeIndex;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize)]
pub struct PortName(String);

/// Input Ports always have a type.
/// If connected, they identify their connection with a `NodeIndex` and `PortName`
#[derive(Debug, Serialize)]
pub(crate) enum InputPort {
    Empty(PortType),
    Connected(PortType, NodeIndex, PortName),
}

/// Output ports always have a type.
/// They optionally have data
#[derive(Debug, Serialize)]
pub(crate) enum OutputPort {
    Empty(PortType),
    Filled(PortData),
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

impl InputPort {
    pub fn get_connected_port_id(&self) -> OutputPortId {
        match self {
            Self::Empty(_) => panic!("Output is not populated"),
            Self::Connected(port_type, n_index, port_name) => OutputPortId {
                node_id: *n_index,
                port_name: port_name.clone(),
                port_type: *port_type,
            },
        }
    }
}

impl OutputPort {
    pub(crate) fn get_data(&self) -> &PortData {
        match self {
            Self::Empty(_) => panic!("Output is not populated"),
            Self::Filled(port_data) => port_data,
        }
    }
}

impl From<&InputPort> for PortType {
    fn from(value: &InputPort) -> Self {
        match value {
            InputPort::Empty(pt) => *pt,
            InputPort::Connected(pt, _, _) => *pt,
        }
    }
}
impl From<&OutputPort> for PortType {
    fn from(value: &OutputPort) -> Self {
        match value {
            OutputPort::Empty(pt) => *pt,
            OutputPort::Filled(pd) => pd.into(),
        }
    }
}

impl From<&str> for PortName {
    fn from(value: &str) -> Self {
        PortName(value.to_string())
    }
}

/// A Port is uniquely identified with a NodeIndex and a PortName
pub struct InputPortId {
    pub node_id: NodeIndex,
    pub port_name: PortName,
    pub port_type: PortType,
}

pub struct OutputPortId {
    pub node_id: NodeIndex,
    pub port_name: PortName,
    pub port_type: PortType,
}
