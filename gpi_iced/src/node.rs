use iced::{
    widget::{
        column, container, horizontal_rule, horizontal_space, row, slider, text, text_input,
        TextInput,
    },
    Alignment::{self, Center},
    Color, Element,
    Length::{self, Fill, Shrink},
};
use ordermap::OrderMap;
use smol_str::SmolStr;
use std::cell::RefCell;

use crate::{
    app::Message,
    graph::GraphNode,
    math_nodes::{Node, Operation, PortData, PortType},
};

pub const NODE_WIDTH: f32 = 100.;
pub const NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 7.5;
pub const NODE_RADIUS: f32 = 5.0;

pub fn node_display(node: &GraphNode<Node, PortType, PortData>, id: u32) -> Element<Message> {
    match &node.data.operation {
        Operation::Constant(value) => container(
            row![
                text(value),
                slider(-100.0..=100., *value, move |value| {
                    Message::UpdateNodeData(id, Operation::Constant(value))
                })
                .width(40.),
            ]
            .align_y(Center)
            .spacing(10.)
            .padding([0., 10.]),
        )
        .center_y(Length::Fill)
        .align_right(Length::Fill)
        .into(),
        Operation::Linspace { start, stop, num } => {
            let (start, stop, num) = (*dbg!(start), *dbg!(stop), *dbg!(num));

            fn numeric_input(input: TextInput<Message>) -> Element<Message> {
                column![
                    input
                        .padding(0)
                        //.size(12)
                        .style(|t, s| {
                            let d = text_input::default(t, s);
                            text_input::Style {
                                border: d.border.color(Color::TRANSPARENT),
                                background: iced::Background::Color(Color::TRANSPARENT),
                                ..d
                            }
                        })
                        .align_x(Alignment::Center),
                    container(horizontal_rule(0)).width(20.).height(1),
                ]
                .height(Shrink)
                .width(30.)
                .align_x(Alignment::Center)
                .into()
            }
            let start_input =
                numeric_input(text_input("0", &start.to_string()).on_input(move |value| {
                    Message::UpdateNodeData(
                        id,
                        Operation::Linspace {
                            start: value.parse().unwrap_or(0.),
                            stop,
                            num,
                        },
                    )
                }));
            let stop_input =
                numeric_input(text_input("10", &stop.to_string()).on_input(move |value| {
                    Message::UpdateNodeData(
                        id,
                        Operation::Linspace {
                            start,
                            stop: value.parse().unwrap_or(0.),
                            num,
                        },
                    )
                }));
            let num_input =
                numeric_input(text_input("100", &num.to_string()).on_input(move |value| {
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
            //.width(Fill)
            //.align_x(Center)
            //.padding(3.)
            //.spacing(3.)
            .into()
        }

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
