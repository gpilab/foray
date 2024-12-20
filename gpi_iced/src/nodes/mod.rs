pub mod constant;
pub mod linspace;
pub mod math_nodes;
pub mod plot;

use iced::{widget::text, Element};

pub type NetworkNode = GraphNode<Node, PortType, PortData>;

use math_nodes::{Node, Operation, PortData, PortType};
use ordermap::OrderMap;
use smol_str::SmolStr;
use std::cell::RefCell;

use crate::{app::Message, graph::GraphNode};

pub const NODE_WIDTH: f32 = 100.;
pub const NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 7.5;
pub const NODE_RADIUS: f32 = 5.0;

pub fn node_display(node: &NetworkNode, id: u32) -> Element<Message> {
    match &node.data.operation {
        Operation::Constant(value) => constant::view(id, *value),
        Operation::Linspace { start, stop, num } => linspace::view(id, *start, *stop, *num),
        Operation::Plot => plot::view(id),
        Operation::Add
        | Operation::Subtract
        | Operation::Multiply
        | Operation::Divide
        | Operation::Identity => text(node.data.short_name.clone()).size(30.).into(),
    }
}

pub fn format_node_output(
    data: &OrderMap<SmolStr, Option<&RefCell<PortData>>>,
) -> Vec<(String, String)> {
    data.into_iter()
        .map(|(port_name, d)| {
            (
                port_name.to_string(),
                d.map(|d| format!("{}", d.borrow()))
                    .unwrap_or("n/a".to_string()),
            )
        })
        .collect()
}
