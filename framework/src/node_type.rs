use petgraph::graph::NodeIndex;

use crate::network::{Network, STRESS_SIZE};

#[derive(Debug, Clone)]
pub enum NodeType {
    Const(i32),
    Add,
    _Sub,
    Offset(i32),
}

impl NodeType {
    pub fn compute(&self, nx: NodeIndex, g: &mut Network) {
        match self {
            NodeType::Const(val) => {
                let node = g.g.node_weight_mut(nx).unwrap();
                node.output
                    .insert("out".into(), Some(vec![*val; STRESS_SIZE]));
            }
            NodeType::Add => {
                let node = g.g.node_weight(nx).unwrap();

                let inputs = &node.input;
                let (a_index, _a_port) = inputs.get("a".into()).unwrap();
                let a =
                    g.g.node_weight(*a_index)
                        .unwrap()
                        .output
                        .get("out")
                        .unwrap()
                        .as_ref()
                        .unwrap();

                let (b_index, _b_port) = inputs.get("b".into()).unwrap();
                let b =
                    g.g.node_weight(*b_index)
                        .unwrap()
                        .output
                        .get("out")
                        .unwrap()
                        .as_ref()
                        .unwrap();

                let out = a.iter().zip(b).map(|(ai, bi)| ai + bi).collect();

                let node = g.g.node_weight_mut(nx).unwrap();
                node.output.insert("out".into(), Some(out));
            }
            NodeType::_Sub => {}
            NodeType::Offset(val) => {
                let node = g.g.node_weight(nx).unwrap();

                let inputs = &node.input;
                let (a_index, _a_port) = inputs.get("a".into()).unwrap();
                let a =
                    g.g.node_weight(*a_index)
                        .unwrap()
                        .output
                        .get("out")
                        .unwrap()
                        .as_ref()
                        .unwrap();

                //let out = a + val;
                let out = a.iter().map(|ai| ai + val).collect();

                let node = g.g.node_weight_mut(nx).unwrap();
                node.output.insert("out".into(), Some(out));
            }
        }
    }
}
