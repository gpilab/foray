use std::ops::Deref;

use ndarray::Array1;

use crate::graph::GraphNode;

use super::{Node, PortData, PortType};

#[allow(clippy::complexity)]
fn binary_operation(
    node: Node,
    f: Box<dyn Fn(&Array1<f64>, &Array1<f64>) -> Array1<f64>>,
) -> GraphNode<Node, PortType, PortData> {
    GraphNode::new(
        node,
        vec![("a", &PortType::Real), ("b", &PortType::Real)],
        vec![("out", &PortType::Real)],
        Box::new(move |inputs, _| {
            let out = match (inputs["a"].borrow().deref(), inputs["b"].borrow().deref()) {
                (PortData::Real(a), PortData::Real(b)) => f(a, b),
                _ => panic!("bad inputs!"),
            };

            [("out".into(), PortData::Real(out))].into()
        }),
    )
}

pub fn add() -> GraphNode<Node, PortType, PortData> {
    binary_operation(Node::Add, Box::new(|a, b| a + b))
}

pub fn subtract() -> GraphNode<Node, PortType, PortData> {
    binary_operation(Node::Subtract, Box::new(|a, b| a - b))
}
//"\u{2212}",
//
pub fn multiply() -> GraphNode<Node, PortType, PortData> {
    binary_operation(Node::Multiply, Box::new(|a, b| a * b))
}

pub fn divide() -> GraphNode<Node, PortType, PortData> {
    binary_operation(Node::Divide, Box::new(|a, b| a / b))
}
//"÷"

pub fn identity_node() -> Node {
    Node::Identity
}

pub fn identity_node_network(port_type: PortType) -> GraphNode<Node, PortType, PortData> {
    GraphNode::new(
        identity_node(),
        vec![("in", &port_type)],
        vec![("out", &port_type)],
        Box::new(|a, _| [("out".into(), a["in"].borrow().clone())].into()),
    )
}
