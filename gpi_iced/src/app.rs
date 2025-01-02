use std::iter::once;

use crate::graph::{Graph, GraphNode, PortRef, IO2};
use crate::math::{Point, Vector};
use crate::node_data::NodeData;
use crate::nodes::linspace::LinspaceConfig;
use crate::nodes::plot::Plot;
use crate::nodes::{self, GUINode, PortData, PortType, NODE_BORDER_WIDTH};
use crate::nodes::{
    format_node_output, INNER_NODE_HEIGHT, INNER_NODE_WIDTH, NODE_RADIUS, PORT_RADIUS,
};
use crate::widget::custom_button;
use crate::widget::node_container::NodeContainer;
use crate::widget::pin::Pin;
use crate::widget::shapes::ShapeId;
use crate::widget::workspace::{self, workspace};
use crate::wires::{active_wire_color, find_port_offset, wire_status};
use canvas::{Path, Stroke};
use iced::advanced::graphics::core::Element;
use iced::border::{radius, rounded};
use iced::widget::{column, *};
use iced::Length::Fill;
use iced::{Alignment, Color};
use ordermap::OrderMap;

#[derive(Default)]
pub enum Action {
    #[default]
    Idle,
    CreatingInputWire(PortRef, Option<PortRef>),
    CreatingOutputWire(PortRef, Option<PortRef>),
    AddingNode,
}

#[derive(Clone, Debug)]
pub enum Message {
    OnDrag(ShapeId, Point),
    OnMove(Point),
    Pan(Vector),
    Config(f32),
    OnSelect(Option<ShapeId>),
    PortPress(PortRef),
    PortStartHover(PortRef),
    PortEndHover(PortRef),
    PortRelease,
    UpdateNodeData(u32, NodeData),
    AddNode(NodeData),
    DeleteNode(u32),
    ToggleDebug,
}
pub struct App {
    graph: Graph<NodeData, PortType, PortData>,
    shapes: workspace::State,
    selected_shape: Option<ShapeId>,
    cursor_position: Point,
    config: f32,
    //#[serde(skip_serializing)]
    theme: Theme,
    action: Action,
    debug: bool,
}

impl Default for App {
    fn default() -> App {
        let mut g = Graph::<NodeData, PortType, PortData>::new();

        let l1 = g.node(NodeData::Linspace(LinspaceConfig::default()));
        let c1 = g.node(NodeData::Constant(0.5));
        let c2 = g.node(NodeData::Constant(-2.));
        let mult1 = g.node(NodeData::Multiply);
        let add1 = g.node(NodeData::Add);
        let plot1 = g.node(NodeData::Plot(Plot::default()));
        let identity = g.node(NodeData::Identity);

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
            (identity, Point::new(100., 300.)),
        ];

        Self {
            graph: g,
            shapes: workspace::State::new(shapes.into()),
            selected_shape: None,
            cursor_position: Default::default(),
            config: 50.,
            theme: Theme::Ferra,
            action: Default::default(),
            debug: false,
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
            Message::OnSelect(maybe_id) => {
                self.selected_shape = maybe_id;
                if let Some(shape_id) = maybe_id {
                    self.graph.exectute_sub_network(shape_id);
                }
            }
            Message::OnDrag(shape_index, cursor_position) => {
                *self
                    .shapes
                    .shape_positions
                    .get_mut(&shape_index)
                    .expect("Shape index must exist") = cursor_position
            }
            Message::PortPress(port) => match port.io {
                IO2::In2 => self.action = Action::CreatingInputWire(port, None),
                IO2::Out2 => self.action = Action::CreatingOutputWire(port, None),
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
            Message::UpdateNodeData(id, node) => {
                let graph_node = self.graph.get_mut_node(id);
                *graph_node = node;

                self.graph.exectute_sub_network(id);
            }
            Message::AddNode(node) => {
                let id = self.graph.node(node);
                self.shapes.shape_positions.insert(id, (100., 500.).into());
            }
            Message::DeleteNode(id) => {
                self.graph.delete_node(id);
                self.shapes.shape_positions.remove(&id);
                self.selected_shape = None;
            }
            Message::ToggleDebug => {
                self.debug = !self.debug;
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
            button(text("New"))
                .on_press(Message::Config(20.))
                .padding([1.0, 4.0])
                .style(button_style),
            horizontal_space(),
            button(text("Load"))
                .on_press(Message::Config(40.))
                .padding([1.0, 4.0])
                .style(button_style),
            horizontal_space(),
            button(text("Save"))
                .on_press(Message::Config(60.))
                .padding([1.0, 4.0])
                .style(button_style),
            horizontal_space(),
            button(text("Dbg"))
                .padding([1.0, 4.0])
                .on_press(Message::ToggleDebug)
                .style(button_style),
        ]
        .spacing(2.0)
        .padding([5., 5.]);

        //// Config
        let config: Element<Message, Theme, Renderer> =
            if let Some(selected_id) = self.selected_shape {
                let node = self.graph.get_node(selected_id);
                let input_data = self.graph.get_input_data(&selected_id);
                let out_port_display = format_node_output(&self.graph.get_output_data(selected_id));
                column![
                    container(text(node.name().clone()).size(20.)).center_x(Fill),
                    horizontal_rule(0),
                    vertical_space().height(10.),
                    node.config_view(selected_id, input_data)
                        .unwrap_or(text("...").into()),
                    vertical_space(),
                    scrollable(if self.debug {
                        out_port_display
                    } else {
                        text("").into()
                    }),
                    row![button("delete node").on_press(Message::DeleteNode(selected_id))]
                ]
                .height(Fill)
                .spacing(5.)
                .padding([10., 5.])
                .into()
            } else {
                let node_list = nodes::available_nodes_view();
                column![
                    container(text("Add Node").size(20.)).center_x(Fill),
                    horizontal_rule(0),
                    vertical_space().height(10.),
                    scrollable(node_list)
                ]
                .spacing(5.)
                .padding([10., 5.])
                .into()
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
                    file_commands.align_y(Alignment::Center).width(Fill),
                    ////
                    horizontal_rule(SEPERATOR),
                    //// Config
                    if self.debug {
                        config.explain(Color::from_rgba(0.7, 0.7, 0.8, 0.2))
                    } else {
                        config
                    }
                ]
                .height(Fill)
                .width(200.),
            ),
            vertical_rule(SEPERATOR),
            container(workspace).height(Fill).width(Fill)
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
                .border(
                    rounded(NODE_RADIUS)
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
                        io: IO2::In2,
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
            let out_port_buttons = outputs
                .iter()
                .enumerate()
                .map(|(i, port)| (Point::new(port_x(i), INNER_NODE_HEIGHT - PORT_RADIUS), port))
                .map(|(point, port)| {
                    let out_port = PortRef {
                        node: id,
                        name: port.0.clone(),
                        io: IO2::Out2,
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

    fn wire_curve(&self, wire_end_node: u32, points: &OrderMap<u32, Point>) -> Vec<(Path, Stroke)> {
        let port_position = |port: &PortRef| {
            points[&port.node] + find_port_offset(port, self.graph.port_index(port)).into()
        };

        //// Handle currently active wire
        // TODO: test nodes with multiple out ports
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
                        builder.move_to(from.into());
                        let mid = f32::abs((to.y - from.y) * 0.5).max(PORT_RADIUS * 2.);
                        builder.bezier_curve_to(
                            (from.x, from.y - mid).into(),
                            (to.x, to.y + mid).into(),
                            to.into(),
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
