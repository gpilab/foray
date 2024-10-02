use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::node_type::NodeType;

type PortData = Vec<i32>;

pub type PortId = (NodeIndex, String);

pub struct Node {
    /// name just used for debugging context
    name: String,
    /// inputs store the the index of the node they are connected to
    pub input: HashMap<String, PortId>,
    /// Output data is stored in the node
    pub output: HashMap<String, Option<PortData>>,
    /// Determines how outputs are calculated from inputs
    pub n_type: NodeType,
}

impl Node {
    //Initialize a node with no connections
    pub fn new(name: String, n_type: NodeType) -> Self {
        Node {
            name,
            n_type,
            input: [].into(),
            output: [].into(),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\t{{
\t  name: {},
\t  type:{:?},
\t  inputs:{:?},
\t  outpus:{:?}
\t}}
",
            self.name,
            self.n_type,
            self.input,
            self.output.len()
        )
    }
}
