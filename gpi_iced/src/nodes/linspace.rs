use iced::{
    widget::{
        column, container, horizontal_rule, horizontal_space, row, text, text_input, TextInput,
    },
    Alignment::Center,
    Color, Element,
    Length::{Fill, Shrink},
};

use crate::{app::Message, graph::GraphNode, nodes::math_nodes::Operation};

use super::{
    math_nodes::{Node, PortData, PortType},
    NetworkNode,
};

pub fn linspace_node(start: f64, stop: f64, num: i64) -> NetworkNode {
    GraphNode::new(
        //initial node
        Node {
            short_name: "Linspace".to_string(),
            full_name: "Linspace".to_string(),
            operation: Operation::Linspace { start, stop, num },
        },
        vec![],
        vec![("out", &PortType::Real)],
        Box::new(move |_, node_data| {
            //node after potential modifications
            if let Operation::Linspace { start, stop, num } = node_data.operation {
                let data: Vec<_> = (0..=num)
                    .map(|i| (i as f64 / num as f64))
                    .map(|c| start + (1. - c) * stop)
                    .collect();

                [("out".into(), PortData::Real(data.clone().into()))].into()
            } else {
                panic!("Linspace Operation is invalid {:?}", node_data)
            }
        }),
    )
}

pub fn view<'a>(id: u32, start: f64, stop: f64, num: i64) -> Element<'a, Message> {
    fn numeric_input(input: TextInput<Message>) -> Element<Message> {
        column![
            input
                .padding(0)
                .style(|t, s| {
                    let d = text_input::default(t, s);
                    text_input::Style {
                        border: d.border.color(Color::TRANSPARENT),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..d
                    }
                })
                .align_x(Center),
            container(horizontal_rule(0)).width(20.).height(1),
        ]
        .height(Shrink)
        .width(30.)
        .align_x(Center)
        .into()
    }
    let start_input = numeric_input(text_input("0", &start.to_string()).on_input(move |value| {
        Message::UpdateNodeData(
            id,
            Operation::Linspace {
                start: value.parse().unwrap_or(0.),
                stop,
                num,
            },
        )
    }));
    let stop_input = numeric_input(text_input("10", &stop.to_string()).on_input(move |value| {
        Message::UpdateNodeData(
            id,
            Operation::Linspace {
                start,
                stop: value.parse().unwrap_or(0.),
                num,
            },
        )
    }));
    let num_input = numeric_input(text_input("100", &num.to_string()).on_input(move |value| {
        Message::UpdateNodeData(
            id,
            Operation::Linspace {
                start,
                stop,
                num: value.parse().unwrap_or(0),
            },
        )
    }));

    column![
        row![
            horizontal_space(),
            start_input,
            text(".."),
            stop_input,
            horizontal_space()
        ]
        .width(Fill)
        .align_y(Center)
        .padding(5.)
        .spacing(2.),
        row![horizontal_space(), text("#"), num_input, horizontal_space()].align_y(Center)
    ]
    .into()
}
