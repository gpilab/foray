use std::collections::HashMap;
use std::fmt::Debug;

use serde::Serialize;

use crate::node_type::NodeType;
use crate::port::NodeIndex;
use crate::port::{InputPort, OutputPort, PortName};
use crate::port::{Port, PortType};

#[derive(Serialize)]
pub(crate) struct Node {
    /// Determines how outputs are calculated from inputs
    pub node_type: NodeType,
    /// This node's index in the `Network`
    pub node_id: NodeIndex,
    /// Each port is named
    /// Output data is stored directly in the node
    output: HashMap<PortName, OutputPort>,
    /// Inputs don't store data,
    /// they only store their type, and the node+port they are connected to
    input: HashMap<PortName, InputPort>,
}

impl Node {
    pub fn new(
        node_type: NodeType,
        input: Vec<(PortName, PortType)>,
        output: Vec<(PortName, PortType)>,
    ) -> Self {
        Node {
            node_type,
            // this will be updated immediately below!
            node_id: 0.into(),
            input: input
                .iter()
                .map(|(name, pt)| (name.clone(), InputPort::Empty(pt.clone())))
                .collect(),
            output: output
                .iter()
                .map(|(name, pt)| (name.clone(), OutputPort::Empty(pt.clone())))
                .collect(),
        }
    }

    pub fn can_connect_child<T: Into<PortName>>(
        &self,
        local_port_name: T,
        to_node: &Node,
        to_port_name: T,
    ) -> bool {
        //// from port exists
        let from_port = self
            .output
            .get(&local_port_name.into())
            .expect("port should exist on node");

        //// to port exists
        let to_port = to_node
            .input
            .get(&to_port_name.into())
            .expect("port should exist on node");

        //// PortTypes match
        PortType::from(from_port) == PortType::from(to_port)
    }

    /// Set the data for an ouput port
    pub fn update_output_data<T: Into<PortName>>(&mut self, port_name: T, port_data: Port) {
        self.output
            .insert(port_name.into(), OutputPort::Filled(port_data))
            .expect("port should exists on Node");
    }

    /// set an input port to point to a parent port
    /// The `network` should also be updated!!!
    pub fn connect_input(
        &mut self,
        local_port_name: PortName,
        parent_node: NodeIndex,
        parent_node_name: PortName,
    ) {
        let input_port = self.input.get(&local_port_name).unwrap();
        self.input.insert(
            local_port_name,
            InputPort::Connected(input_port.into(), parent_node, parent_node_name),
        );
    }

    pub(crate) fn get_output_data(&self, port_name: &PortName) -> &Port {
        self.output.get(port_name).unwrap().get_data()
    }

    pub(crate) fn get_connected_port_id(&self, port_name: &PortName) -> (NodeIndex, PortName) {
        self.input.get(port_name).unwrap().get_connected_port_id()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t{{
\t  type:{:?},
\t  inputs:{:?},
\t  outputs:{:?}
\t}}
",
            self.node_type, self.input, self.output
        )
    }
}

//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn can_connect_node() {
//        //node 1
//        let n1 = Node::new(
//            NodeType::Add,
//            vec![("a".into(), PortType::Integer)],
//            vec![
//                ("out1".into(), PortType::Integer),
//                ("out2".into(), PortType::Real),
//            ],
//        );
//        //node 2
//        let n2 = Node::new(
//            NodeType::Add,
//            vec![("a".into(), PortType::Integer)],
//            vec![("out".into(), PortType::Integer)],
//        );
//
//        //can node 1 connect to node2?
//        assert!(n1.can_connect_child("out1", &n2, "a"));
//        assert!(!n1.can_connect_child("out2", &n2, "a"));
//    }
//}
