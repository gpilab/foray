pub mod constant;
pub mod linspace;
pub mod math_nodes;
pub mod plot;

use iced::{
    widget::{canvas::path::lyon_path::math::Size, column, container, row, text},
    Element,
};

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

pub fn node_display<'a>(
    node: &NetworkNode,
    id: u32,
    input_data: Option<OrderMap<SmolStr, &RefCell<PortData>>>,
) -> (Element<'a, Message>, Size) {
    let default_node_size = Size::new(NODE_WIDTH, NODE_HEIGHT);
    match &node.data.operation {
        Operation::Constant(value) => (constant::view(id, *value), default_node_size),
        Operation::Linspace { start, stop, num } => {
            (linspace::view(id, *start, *stop, *num), default_node_size)
        }
        Operation::Plot => (plot::view(id, input_data), default_node_size * 2.),
        Operation::Add
        | Operation::Subtract
        | Operation::Multiply
        | Operation::Divide
        | Operation::Identity => (
            text(node.data.short_name.clone()).size(30.).into(),
            default_node_size,
        ),
    }
}

pub fn format_node_output<'a>(
    data: &OrderMap<SmolStr, Option<&RefCell<PortData>>>,
) -> Element<'a, Message> {
    //TODO: clean this up by iterating straight to text elements?
    let node_output = data.into_iter().map(|(port_name, d)| {
        (
            port_name.to_string(),
            d.map(|d| format!("{}", d.borrow()))
                .unwrap_or("n/a".to_string()),
        )
    });

    container(column(node_output.map(|(lbl, val)| {
        row![text(lbl).size(12.), text(val).size(12.)]
            .spacing(5.0)
            .into()
    })))
    .into()
}
