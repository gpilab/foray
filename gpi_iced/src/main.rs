use canvas::{Path, Stroke};
use gpi_iced::graph::{constant_node, identity_node, Graph};
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

struct Example {
    graph: Graph<Node, String, PortType, u32>,
    shapes: workspace::State,
    selected_shape: Option<ShapeId>,
    config: f32,
    theme: Theme,
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
            |name: &str, c: u32| constant_node(Node { name: name.into() }, c, PortType::Integer);
        let identity_node =
            |name: &str| identity_node(Node { name: name.into() }, PortType::Integer);

        let initial_nodes = vec![
            constant_node("a", 7),
            identity_node("b"),
            identity_node("c"),
            identity_node("d"),
            identity_node("e"),
            identity_node("f"),
        ];

        let mut g = Graph::<Node, String, PortType, u32>::new();
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
            .map(|((k, _v), p)| (*k, *p))
            .collect();

        Self {
            graph: g,
            shapes: workspace::State::new(nr),
            selected_shape: None,
            config: 50.,
            theme: Theme::Ferra,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnDrag(ShapeId, Point),
    Pan(Vector),
    Config(f32),
    OnSelect(ShapeId),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Pan(delta) => {
                self.shapes.camera.position.x -= delta.x * 2.;
                self.shapes.camera.position.y -= delta.y * 2.;
            }
            Message::Config(v) => self.config = v,
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
        };
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        const SEPERATOR: f32 = 1.0;

        fn button_style(t: &Theme, s: button::Status) -> button::Style {
            let mut style = button::secondary(t, s);
            style.border.radius = radius(0.);
            style
        }
        fn port_style(t: &Theme, s: button::Status) -> button::Style {
            let mut style =
                button::primary(t, s).with_background(t.extended_palette().primary.weak.color);
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
            row!["Label1", slider(0.0..=100.0, self.config, Message::Config)]
                .spacing(20.)
                .align_y(Alignment::Center),
            row!["Label2", slider(0.0..=100.0, self.config, Message::Config)]
                .spacing(20.)
                .align_y(Alignment::Center),
            row!["Label3", slider(0.0..=100.0, self.config, Message::Config)]
                .spacing(20.)
                .align_y(Alignment::Center),
            vertical_space(),
        ]
        .spacing(5.0)
        .padding(5.);

        let workspace = workspace(
            &self.shapes,
            |id| {
                let node = self.graph.get_node(id);
                let is_selected = match self.selected_shape {
                    Some(s_id) => id == s_id,
                    None => false,
                };

                let name = node.data.name.clone();

                let out_edges = self.graph.outgoing_edges(&id);
                dbg!(&out_edges);
                let out_ports = out_edges
                    .iter()
                    .enumerate()
                    .map(|(i, (_from, _to))| {
                        let port_x = i as f32 * (NODE_WIDTH / 4.) + 10.0;
                        Point::new(port_x, NODE_HEIGHT - 5.)
                    })
                    .map(|v| {
                        Pin::new(button("").style(port_style).width(10.).height(10.))
                            .position(v)
                            .into()
                    });

                let in_edges = self.graph.incoming_edges(&id);
                dbg!(&in_edges);
                let in_ports = in_edges
                    .iter()
                    .enumerate()
                    .map(|(i, (_from, _to))| {
                        let port_x = i as f32 * (NODE_WIDTH / 4.) + 10.0;
                        Point::new(port_x, -5.)
                    })
                    .map(|v| {
                        Pin::new(button("").style(port_style).width(10.).height(10.))
                            .position(v)
                            .into()
                    });

                let content: Element<Message, Theme, Renderer> = NodeContainer::new(
                    container(text(name))
                        .style(move |t: &Theme| {
                            let outline_color = match is_selected {
                                true => t.extended_palette().primary.strong.color,
                                false => t.extended_palette().secondary.strong.color,
                            };
                            container::transparent(t)
                                .border(rounded(5.).color(outline_color).width(2.))
                                .background(self.theme.palette().background)
                        })
                        .center_x(NODE_WIDTH)
                        .center_y(NODE_HEIGHT),
                    in_ports.chain(out_ports).collect(),
                )
                .into();
                content //.explain(Color::WHITE)
            },
            |wire_end_node, points| {
                let incoming_wires = self.graph.incoming_edges(&wire_end_node);
                incoming_wires
                    .iter()
                    .enumerate()
                    .map(|(i, (wire_start_node, _wire_start_port))| {
                        // WARN: i assumes sorted ports for positioning
                        let port_x = i as f32 * (NODE_WIDTH / 4.) + 15.0;
                        (
                            //TODO: change port_x based on it's from_node output position
                            points[wire_start_node] + Vector::new(port_x, NODE_HEIGHT),
                            points[&wire_end_node] + Vector::new(port_x, 0.),
                        )
                    })
                    .map(|(from, to)| {
                        (
                            Path::new(|builder| {
                                builder.move_to(from);
                                let mid = f32::abs((from.y - to.y) * 0.5).max(15.0);
                                builder.bezier_curve_to(
                                    (from.x, from.y + mid).into(),
                                    (to.x, to.y - mid).into(),
                                    to,
                                );
                            }),
                            Stroke::default()
                                .with_width(3.0)
                                .with_color(self.theme.extended_palette().secondary.weak.color)
                                .with_line_cap(canvas::LineCap::Round),
                        )
                    })
                    .collect()
            },
        )
        .on_shape_drag(Message::OnDrag)
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
