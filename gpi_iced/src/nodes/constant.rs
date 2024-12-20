use iced::{
    widget::{container, row, slider, text},
    Alignment::Center,
    Element,
    Length::Fill,
};

use crate::{app::Message, graph::GraphNode};

use super::math_nodes::Operation;
use super::{
    math_nodes::{Node, PortData, PortType},
    NetworkNode,
};

pub fn constant_node(value: f64) -> NetworkNode {
    GraphNode::new(
        Node {
            short_name: "const".to_string(),
            full_name: "Constant".to_string(),
            operation: Operation::Constant(value),
        },
        vec![],
        vec![("out", &PortType::Real)],
        Box::new(move |_, node_data| {
            //TODO: make a node trait that implements compute, so we have a guarenteed Operation
            //type? Operation would be a type on the trait rather than in an enum
            if let Operation::Constant(value) = node_data.operation {
                [("out".into(), PortData::Real(vec![value].into()))].into()
            } else {
                panic!("Constant Operation is invalid {:?}", node_data)
            }
        }),
    )
}
pub fn view<'a>(id: u32, value: f64) -> Element<'a, Message> {
    container(
        row![
            text(value),
            slider(-100.0..=100., value, move |value| {
                Message::UpdateNodeData(id, Operation::Constant(value))
            })
            .width(40.),
        ]
        .align_y(Center)
        .spacing(10.)
        .padding([0., 10.]),
    )
    .center_y(Fill)
    .align_right(Fill)
    .into()
}
