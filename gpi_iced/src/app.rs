use std::iter::once;

use crate::graph::{Graph, PortRef, IO};
use crate::nodes::constant::constant_node;
use crate::nodes::linspace::linspace_node;
use crate::nodes::plot;
use crate::nodes::{
    format_node_output,
    math_nodes::{add, multiply, Node, Operation, PortData, PortType},
    node_display, NODE_HEIGHT, NODE_RADIUS, NODE_WIDTH, PORT_RADIUS,
};
use crate::widget::custom_button;
use crate::widget::node_container::NodeContainer;
use crate::widget::pin::Pin;
use crate::widget::shapes::ShapeId;
use crate::widget::workspace::{self, workspace};
use crate::wires::{active_wire_color, find_port_offset, wire_status};
use advanced::graphics::core::Element;
use canvas::{Path, Stroke};
use iced::border::{radius, rounded};
use iced::widget::{column, *};
use iced::*;
use ordermap::OrderMap;

#[derive(Default)]
pub enum Action {
    #[default]
    Idle,
    CreatingInputWire(PortRef, Option<PortRef>),
    CreatingOutputWire(PortRef, Option<PortRef>),
    AddingNode,
}

#[derive(Debug, Clone)]
pub enum Message {
    OnDrag(ShapeId, Point),
    OnMove(Point),
    Pan(Vector),
    Config(f32),
    OnSelect(ShapeId),
    PortPress(PortRef),
    PortStartHover(PortRef),
    PortEndHover(PortRef),
    PortRelease,
    UpdateNodeData(u32, Operation),
}

pub struct App {
    graph: Graph<Node, PortType, PortData>,
    shapes: workspace::State,
    selected_shape: Option<ShapeId>,
    cursor_position: Point,
    config: f32,
    theme: Theme,
    action: Action,
}

impl Default for App {
    fn default() -> App {
        let mut g = Graph::<Node, PortType, PortData>::new();

        let l1 = g.node(linspace_node(0., 10., 10));
        let c1 = g.node(constant_node(9.));
        let c2 = g.node(constant_node(13.));
        let mult1 = g.node(multiply());
        let add1 = g.node(add());
        let plot1 = g.node(plot::node());

        g.connect((l1, "out"), (mult1, "a"));
        g.connect((c1, "out"), (mult1, "b"));
        g.connect((mult1, "out"), (add1, "a"));
        g.connect((c2, "out"), (add1, "b"));
        g.connect((l1, "out"), (plot1, "x"));
        g.connect((add1, "out"), (plot1, "y"));
        g.execute_network();

        let shapes = [
            (l1, Point::new(100., 100.)),
            (c1, Point::new(250., 80.)),
            (c2, Point::new(400., 100.)),
            (mult1, Point::new(200., 200.)),
            (add1, Point::new(300., 300.)),
            (plot1, Point::new(200., 400.)),
        ];

        Self {
            graph: g,
            shapes: workspace::State::new(shapes.into()),
            selected_shape: None,
            cursor_position: Default::default(),
            config: 50.,
            theme: Theme::Ferra,
            action: Default::default(),
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Pan(delta) => {
                self.shapes.camera.position.x -= delta.x * 2.;
                self.shapes.camera.position.y -= delta.y * 2.;
            }
            Message::Config(v) => self.config = v,
            Message::OnMove(cursor_position) => self.cursor_position = cursor_position,
            Message::OnSelect(shape_id) => {
                self.selected_shape = Some(shape_id);
                self.graph.exectute_sub_network(shape_id);
            }
            Message::OnDrag(shape_index, cursor_position) => {
                *self
                    .shapes
                    .shape_positions
                    .get_mut(&shape_index)
                    .expect("Shape index must exist") = cursor_position
            }
            Message::PortPress(port) => match port.io {
                IO::In => self.action = Action::CreatingInputWire(port, None),
                IO::Out => self.action = Action::CreatingOutputWire(port, None),
            },
            Message::PortRelease => {
                match &self.action {
                    Action::CreatingInputWire(input, Some(output)) => {
                        self.graph.remove_edge(input);
                        self.graph.add_edge_from_ref(output, input)
                    }
                    Action::CreatingOutputWire(output, Some(input)) => {
                        self.graph.remove_edge(input);
                        self.graph.add_edge_from_ref(output, input)
                    }
                    _ => {}
                }

                self.action = Action::Idle;
            }
            Message::PortStartHover(hover_port) => match &self.action {
                Action::CreatingInputWire(input, _) => {
                    if *input != hover_port {
                        self.action = Action::CreatingInputWire(input.clone(), Some(hover_port))
                    }
                }
                Action::CreatingOutputWire(output, _) => {
                    if *output != hover_port {
                        self.action = Action::CreatingOutputWire(output.clone(), Some(hover_port))
                    }
                }
                _ => {}
            },
            Message::PortEndHover(_port) => match &self.action {
                Action::CreatingInputWire(input, _) => {
                    self.action = Action::CreatingInputWire(input.clone(), None)
                }
                Action::CreatingOutputWire(output, _) => {
                    self.action = Action::CreatingOutputWire(output.clone(), None)
                }
                _ => {}
            },
            Message::UpdateNodeData(id, operation) => {
                let node = self.graph.get_mut_node(id);
                node.data.operation = operation;

                self.graph.exectute_sub_network(id);
            }
        };
    }

    pub fn view(&self) -> Element<Message, Theme, Renderer> {
        const SEPERATOR: f32 = 1.0;

        fn button_style(t: &Theme, s: button::Status) -> button::Style {
            let mut style = button::secondary(t, s);
            style.border.radius = radius(0.);
            style
        }

        let file_commands = row![
            horizontal_space(),
            button(text("New").line_height(0.6))
                .on_press(Message::Config(20.))
                .style(button_style),
            horizontal_space(),
            button(text("Load").line_height(0.6))
                .on_press(Message::Config(40.))
                .style(button_style),
            horizontal_space(),
            button(text("Save").line_height(0.6))
                .on_press(Message::Config(60.))
                .style(button_style),
            horizontal_space(),
        ]
        .spacing(2.0)
        .padding([5., 10.]);

        //// Config
        let config: Element<Message, Theme, Renderer> =
            if let Some(selected_id) = self.selected_shape {
                let node = self.graph.get_node(selected_id);
                let out_port_display = format_node_output(&self.graph.get_output_data(selected_id));

                column![
                    container(text(node.data.full_name.clone()).size(20.)).center_x(Fill),
                    horizontal_rule(0),
                    vertical_space().height(10.),
                    scrollable(out_port_display)
                ]
                .spacing(5.)
                .padding([10., 5.])
                .into()
            } else {
                text("no node selected").into()
            };

        //// Canvas
        let workspace = workspace(
            &self.shapes,
            //// Nodes
            |id| self.node_content(id),
            //// Wires
            |wire_end_node, points| self.wire_curve(wire_end_node, points),
        )
        .on_shape_drag(Message::OnDrag)
        .on_cursor_move(Message::OnMove)
        .on_press(Message::OnSelect)
        .pan(Message::Pan);

        //// View
        row![
            container(
                column![
                    //// File
                    file_commands.align_y(Alignment::Center).width(Length::Fill),
                    ////
                    horizontal_rule(SEPERATOR),
                    //// Config
                    config,
                ]
                .height(Length::Fill)
                .width(250.),
            ),
            vertical_rule(SEPERATOR),
            container(workspace)
                .height(Length::Fill)
                .width(Length::Fill)
        ]
        .into()
    }

    fn node_content(&self, id: u32) -> Element<Message, Theme, Renderer> {
        let node = self.graph.get_node(id);
        let is_selected = match self.selected_shape {
            Some(s_id) => id == s_id,
            None => false,
        };

        fn port_style(t: &Theme, s: custom_button::Status) -> custom_button::Style {
            let mut style = custom_button::primary(t, s);
            style.border.radius = radius(100.);
            style
        }
        let node_style = move |t: &Theme| {
            let outline_color = match is_selected {
                true => t.extended_palette().primary.strong.color,
                false => t.extended_palette().secondary.strong.color,
            };
            container::transparent(t)
                .border(rounded(NODE_RADIUS).color(outline_color).width(2.))
                .background(self.theme.palette().background)
        };

        //TODO: clean up this function, use something similar to wires.rs
        let port_x = |i: usize| i as f32 * (NODE_WIDTH / 4.) + NODE_RADIUS * 2.;
        //// Ports
        let port_buttons = {
            let in_port_buttons = node
                .inputs
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
            let out_port_buttons = node
                .outputs
                .iter()
                .enumerate()
                .map(|(i, port)| (Point::new(port_x(i), NODE_HEIGHT - PORT_RADIUS), port))
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
        let (node_display, node_size) = node_display(node, id, input_data);

        //// Node
        let node_inner: Element<Message, Theme, Renderer> =
            container(center(node_display)).style(node_style).into();

        let content: Element<Message, Theme, Renderer> = NodeContainer::new(
            //node_inner.explain(Color::from_rgba(0.7, 0.7, 0.8, 0.2)),
            node_inner,
            port_buttons.collect(),
        )
        .width(node_size.width)
        .height(node_size.height)
        .into();
        content
    }

    fn wire_curve(&self, wire_end_node: u32, points: &OrderMap<u32, Point>) -> Vec<(Path, Stroke)> {
        let port_position = |port: &PortRef| {
            points[&port.node] + find_port_offset(port, self.graph.port_index(port))
        };

        //// Handle currently active wire
        // TODO: account for wire postion that are not the first
        let active_wire = match &self.action {
            Action::CreatingInputWire(input, Some(tentative_output)) => {
                Some((port_position(input), port_position(tentative_output)))
            }
            Action::CreatingInputWire(input, None) => Some((
                port_position(input),
                self.cursor_position + self.shapes.camera.position,
            )),
            Action::CreatingOutputWire(output, Some(input)) => {
                Some((port_position(input), port_position(output)))
            }
            Action::CreatingOutputWire(output, None) => Some((
                self.cursor_position + self.shapes.camera.position,
                port_position(output),
            )),
            _ => None,
        };

        //// Handle all wires
        let incoming_wires = self.graph.incoming_edges(&wire_end_node);
        incoming_wires
            .iter()
            .map(|(from, to)| {
                let color = wire_status(from, to, &self.action, &self.theme);
                ((port_position(to), port_position(from)), color)
            })
            //// include the active wire
            .chain(once(active_wire.map(|w| (w, active_wire_color(&self.theme)))).flatten())
            //// build the wire curves
            .map(|((from, to), color)| {
                (
                    Path::new(|builder| {
                        builder.move_to(from);
                        let mid = f32::abs((to.y - from.y) * 0.5).max(PORT_RADIUS * 2.);
                        builder.bezier_curve_to(
                            (from.x, from.y - mid).into(),
                            (to.x, to.y + mid).into(),
                            to,
                        );
                    }),
                    Stroke::default()
                        .with_width(3.0)
                        .with_color(color)
                        .with_line_cap(canvas::LineCap::Round),
                )
            })
            .collect()
    }
}
