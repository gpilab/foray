use std::fmt::Debug;

use petgraph::{
    graph::{DiGraph, EdgeIndex, NodeIndex},
    visit::Topo,
};

use crate::{
    node::{Node, PortId},
    node_type::NodeType,
};

pub const STRESS_SIZE: usize = 100_000_000;

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
    pub fn add_node(&mut self, n_type: NodeType) -> NodeIndex {
        let id = self.g.node_count() + 1;
        self.g.add_node(Node::new(id.to_string(), n_type))
    }
    pub fn add_edge(&mut self, from: PortId, to: PortId) -> EdgeIndex {
        //// From node records it's output port with no reference to child
        self.g
            .node_weight_mut(from.0)
            .unwrap()
            .output
            .insert(from.1.clone(), None);

        //// To Node references its parent, From Node
        self.g
            .node_weight_mut(to.0)
            .unwrap()
            .input
            .insert(to.1.clone(), (from.0.clone(), from.1.clone()));

        //// Store the edge in the graph
        self.g.add_edge(from.0, to.0, ())
    }

    /// Loop through the graph and propogate values
    pub fn process(&mut self) {
        let mut topo = Topo::new(&self.g);
        while let Some(nx) = topo.next(&self.g) {
            let node = self.g.node_weight(nx).unwrap();
            node.n_type.clone().compute(nx, self);
        }
    }
}
