use std::fmt::Debug;

use petgraph::{
    graph::{DiGraph, EdgeIndex, NodeIndex},
    visit::Topo,
};

use crate::{
    node::{Node, PortId, PortName},
    port::PortData,
};

#[derive(Debug)]
pub struct Network {
    pub g: DiGraph<Node, ()>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            g: DiGraph::<Node, ()>::new(),
        }
    }
    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        self.g.add_node(node)
    }

    pub fn add_edge(
        &mut self,
        from_node_idx: NodeIndex,
        from_port_name: PortName,
        to_node_idx: NodeIndex,
        to_port_name: PortName,
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
        to_node.connect_input(to_port_name, from_node_idx, from_port_name);

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

    pub fn get_output_data(&self, port_id: PortId) -> &PortData {
        self.g
            .node_weight(port_id.0)
            .unwrap()
            .output
            .get(&port_id.1)
            .unwrap()
            .get_data()
    }

    pub(crate) fn retrieve_input_data(&self, node: &Node, input_port_name: &PortName) -> &PortData {
        let parent_port_id = node
            .input
            .get(input_port_name)
            .unwrap()
            .get_connected_port_id();
        self.get_output_data(parent_port_id)
    }
}
