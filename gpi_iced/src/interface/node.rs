use crate::app::{App, Message};
use crate::graph::{GraphNode, PortRef, IO};
use crate::math::Point;
use crate::nodes::{
    GUINode, INNER_NODE_HEIGHT, INNER_NODE_WIDTH, NODE_BORDER_WIDTH, NODE_RADIUS, PORT_RADIUS,
};
use crate::widget::custom_button;
use crate::widget::node_container::NodeContainer;
use crate::widget::pin::Pin;
use iced::{border, widget::*, Color, Element};

impl App {
    pub fn node_content(&self, id: u32) -> Element<Message, Theme, Renderer> {
        let node = self.graph.get_node(id);
        let is_selected = match self.selected_shape {
            Some(s_id) => id == s_id,
            None => false,
        };

        fn port_style(t: &Theme, s: custom_button::Status) -> custom_button::Style {
            let mut style = custom_button::primary(t, s);
            style.border.radius = border::radius(100.);
            style
        }
        let node_style = move |t: &Theme| {
            let outline_color = match is_selected {
                true => t.extended_palette().primary.strong.color,
                false => t.extended_palette().secondary.strong.color,
            };
            container::transparent(t)
                .border(
                    border::rounded(NODE_RADIUS)
                        .color(outline_color)
                        .width(NODE_BORDER_WIDTH),
                )
                .background(self.theme.palette().background)
        };

        //TODO: clean up this function, use something similar to wires.rs
        let port_x = |i: usize| i as f32 * (INNER_NODE_WIDTH / 4.) + NODE_RADIUS * 2.;

        //// Ports
        let inputs = node.inputs();
        let outputs = node.outputs();

        let port_buttons = {
            let in_port_buttons = inputs
                .iter()
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
                                .on_right_press(Message::PortDelete(in_port.clone()))
                                .on_release_self(Message::PortRelease)
                                .style(port_style)
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
                .iter()
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
                                .on_right_press(Message::PortDelete(out_port.clone()))
                                .on_release_self(Message::PortRelease)
                                .style(port_style)
                                .width(PORT_RADIUS * 2.)
                                .height(PORT_RADIUS * 2.),
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
        let (node_size, node_view) = node.view(id, input_data);

        //// Node
        let node_inner: Element<Message, Theme, Renderer> = container(node_view)
            .style(node_style)
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
