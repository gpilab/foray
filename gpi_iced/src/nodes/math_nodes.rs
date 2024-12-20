use std::ops::Deref;

use derive_more::derive::Display;
use ndarray::Array1;

use crate::graph::GraphNode;

#[derive(Debug)]
pub struct Node {
    pub short_name: String,
    pub full_name: String,
    pub operation: Operation,
}

#[derive(Clone, Debug)]
pub enum PortType {
    Integer,
    Real,
    Complex,
}

#[derive(Debug, Clone)]
pub enum PortData {
    Integer(Array1<i64>),
    Real(Array1<f64>),
    Complex(Array1<(f64, f64)>),
}

impl From<&PortData> for PortType {
    fn from(value: &PortData) -> Self {
        match value {
            PortData::Integer(_) => PortType::Integer,
            PortData::Real(_) => PortType::Real,
            PortData::Complex(_) => PortType::Complex,
        }
    }
}

impl std::fmt::Display for PortData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(data) => write!(f, "{:?}", data.to_vec()),
            Self::Real(data) => write!(f, "{:.2?}", data.to_vec()),
            Self::Complex(data) => write!(f, "{:.2?}", data.to_vec()),
        }
    }
}

#[derive(Display, Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Constant(f64),
    Identity,
    #[display("({}..={}),#{}", start, stop, num)]
    Linspace {
        start: f64,
        stop: f64,
        num: i64,
    },
    Plot,
}

#[allow(clippy::complexity)]
fn binary_operation(
    short_name: impl Into<String>,
    full_name: impl Into<String>,
    operation: Operation,
    f: Box<dyn Fn(&Array1<f64>, &Array1<f64>) -> Array1<f64>>,
) -> GraphNode<Node, PortType, PortData> {
    GraphNode::new(
        Node {
            short_name: short_name.into(),
            full_name: full_name.into(),
            operation,
        },
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
    binary_operation("+", "Addition", Operation::Add, Box::new(|a, b| a + b))
}
pub fn subtract() -> GraphNode<Node, PortType, PortData> {
    binary_operation(
        "\u{2212}",
        "Subtraction",
        Operation::Subtract,
        Box::new(|a, b| a - b),
    )
}
pub fn multiply() -> GraphNode<Node, PortType, PortData> {
    binary_operation("x", "Multiply", Operation::Multiply, Box::new(|a, b| a * b))
}
pub fn divide() -> GraphNode<Node, PortType, PortData> {
    binary_operation("÷", "Divide", Operation::Divide, Box::new(|a, b| a / b))
}

pub fn identity_node(port_type: PortType) -> GraphNode<Node, PortType, PortData> {
    GraphNode::new(
        Node {
            short_name: "I".to_string(),
            full_name: "Identity".to_string(),
            operation: Operation::Identity,
        },
        vec![("in", &port_type)],
        vec![("out", &port_type)],
        Box::new(|a, _| [("out".into(), a["in"].borrow().clone())].into()),
    )
}
