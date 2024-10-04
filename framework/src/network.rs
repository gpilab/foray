use std::fmt::Debug;

use petgraph::{
    graph::{DiGraph, EdgeIndex},
    visit::Topo,
};
use serde::Serialize;

use crate::{
    node::Node,
    node_type::NodeType,
    port::{NodeIndex, OutputPortId, PortData, PortName, PortType},
};

#[derive(Debug, Serialize)]
pub struct Network {
    pub(crate) g: DiGraph<Node, ()>,
}
impl Default for Network {
    ///Initialize an empty network
    fn default() -> Self {
        Network {
            g: DiGraph::<Node, ()>::new(),
        }
    }
}
impl Network {
    //// Mutators

    /// Add a new node to the network
    pub fn add_node(
        &mut self,
        n_type: NodeType,
        input: Vec<(PortName, PortType)>,
        output: Vec<(PortName, PortType)>,
    ) -> NodeIndex {
        let node_id = self.g.add_node(Node::new(n_type, input, output));
        // now that we have a node_id, set it on the node
        self.g.node_weight_mut(node_id).unwrap().node_id = node_id;

        node_id
    }

    /// connect an input node to an output node
    pub fn connect_nodes<T: Into<PortName> + Clone>(
        &mut self,
        from_node_idx: NodeIndex,
        from_port_name: T,
        to_node_idx: NodeIndex,
        to_port_name: T,
    ) -> EdgeIndex {
        //// Check Port Compatability
        {
            // need to use immutable references to check two nodes at once
            let from_node = self.g.node_weight(from_node_idx).unwrap();
            let to_node = self.g.node_weight(to_node_idx).unwrap();
            if !from_node.can_connect_child(from_port_name.clone(), to_node, to_port_name.clone()) {
                // TODO: Custom error types and returning a result would be nice here
                panic!(
                    "Tried to connect incompatible ports: {:?}->{:?}",
                    from_node, to_node
                );
            }
        }
        //// Update to_node's input to point to from_node
        let to_node = self.g.node_weight_mut(to_node_idx).unwrap();
        to_node.connect_input(to_port_name.into(), from_node_idx, from_port_name.into());

        //// Create the edge in the graph as well
        self.g.add_edge(from_node_idx, to_node_idx, ())
    }

    /// Loop through the graph and propogate values
    pub fn process(&mut self) {
        let mut topo = Topo::new(&self.g);
        while let Some(nx) = topo.next(&self.g) {
            let node = self.g.node_weight(nx).unwrap();
            node.node_type.clone().compute(nx, self);
        }
    }

    //// Accessors

    pub fn get_output_data(&self, port_id: OutputPortId) -> &PortData {
        self.g
            .node_weight(port_id.node_id)
            .unwrap()
            .get_output_data(&port_id.port_name)
    }

    pub(crate) fn retrieve_input_data(&self, node: &Node, input_port_name: &PortName) -> &PortData {
        let parent_port_id = node.get_connected_port_id(input_port_name);
        self.get_output_data(parent_port_id)
    }
}
