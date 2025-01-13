pub mod constant;
pub mod linspace;
pub mod math_nodes;
pub mod plot;
pub mod plot_complex;
pub mod port;

pub use port::PortData;
pub use port::PortType;

use crate::OrderMap;
use crate::{app::Message, node_data::NodeData, style};
use iced::{
    widget::{button, column, container, container::bordered_box, horizontal_rule, row, text},
    Alignment::Center,
    Color, Element,
    Length::Fill,
};
use std::cell::RefCell;
use strum::IntoEnumIterator;
pub const INNER_NODE_WIDTH: f32 = 120.;
pub const INNER_NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 7.5;
pub const NODE_RADIUS: f32 = 5.0;
pub const NODE_BORDER_WIDTH: f32 = 2.0;
pub const OUTER_NODE_WIDTH: f32 = INNER_NODE_WIDTH + NODE_BORDER_WIDTH;
pub const OUTER_NODE_HEIGHT: f32 = INNER_NODE_HEIGHT + NODE_BORDER_WIDTH;

pub fn default_node_size() -> iced::Size {
    iced::Size::new(OUTER_NODE_WIDTH, OUTER_NODE_HEIGHT)
}

pub trait GUINode: derive_more::Debug {
    //TODO make this more understandable. clearer distinction between graph and gui?
    // split out port names, and compute logic?
    //fn network_node(&self) -> GraphNode<PortType, PortData>;

    //TODO: Port validation logic? here, or handled at the portType level?
    //TODO: conversion logic?

    fn name(&self) -> String;

    fn view<'a>(
        &'a self,
        _id: u32,
        _input_data: Option<OrderMap<String, &RefCell<PortData>>>,
    ) -> (iced::Size, Element<'a, Message>) {
        (default_node_size(), text("default").into())
    }

    fn config_view<'a>(
        &'a self,
        _id: u32,
        _input_data: Option<OrderMap<String, &RefCell<PortData>>>,
    ) -> Option<Element<'a, Message>> {
        None
    }

    fn templates(&self) -> Vec<Self>
    where
        Self: Sized + Clone,
    {
        vec![self.clone()]
    }
}

pub fn format_node_output<'a>(
    data: &OrderMap<String, Option<&RefCell<PortData>>>,
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

pub(crate) fn available_nodes() -> Vec<NodeData> {
    NodeData::iter().flat_map(|node| node.templates()).collect()
}

pub(crate) fn node_list_view<'a>(nodes: &[NodeData]) -> Element<'a, Message> {
    container(
        container(
            //TODO: don't create new nodes on every view. store a list of Node templates in App
            // New nodes are expensive for python nodes which need to read their source
            column(nodes.iter().map(|node| {
                button(
                    row![
                        horizontal_rule(0.0),
                        container(text(node.name())).padding(4.0)
                    ]
                    //.spacing(4.0)
                    .align_y(Center),
                )
                .padding(0.)
                .on_press(Message::AddNode(node.clone()))
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
