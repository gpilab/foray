pub mod constant;
pub mod linspace;
pub mod math_nodes;
pub mod plot;

use iced::{
    widget::{button, column, container, container::bordered_box, horizontal_rule, row, text},
    Alignment::Center,
    Color, Element,
    Length::Fill,
    Size,
};

use math_nodes::Operation;
use ndarray::Array1;
use ordermap::OrderMap;
use smol_str::SmolStr;
use std::cell::RefCell;
use strum::{IntoEnumIterator, VariantNames};

use crate::{app::Message, graph::GraphNode, style};

pub const NODE_WIDTH: f32 = 100.;
pub const NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 7.5;
pub const NODE_RADIUS: f32 = 5.0;
pub const NODE_BORDER_WIDTH: f32 = 1.0;

#[derive(Debug)]
pub struct Node {
    pub short_name: String,
    pub full_name: String,
    pub operation: Operation,
}

pub type NetworkNode = GraphNode<Node, PortType, PortData>;
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

pub trait GUINode {
    type Config: Default;
    type NodeData: Default;

    fn network_node(config: Self::Config) -> NetworkNode;
    fn view<'a>(id: u32, config: Self::Config) -> Element<'a, Message>;
}

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
            Size::new(60., 60.),
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

pub(crate) fn available_nodes_view<'a>() -> Element<'a, Message> {
    container(
        container(
            column(
                math_nodes::Operation::iter()
                    .zip(Operation::VARIANTS)
                    .map(|(o, name)| {
                        button(
                            row![horizontal_rule(0.0), container(text(*name)).padding(4.0)]
                                //.spacing(4.0)
                                .align_y(Center),
                        )
                        .padding(0.)
                        .on_press(Message::AddNode(o))
                        .width(Fill)
                        .style(style::button::list)
                        .into()
                    }),
            )
            .width(150.),
        )
        .style(|t| bordered_box(t).background(Color::TRANSPARENT)),
    )
    .center_x(Fill)
    .into()
}
