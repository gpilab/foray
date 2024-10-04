use std::collections::HashMap;

use gpirs::{port::PortValue, pyo::gpipy_compute};
use petgraph::graph::NodeIndex;
use serde::Serialize;

use crate::{network::Network, port::PortData, port::PortName};

#[derive(Debug, Clone, Serialize)]
pub enum NodeType {
    Fill(f64, usize),
    Add,
    _Sub,
    Offset(f64),
    Sum,
    Python(String),
}

impl NodeType {
    pub fn compute(&self, nx: NodeIndex, g: &mut Network) {
        //dbg!("Starting compute", self, &g);
        let node = g.g.node_weight(nx).unwrap();
        match self {
            NodeType::Fill(val, amount) => {
                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", PortData::Vec(vec![PortData::Real(*val); *amount]));
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
            NodeType::Sum => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));
                let PortData::Vec(a) = a else { todo!() };

                let out = PortData::Real(
                    a.iter()
                        .map(|ai| {
                            let PortData::Real(ai) = ai else { todo!() };
                            ai
                        })
                        .sum(),
                );

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
            NodeType::Python(py_node) => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));
                let b = g.retrieve_input_data(node, &PortName::from("b"));
                let PortData::Vec(a) = a else { todo!() };
                let a = a
                    .iter()
                    .map(|ai| {
                        let PortData::Real(ai) = ai else { todo!() };
                        PortValue::Real(*ai)
                    })
                    .collect();

                let PortData::Real(b) = b else { todo!() };
                let out = gpipy_compute(
                    "scale_array",
                    gpirs::node::NodeInputValue(HashMap::from([
                        ("a".to_string(), PortValue::Array(a)),
                        ("b".to_string(), PortValue::Real(*b)),
                    ])),
                )
                .map_err(|op| op.to_string());
                let out = out.unwrap().get_first();

                let PortValue::Array(out) = out else { todo!() };
                let out = PortData::Vec(
                    out.into_iter()
                        .map(|oi| {
                            let PortValue::Real(oi) = oi else { todo!() };
                            PortData::Real(oi)
                        })
                        .collect(),
                );

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
        };
    }
}
