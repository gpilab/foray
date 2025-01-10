use crate::graph::{Graph, PortRef, IO};
use crate::interface::{side_bar::side_bar, SEPERATOR};
use crate::math::{Point, Vector};
use crate::node_data::NodeData;
use crate::nodes::linspace::LinspaceConfig;
use crate::nodes::plot::Plot;
use crate::nodes::{PortData, PortType};
use crate::widget::shapes::ShapeId;
use crate::widget::workspace::{self, workspace};
use crate::OrderMap;
use iced::advanced::graphics::core::Element;
use iced::event::listen_with;
use iced::keyboard::key::Named;
use iced::keyboard::{Key, Modifiers};
use iced::widget::*;
use iced::Length::Fill;
use iced::{Subscription, Task};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Default, Clone)]
pub enum Action {
    #[default]
    Idle,
    CreatingInputWire(PortRef, Option<PortRef>),
    CreatingOutputWire(PortRef, Option<PortRef>),
    AddingNode,
}

type UndoStash = Vec<(
    Graph<NodeData, PortType, PortData>,
    OrderMap<ShapeId, Point>,
)>;

#[derive(Serialize, Deserialize)]
pub struct App {
    pub graph: Graph<NodeData, PortType, PortData>,
    pub shapes: workspace::State,
    pub selected_shape: Option<ShapeId>,
    pub cursor_position: Point,
    pub config: f32,
    pub debug: bool,
    #[serde(skip, default = "default_theme")]
    pub theme: Theme,
    #[serde(skip)]
    pub action: Action,
    #[serde(skip)]
    pub undo_stack: UndoStash,
    #[serde(skip)]
    pub redo_stack: UndoStash,
}

#[derive(Clone, Debug)]
pub enum Message {
    //// Workspace
    OnDrag(ShapeId, Point),
    OnMove(Point),
    Pan(Vector),

    //// Port
    PortStartHover(PortRef),
    PortEndHover(PortRef),
    PortPress(PortRef),
    PortRelease,
    PortDelete(PortRef),

    //// Node
    OnSelect(Option<ShapeId>),
    UpdateNodeData(u32, NodeData),
    AddNode(NodeData),
    DeleteNode(u32),

    //// Application
    Config(f32),
    ToggleDebug,
    Save,
    Load,

    //// Focus
    FocusNext,
    FocusPrevious,

    //// History
    Undo,
    Redo,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            //// Workspace
            Message::OnDrag(shape_index, cursor_position) => {
                //TODO: how to handle undo/redo? Drag End event?
                *self
                    .shapes
                    .shape_positions
                    .get_mut(&shape_index)
                    .expect("Shape index must exist") = cursor_position
            }
            Message::OnMove(cursor_position) => self.cursor_position = cursor_position,
            Message::Pan(delta) => {
                self.shapes.camera.position.x -= delta.x * 2.;
                self.shapes.camera.position.y -= delta.y * 2.;
            }

            //// Port
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
            Message::PortPress(port) => match port.io {
                IO::In => self.action = Action::CreatingInputWire(port, None),
                IO::Out => self.action = Action::CreatingOutputWire(port, None),
            },
            Message::PortRelease => {
                match &self.action.clone() {
                    Action::CreatingInputWire(input, Some(output))
                    | Action::CreatingOutputWire(output, Some(input)) => {
                        self.stash_state();
                        self.graph.remove_edge(input);
                        self.graph.add_edge_from_ref(output, input);
                        self.graph.exectute_sub_network(output.node);
                    }
                    _ => {}
                }

                self.action = Action::Idle;
            }
            Message::PortDelete(port) => {
                self.stash_state();
                self.graph.remove_edge(&port)
            }

            //// Node
            Message::OnSelect(maybe_id) => {
                self.selected_shape = maybe_id;
                if let Some(shape_id) = maybe_id {
                    self.graph.exectute_sub_network(shape_id);
                }
            }

            Message::UpdateNodeData(id, node) => {
                self.stash_state();
                *self.graph.get_mut_node(id) = node;
                self.graph.exectute_sub_network(id);
            }
            Message::AddNode(node) => {
                self.stash_state();
                let id = self.graph.node(node);
                self.shapes.shape_positions.insert(id, (100., 500.).into());
            }
            Message::DeleteNode(id) => {
                self.stash_state();
                self.graph.delete_node(id);
                self.shapes.shape_positions.remove(&id);
                self.selected_shape = None;
                //PERF: ideally, we should only execute affected nodes
                self.graph.execute_network();
            }

            //// Application
            Message::Config(v) => self.config = v,
            Message::ToggleDebug => {
                self.debug = !self.debug;
            }
            Message::Save => {
                std::fs::write(
                    "network.ron",
                    ron::ser::to_string_pretty(
                        &self,
                        ron::ser::PrettyConfig::default().compact_arrays(true),
                    )
                    .unwrap(),
                )
                .expect("Could not save to file");
            }
            Message::Load => {
                *self = ron::from_str(&read_to_string("network.ron").expect("Could not read file"))
                    .expect("could not parse file");
                self.graph.execute_network();
            }

            //// Focus
            Message::FocusNext => return focus_next(),
            Message::FocusPrevious => return focus_previous(),

            //// History
            Message::Undo => {
                if let Some(prev) = self.undo_stack.pop() {
                    self.redo_stack
                        .push((self.graph.clone(), self.shapes.shape_positions.clone()));
                    self.graph = prev.0;
                    self.shapes.shape_positions = prev.1;
                }
            }
            Message::Redo => {
                if let Some(next) = self.redo_stack.pop() {
                    self.undo_stack
                        .push((self.graph.clone(), self.shapes.shape_positions.clone()));
                    self.graph = next.0;
                    self.shapes.shape_positions = next.1;
                }
            }
        };
        Task::none()
    }

    /// App View
    pub fn view(&self) -> Element<Message, Theme, Renderer> {
        row![
            side_bar(self),
            vertical_rule(SEPERATOR),
            container(
                workspace(
                    &self.shapes,
                    //// Node view
                    |id| self.node_content(id),
                    //// Wires paths
                    |wire_end_node, points| self.wire_curve(wire_end_node, points),
                )
                .on_shape_drag(Message::OnDrag)
                .on_cursor_move(Message::OnMove)
                .on_press(Message::OnSelect)
                .pan(Message::Pan)
            )
            .height(Fill)
            .width(Fill)
        ]
        .into()
    }

    /// stash current app state, and reset the redo stack
    fn stash_state(&mut self) {
        //PERF: cloning the wire data in here will be really expensive when large data is used
        // Need to only store graph structure, and transfer data between undo/redo states.
        // just need to carefully handle invalid data between states
        // Also - likely need to re-execute graph after undo/redo, to make sure everything is up to
        // date!
        //PERF: set a stack limit!
        self.undo_stack
            .push((self.graph.clone(), self.shapes.shape_positions.clone()));

        // Don't let the stack get too big, especially while we naively store everything
        self.undo_stack.truncate(10);

        self.redo_stack.clear();
    }
}

pub fn theme(_state: &App) -> Theme {
    default_theme()
}

fn default_theme() -> Theme {
    Theme::Ferra
}

pub fn subscriptions(_state: &App) -> Subscription<Message> {
    listen_with(|event, _status, _id| match event {
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key: Key::Named(Named::Tab),
            modifiers,
            ..
        }) => {
            if modifiers.contains(Modifiers::SHIFT) {
                Some(Message::FocusPrevious)
            } else {
                Some(Message::FocusNext)
            }
        }
        _ => None,
    })
}

impl Default for App {
    fn default() -> App {
        // try to load file
        match read_to_string("network.ron").map(|s| ron::from_str::<App>(&s)) {
            Ok(Ok(app)) => {
                let mut app = app;
                app.graph.execute_network();
                app
            }
            _ => {
                println!("Failed to load file, loading defaults");
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
                    debug: false,
                    theme: Theme::Ferra,
                    config: 50.,

                    selected_shape: None,
                    cursor_position: Default::default(),
                    action: Default::default(),

                    shapes: workspace::State::new(shapes.into()),
                    graph: g,
                    undo_stack: vec![],
                    redo_stack: vec![],
                }
            }
        }
    }
}
