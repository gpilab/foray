use petgraph::graph::NodeIndex;
use serde::Serialize;

use crate::{network::Network, node::PortName, port::PortData};

//const STRESS_SIZE: usize = 100_000_000;
const STRESS_SIZE: usize = 3;
#[derive(Debug, Clone, Serialize)]
pub enum NodeType {
    Const(f64),
    Add,
    _Sub,
    Offset(f64),
}

impl NodeType {
    pub fn compute(&self, nx: NodeIndex, g: &mut Network) {
        dbg!("Starting compute", self, &g);
        let node = g.g.node_weight(nx).unwrap();
        match self {
            NodeType::Const(val) => {
                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data(
                    "out",
                    PortData::Vec(vec![PortData::Real(*val); STRESS_SIZE]),
                );
            }
            NodeType::Add => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));
                let b = g.retrieve_input_data(node, &PortName::from("b"));

                let PortData::Vec(a) = a else { todo!() };
                let PortData::Vec(b) = b else { todo!() };

                let out = PortData::Vec(
                    a.iter()
                        .zip(b)
                        .map(|(ai, bi)| {
                            let PortData::Real(ai) = ai else { todo!() };

                            let PortData::Real(bi) = bi else { todo!() };
                            PortData::Real(ai + bi)
                        })
                        .collect(),
                );

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
            NodeType::_Sub => {}
            NodeType::Offset(val) => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));

                let PortData::Vec(a) = a else { todo!() };

                let out = PortData::Vec(
                    a.iter()
                        .map(|ai| {
                            let PortData::Real(ai) = ai else { todo!() };
                            PortData::Real(ai + val)
                        })
                        .collect(),
                );

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
        };
    }
}
