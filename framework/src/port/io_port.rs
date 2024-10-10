use serde::Serialize;

use super::{NodeIndex, Port, PortType, PrimitiveType};

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
    Filled(Port),
}
//
impl From<&Port> for PortType {
    fn from(value: &Port) -> Self {
        match value {
            Port::Primitive(p) => PortType::Primitive(PrimitiveType::from(p)),
            Port::Array(_) => todo!(),
            Port::Struct(_) => todo!(),
        }
    }
}

impl InputPort {
    pub fn get_connected_port_id(&self) -> (NodeIndex, PortName) {
        match self {
            Self::Empty(_) => panic!("Output is not populated"),
            Self::Connected(_, n_index, port_name) => (*n_index, port_name.clone()),
        }
    }
}

impl OutputPort {
    pub(crate) fn get_data(&self) -> &Port {
        match self {
            Self::Empty(_) => panic!("Output is not populated"),
            Self::Filled(port_data) => port_data,
        }
    }
}

impl From<&InputPort> for PortType {
    fn from(value: &InputPort) -> Self {
        match value {
            InputPort::Connected(pt, _, _) | InputPort::Empty(pt) => pt.clone(),
        }
    }
}

impl From<&OutputPort> for PortType {
    fn from(value: &OutputPort) -> Self {
        match value {
            OutputPort::Empty(pt) => pt.clone(),
            OutputPort::Filled(pd) => pd.into(),
        }
    }
}

impl From<&str> for PortName {
    fn from(value: &str) -> Self {
        PortName(value.to_string())
    }
}

// A Port is uniquely identified with a `NodeIndex` and a `PortName`
//pub struct InputPortId {
//    pub node_id: NodeIndex,
//    pub port_name: PortName,
//    pub port_type: PortType,
//}
//
//pub struct OutputPortId {
//    pub node_id: NodeIndex,
//    pub port_name: PortName,
//    pub port_type: PortType,
//}
