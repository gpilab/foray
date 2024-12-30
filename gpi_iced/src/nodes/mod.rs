pub mod constant;
pub mod linspace;
pub mod math_nodes;
pub mod plot;

use derive_more::derive::Display;
use iced::{
    widget::{button, column, container, container::bordered_box, horizontal_rule, row, text},
    Alignment::Center,
    Color, Element,
    Length::Fill,
};

use math_nodes::{add, divide, identity_node, identity_node_network, multiply, subtract};
use ndarray::Array1;
use ordermap::OrderMap;
use smol_str::SmolStr;
use std::cell::RefCell;
use strum::{EnumIter, IntoEnumIterator, VariantNames};
//use strum::{IntoEnumIterator, VariantNames};

use crate::{app::Message, graph::GraphNode, style};

pub const NODE_WIDTH: f32 = 100.;
pub const NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 7.5;
pub const NODE_RADIUS: f32 = 5.0;
pub const NODE_BORDER_WIDTH: f32 = 1.0;

#[derive(Display, Debug, Clone, VariantNames, EnumIter)]
pub enum Node {
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

impl GUINode for Node {
    fn network_node(self) -> NetworkNode {
        match &self {
            Node::Constant(value) => constant::constant_node(*value),
            Node::Linspace { start, stop, num } => {
                linspace::linspace_node_network(*start as f32, *stop as f32, *num as i32)
            }
            Node::Plot => plot::node(),
            Node::Add => add(),
            Node::Subtract => subtract(),
            Node::Multiply => multiply(),
            Node::Divide => divide(),
            Node::Identity => identity_node_network(PortType::Real),
        }
    }

    fn name(&self) -> String {
        match &self {
            Node::Constant(..) => "constant",
            Node::Linspace { .. } => "linspace",
            Node::Plot => "plot",
            Node::Add => "add",
            Node::Subtract => "subtract",
            Node::Multiply => "multiply",
            Node::Divide => "divide",
            Node::Identity => "identity",
        }
        .to_string()
    }

    fn view<'a>(
        &self,
        id: u32,
        input_data: Option<OrderMap<SmolStr, &RefCell<PortData>>>,
    ) -> Element<'a, Message> {
        let text_display = |s: String| -> Element<Message> { text(s).size(30).into() };
        match &self {
            Node::Constant(value) => constant::view(id, *value),
            Node::Linspace { start, stop, num } => linspace::view(id, *start, *stop, *num),
            Node::Plot => plot::view(id, input_data),
            Node::Add => text_display("+".into()),
            Node::Subtract => text_display("-".into()),
            Node::Multiply => text_display("*".into()),
            Node::Divide => text_display("/".into()),
            Node::Identity => text_display("I".to_string()),
        }
    }
}

pub type NetworkNode = GraphNode<Node, PortType, PortData>;
#[derive(Clone, Debug, Default)]
pub enum PortType {
    Integer,
    #[default]
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

// DynClone is used so that we can hold a Box<dyn GUINode>  in `Message`
// It is convenient (maybe required?) to be able to do this so that we
// can have messages that create new nodes, or update node configurations dynamicaly
//
// An alternative would be to have an enum of nodes, but this doesn't generalize well,
// and I'd like to be able keep this logic factored out of this specifc application
pub trait GUINode: derive_more::Debug + dyn_clone::DynClone + Send {
    //TODO make this more understandable. clearer distinction between graph and gui?
    // split out port names, and compute logic?
    fn network_node(self) -> NetworkNode;

    //TODO: Port validation logic? here, or handled at the portType level?
    //TODO: conversion logic?

    fn name(&self) -> String;

    fn view<'a>(
        &self,
        _id: u32,
        _input_data: Option<OrderMap<SmolStr, &RefCell<PortData>>>,
    ) -> Element<'a, Message> {
        text("default").into()
    }
}

// Enables objects holding Box<dyn GUINode> to be able to derive Clone
dyn_clone::clone_trait_object!(GUINode);

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

pub fn available_nodes() -> Vec<Node> {
    vec![identity_node()]
}

pub(crate) fn available_nodes_view<'a>() -> Element<'a, Message> {
    container(
        container(
            column(Node::iter().map(|node| {
                button(
                    row![
                        horizontal_rule(0.0),
                        container(text(node.name())).padding(4.0)
                    ]
                    //.spacing(4.0)
                    .align_y(Center),
                )
                .padding(0.)
                .on_press(Message::AddNode(node))
                .width(Fill)
                .style(style::button::list)
                .into()
            }))
            .width(150.),
        )
        .style(|t| bordered_box(t).background(Color::TRANSPARENT)),
    )
    .center_x(Fill)
    .into()
}
