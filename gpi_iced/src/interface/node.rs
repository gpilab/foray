use std::f32::consts::PI;
use std::time::Instant;

use crate::app::{App, Message};
use crate::graph::{GraphNode, PortRef, IO};
use crate::gui_node::{GUINode, PortDataContainer};
use crate::math::Point;
use crate::nodes::port::PortType;
use crate::nodes::status::NodeStatus;
use crate::nodes::NodeData;
use crate::widget::custom_button;
use crate::widget::node_container::NodeContainer;
use crate::widget::pin::Pin;
use crate::StableMap;
use iced::{
    border, color,
    widget::{column, *},
    Color, Element,
};

pub const INNER_NODE_WIDTH: f32 = 120.;
pub const INNER_NODE_HEIGHT: f32 = 60.;
pub const PORT_RADIUS: f32 = 8.5;
pub const NODE_RADIUS: f32 = 5.0;
pub const NODE_BORDER_WIDTH: f32 = 2.0;
pub const OUTER_NODE_WIDTH: f32 = INNER_NODE_WIDTH + NODE_BORDER_WIDTH;
pub const OUTER_NODE_HEIGHT: f32 = INNER_NODE_HEIGHT + NODE_BORDER_WIDTH;

pub fn default_node_size() -> iced::Size {
    iced::Size::new(OUTER_NODE_WIDTH, OUTER_NODE_HEIGHT)
}

impl App {
    pub fn node_content(&self, id: u32) -> Element<Message, Theme, Renderer> {
        let node = self.graph.get_node(id);
        let is_selected = self.selected_shapes.contains(&id);

        let port_style =
            move |port_type: &PortType, s: &custom_button::Status| -> custom_button::Style {
                let color_pair = match port_type {
                    //TODO: put these colors into AppTheme
                    PortType::Integer => (color!(209, 77, 65), color!(175, 48, 41)), //red
                    PortType::Real => (
                        self.app_theme.primary.weak_color().into(),
                        self.app_theme.primary.base_color.into(),
                    ),
                    PortType::Complex => (color!(135, 154, 57), color!(102, 128, 11)), //green
                    PortType::ArrayInteger => (color!(209, 77, 65), color!(175, 48, 41)), //red
                    PortType::ArrayReal => (color!(67, 133, 190), color!(32, 94, 166)), //blue
                    PortType::ArrayComplex => (color!(58, 169, 159), color!(36, 131, 123)), //cyan
                    PortType::Dynamic => (color!(209, 150, 65), color!(175, 125, 41)), //orange
                    PortType::Object(_) => (color!(229, 180, 65), color!(200, 160, 41)), //yellow
                };

                let mut style = custom_button::custom(*s, color_pair.1, color_pair.0);
                style.border.radius = border::radius(100.);
                style
            };
        let node_style = move |node: &NodeData, t: &Theme| {
            let color = match &node.status {
                NodeStatus::Idle | NodeStatus::Running(_) => match is_selected {
                    true => t.extended_palette().primary.strong.color,
                    false => t.extended_palette().secondary.strong.color,
                },
                NodeStatus::Error(_node_error) => match is_selected {
                    true => t.extended_palette().danger.base.color,
                    false => t.extended_palette().danger.weak.color,
                },
            };
            let run_time = match &node.status {
                NodeStatus::Running(start_inst) => (Instant::now() - *start_inst).as_secs_f32(),
                _ => 0.0,
            };

            let pulse_freq = 1.0;
            let pulse = color.scale_alpha(
                ((run_time  // t
                        * pulse_freq * 2.0 * PI // f 
                    ).cos() // start at 1.0
                    + 1.0 // shift range 0.0-2.0
                ) * 0.5 // scale range 0.0-1.0
                    * 0.75  // scale range 0.0-0.75
                    + 0.25, // shift range 0.25-1.0
            );

            container::transparent(t)
                .border(
                    border::rounded(NODE_RADIUS)
                        .color(pulse)
                        .width(NODE_BORDER_WIDTH),
                )
                .background(iced::Color::from(self.app_theme.background.base_color))
        };

        //TODO: clean up this function, use something similar to wires.rs
        let port_x = |i: usize| i as f32 * (INNER_NODE_WIDTH / 4.) + NODE_RADIUS * 2.;

        //// Ports
        let inputs = node.inputs();
        let outputs = node.outputs();

        let port_buttons = {
            let in_port_buttons = inputs
                .into_iter()
                .enumerate()
                .map(|(i, port)| (Point::new(port_x(i), -PORT_RADIUS), port))
                .map(|(point, port)| {
                    let in_port = PortRef {
                        node: id,
                        name: port.0.clone(),
                        io: IO::In,
                    };
                    Pin::new(
                        mouse_area(
                            custom_button::Button::new("")
                                .on_press(Message::PortPress(in_port.clone()))
                                .on_drag(Message::OnMove)
                                .on_right_press(Message::PortDelete(in_port.clone()))
                                .on_release_self(Message::PortRelease)
                                .style(move |_t, s| port_style(&port.1, &s))
                                .width(PORT_RADIUS * 2.)
                                .height(PORT_RADIUS * 2.),
                        )
                        .on_enter(Message::PortStartHover(in_port.clone()))
                        .on_exit(Message::PortEndHover(in_port.clone())),
                    )
                    .position(point)
                    .into()
                });
            let out_port_buttons = outputs
                .into_iter()
                .enumerate()
                .map(|(i, port)| (Point::new(port_x(i), INNER_NODE_HEIGHT - PORT_RADIUS), port))
                .map(|(point, port)| {
                    let out_port = PortRef {
                        node: id,
                        name: port.0.clone(),
                        io: IO::Out,
                    };

                    Pin::new(
                        mouse_area(
                            custom_button::Button::new(vertical_space())
                                .on_press(Message::PortPress(out_port.clone()))
                                .on_drag(Message::OnMove)
                                .on_right_press(Message::PortDelete(out_port.clone()))
                                .on_release_self(Message::PortRelease)
                                .style(move |_t, s| port_style(&port.1, &s))
                                .width(PORT_RADIUS * 2.)
                                .height(PORT_RADIUS * 2.)
                                .padding(2.0),
                        )
                        .on_enter(Message::PortStartHover(out_port.clone()))
                        .on_exit(Message::PortEndHover(out_port.clone())),
                    )
                    .position(point)
                    .into()
                });
            in_port_buttons.chain(out_port_buttons)
        };

        let input_data = self.graph.get_input_data(&id);
        let (node_size, node_view) = node.template.view(id, input_data);

        //// Node
        let node_inner: Element<Message, Theme, Renderer> = container(node_view)
            .style(move |theme| node_style(node, theme))
            .center_x(node_size.width)
            .center_y(node_size.height)
            .into();

        let content: Element<Message, Theme, Renderer> = NodeContainer::new(
            if self.debug {
                node_inner.explain(Color::from_rgba(0.7, 0.7, 0.8, 0.2))
            } else {
                node_inner
            },
            port_buttons.collect(),
        )
        .width(node_size.width)
        .height(node_size.height)
        .into();
        content
    }
}

pub fn format_node_output<'a>(
    node: &NodeData,
    data: &StableMap<String, Option<&PortDataContainer>>,
) -> Element<'a, Message> {
    //TODO: clean this up by iterating straight to text elements?
    let node_output = data.iter().map(|(port_name, d)| {
        (
            port_name.to_string(),
            d.map(|d| format!("{}", d.read().unwrap()))
                .unwrap_or("n/a".to_string()),
        )
    });

    container(column![
        text(format!("{:#?}", node)).size(12.),
        column(node_output.map(|(lbl, val)| {
            row![text(lbl).size(12.), text(val).size(12.)]
                .spacing(5.0)
                .into()
        }))
    ])
    .into()
}
