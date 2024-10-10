//use gpirs::{port::PortValue, pyo::gpipy_compute};
use petgraph::graph::NodeIndex;
use serde::Serialize;

use crate::{
    network::Network,
    port::{ArrayValue, Port, PortName, Primitive, Real},
};

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
                node.update_output_data(
                    "out",
                    Port::Array(ArrayValue::Real(
                        vec![*amount],
                        vec![(*val).into(); *amount],
                    )),
                );
            }
            NodeType::Add => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));
                let b = g.retrieve_input_data(node, &PortName::from("b"));

                let Port::Array(ArrayValue::Real(a_shape, a)) = a else {
                    todo!()
                };
                let Port::Array(ArrayValue::Real(_, b)) = b else {
                    todo!()
                };

                let out = Port::Array(ArrayValue::Real(
                    a_shape.clone(),
                    a.iter().zip(b).map(|(ai, bi)| (*ai + *bi)).collect(),
                ));

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
            NodeType::_Sub => {}
            NodeType::Offset(val) => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));
                let Port::Array(ArrayValue::Real(a_shape, a)) = a else {
                    todo!()
                };

                let out = Port::Array(ArrayValue::Real(
                    a_shape.clone(),
                    a.iter().map(|ai| *ai + (*val).into()).collect(),
                ));

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
            NodeType::Sum => {
                let a = g.retrieve_input_data(node, &PortName::from("a"));
                let Port::Array(ArrayValue::Real(_a_shape, a)) = a else {
                    todo!()
                };
                //TODO: is this copy bad?
                let sum: Real = a.iter().copied().sum();
                let out = Port::Primitive(Primitive::Real(sum));

                let node = g.g.node_weight_mut(nx).unwrap();
                node.update_output_data("out", out);
            }
            NodeType::Python(_py_node) => {
                todo!();
                //    let a = g.retrieve_input_data(node, &PortName::from("a"));
                //    let b = g.retrieve_input_data(node, &PortName::from("b"));
                //    let Port::Array(ArrayValue::Real(a_shape, a)) = a else {
                //        todo!()
                //    };
                //    let Port::Array(ArrayValue::Real(_, b)) = b else {
                //        todo!()
                //    };
                //
                //    let a = a
                //        .iter()
                //        .map(|ai| {
                //            let PortValue::Real(ai) = ai else { todo!() };
                //            PortValue::Real(*ai as f64)
                //        })
                //        .collect();
                //
                //    let PortValue::Real(b) = b else { todo!() };
                //    let out = gpipy_compute(
                //        "scale_array",
                //        gpirs::node::NodeInputValue(HashMap::from([
                //            ("a".to_string(), PortValue::Array(a)),
                //            ("b".to_string(), PortValue::Real(*b as f64)),
                //        ])),
                //    )
                //    .map_err(|op| op.to_string());
                //    let out = out.unwrap().get_first();
                //
                //    let PortValue::Array(out) = out else { todo!() };
                //    let out = PortValue::Vec(
                //        out.into_iter()
                //            .map(|oi| {
                //                let PortValue::Real(oi) = oi else { todo!() };
                //                PortValue::Real(oi as f32)
                //            })
                //            .collect(),
                //    );
                //
                //    let node = g.g.node_weight_mut(nx).unwrap();
                //    node.update_output_data("out", out);
                //}
                //};
            }
        };
    }
}
