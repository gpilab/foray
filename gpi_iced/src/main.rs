use std::iter::once;

use canvas::{Path, Stroke};
use gpi_iced::graph::{constant_node, identity_node, Graph, PortRef, IO};
use gpi_iced::widget::custom_button;
use gpi_iced::widget::node_container::NodeContainer;
use gpi_iced::widget::pin::Pin;
use gpi_iced::widget::shapes::ShapeId;
use gpi_iced::widget::workspace::{self, workspace};
use iced::border::{radius, rounded};
use iced::widget::{column, *};
use iced::Element;
use iced::*;
//use ndarray::{ArrayD, ArrayViewD, IxDyn};
const NODE_WIDTH: f32 = 100.;
const NODE_HEIGHT: f32 = 60.;

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    application("gpi_v2", Example::update, Example::view)
        .antialiasing(true)
        .theme(theme)
        .window_size((800., 600.))
        .decorations(true)
        .run()
}

fn theme(_state: &Example) -> Theme {
    Theme::Ferra
}

#[derive(Debug)]
struct Node {
    name: String,
}

#[derive(Clone, Debug)]
enum PortType {
    Integer,
    _Real,
    _Complex,
}

#[derive(Default)]
enum Action {
    #[default]
    Idle,
    CreatingWire(PortRef, Option<PortRef>),
}

#[derive(Debug, Clone)]
enum Message {
    OnDrag(ShapeId, Point),
    OnMove(Point),
    Pan(Vector),
    Config(f32),
    OnSelect(ShapeId),
    PortPress(PortRef),
    PortStartHover(PortRef),
    PortEndHover(PortRef),
    PortRelease,
}

struct Example {
    graph: Graph<Node, PortType, u32>,
    shapes: workspace::State,
    selected_shape: Option<ShapeId>,
    cursor_position: Point,
    config: f32,
    theme: Theme,
    action: Action,
}

impl Default for Example {
    fn default() -> Example {
        let points = [
            Point::new(300., 100.),
            Point::new(300., 200.),
            Point::new(150., 300.),
            Point::new(400., 300.),
            Point::new(100., 400.),
            Point::new(250., 400.),
        ];

        let constant_node =
            |name: &str, c: u32| constant_node(Node { name: name.into() }, c, &PortType::Integer);
        let identity_node =
            |name: &str| identity_node(Node { name: name.into() }, &PortType::Integer);

        let initial_nodes = vec![
            constant_node("a", 7),
            identity_node("b"),
            identity_node("c"),
            identity_node("d"),
            identity_node("e"),
            identity_node("f"),
        ];

        let mut g = Graph::<Node, PortType, u32>::new();
        initial_nodes.into_iter().for_each(|n| {
            g.add_node(n);
        });
        g.add_edge((0, "out"), (1, "in"));
        g.add_edge((1, "out"), (2, "in"));
        g.add_edge((1, "out"), (3, "in"));
        g.add_edge((2, "out"), (4, "in"));
        g.add_edge((3, "out"), (5, "in"));

        let nodes_refs = g.nodes_ref();
        let nr = nodes_refs
            .iter()
            .zip(points.iter())
            .map(|(k, p)| (*k, *p))
            .collect();

        Self {
            graph: g,
            shapes: workspace::State::new(nr),
            selected_shape: None,
            cursor_position: Default::default(),
            config: 50.,
            theme: Theme::Ferra,
            action: Default::default(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) {
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
            Message::PortPress(port_id) => self.action = Action::CreatingWire(port_id, None),
            //TODO: handle connecting nodes
            Message::PortRelease => {
                dbg!("port_drop");
                if let Action::CreatingWire(from, Some(to)) = &self.action {
                    let (in_port, out_port) = match (from.io.clone(), to.io.clone()) {
                        (IO::Out, IO::In) => (to, from),
                        (IO::In, IO::Out) => (from, to),
                        _ => panic!("attempted invalid port connection"),
                    };
                    self.graph.add_edge(
                        (out_port.node, out_port.name.clone()),
                        (in_port.node, in_port.name.clone()),
                    );
                    //TODO: remove duplicate port
                    //self.graph.remove_edge();
                }
                self.action = Action::Idle;
            }
            Message::PortStartHover(port_id) => {
                if let Action::CreatingWire(from_port, _) = &self.action {
                    //TODO: do  a better check for valid connection
                    if *from_port != port_id {
                        self.action = Action::CreatingWire(from_port.clone(), Some(port_id))
                    }
                }
            }
            Message::PortEndHover(_port_id) => {
                if let Action::CreatingWire(from_port, Some(_tentative_connection)) = &self.action {
                    //TODO: do  a better check for valid connection
                    //if *from_port != port_id {
                    self.action = Action::CreatingWire(from_port.clone(), None)
                    //}
                }
            }
        };
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        const SEPERATOR: f32 = 1.0;

        fn button_style(t: &Theme, s: button::Status) -> button::Style {
            let mut style = button::secondary(t, s);
            style.border.radius = radius(0.);
            style
        }
        fn port_style(t: &Theme, s: custom_button::Status) -> custom_button::Style {
            let mut style = custom_button::primary(t, s);
            style.border.radius = radius(100.);
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

        let config = column![
            vertical_space().height(20.),
            row!["Label_1", slider(0.0..=100.0, self.config, Message::Config)]
                .spacing(20.)
                .align_y(Alignment::Center),
            row!["Label_2", slider(0.0..=100.0, self.config, Message::Config)]
                .spacing(20.)
                .align_y(Alignment::Center),
            row!["Label_3", slider(0.0..=100.0, self.config, Message::Config)]
                .spacing(20.)
                .align_y(Alignment::Center),
            vertical_space(),
        ]
        .spacing(5.0)
        .padding(5.);

        const PORT_RADIUS: f32 = 7.5;
        const NODE_RADIUS: f32 = 5.0;
        let port_x = |i: usize| i as f32 * (NODE_WIDTH / 4.) + NODE_RADIUS * 2.;

        let workspace = workspace(
            &self.shapes,
            |id| {
                let node = self.graph.get_node(id);
                let is_selected = match self.selected_shape {
                    Some(s_id) => id == s_id,
                    None => false,
                };

                let name = node.data.name.clone();

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
                                    custom_button::Button::new("")
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

                //// Node
                let content: Element<Message, Theme, Renderer> = NodeContainer::new(
                    container(text(name))
                        .style(move |t: &Theme| {
                            let outline_color = match is_selected {
                                true => t.extended_palette().primary.strong.color,
                                false => t.extended_palette().secondary.strong.color,
                            };
                            container::transparent(t)
                                .border(rounded(NODE_RADIUS).color(outline_color).width(2.))
                                .background(self.theme.palette().background)
                        })
                        .padding(5.)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill),
                    port_buttons.collect(),
                )
                .width(NODE_WIDTH)
                .height(NODE_HEIGHT)
                .into();
                content //.explain(Color::WHITE)
            },
            //// Wires
            |wire_end_node, points| {
                let from_offset = Vector::new(PORT_RADIUS, PORT_RADIUS / 2.);
                let to_offset = Vector::new(PORT_RADIUS, -PORT_RADIUS / 2.);
                let find_port_offset = |port_id: &PortRef| -> Point {
                       match port_id.io {
                           IO::In => {
                               let i = &self
                                   .graph
                                   .get_node(port_id.node)
                                   .inputs
                                   .iter()
                                   .position(|n| *n.0 == *port_id.name)
                                   .unwrap_or_else(|| {
                                       panic!(
                                           "PortId must have valid input node index and port id {port_id:?}",
                                       )
                                   });
                               points[&port_id.node] + Vector::new(port_x(*i), 0.) + from_offset
                           }
                           IO::Out => {
                               let i = &self
                                   .graph
                                   .get_node(wire_end_node)
                                   .outputs
                                   .iter()
                                   .position(|n| *n.0 == *port_id.name)
                                   .unwrap_or_else(|| {
                                       panic!(
                                           "PortId must have valid output node index and port id {port_id:?}",
                                       )
                                   });
                               points[&port_id.node]
                                   + Vector::new(port_x(*i), NODE_HEIGHT)
                                   + to_offset
                           }
                       }
                   };

                   let new_connection_proposed = matches!(&self.action, Action::CreatingWire(_, Some(_tentative)));


                   let creating_active_wire = match &self.action {
                       //TODO: account for wire postion that are not the first
                       Action::CreatingWire(start_port, tentative_end) => {

                           let end_pos = match tentative_end {
                               Some(end_port) => find_port_offset(end_port),
                               None => self.cursor_position,
                           };
                           let start_pos = find_port_offset(start_port);

                           match start_port.io{
                               IO::In => {
                                   if start_port.node == wire_end_node{
                                           Some(((start_pos,end_pos),self.theme.extended_palette().secondary.weak.color))
                                   }
                                   else{
                                       None
                                   }
                               },
                               IO::Out =>{
                                   if start_port.node == wire_end_node{
                                           Some(((end_pos,start_pos),self.theme.extended_palette().secondary.weak.color))
                                   }
                                   else{
                                       None
                                   }

                               }
                           }
                       }
                       _=>None
                   };

                   let incoming_wires = self.graph.incoming_edges(&wire_end_node);
                   incoming_wires
                       .iter()
                       .map(|(from,to)| {

                           let connection = (
                               find_port_offset(to),
                               find_port_offset(from),
                               );
                           let color = if let Some((active_wire,_)) =  creating_active_wire {
                                if(dbg!(active_wire.0) == dbg!(connection.0) ) {
                                   if dbg!(new_connection_proposed) {
                                       self.theme.extended_palette().danger.base.color
                                   }
                                   else{
                                       self.theme.extended_palette().danger.weak.color
                                   }
                               }else{
                                   self.theme.extended_palette().success.weak.color
                               }
                           }else{
                                   self.theme.extended_palette().success.strong.color
                           };
                           (connection,color)
                       })
                       .chain(once(creating_active_wire).flatten())
                       .map(|((from, to),color)| {
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
               },
        )
        .on_shape_drag(Message::OnDrag)
        .on_cursor_move(Message::OnMove)
        .on_press(Message::OnSelect)
        .pan(Message::Pan);

        ////View
        row![
            container(
                column![
                    //// File
                    file_commands.align_y(Alignment::Center).width(Length::Fill),
                    ////Config
                    horizontal_rule(SEPERATOR),
                    ////
                    config
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
}
