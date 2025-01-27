use crate::file_watch::file_watch_subscription;
use crate::graph::{Graph, PortRef, IO};
use crate::interface::theme_config::{AppThemeMessage, GuiColorMessage};
use crate::interface::{side_bar::side_bar, SEPERATOR};
use crate::math::{Point, Vector};
use crate::nodes::port::PortData;
use crate::nodes::port::PortType;
use crate::nodes::status::{NodeError, NodeStatus};
use crate::nodes::{NodeData, NodeTemplate};
use crate::python::py_node::PyNode;
use crate::style::theme::AppTheme;
use crate::widget::shapes::ShapeId;
use crate::widget::workspace::{self, workspace};
use crate::OrderMap;

use iced::advanced::graphics::core::Element;
use iced::event::listen_with;
use iced::keyboard::key::Named;
use iced::keyboard::{Key, Modifiers};
use iced::widget::{column, *};
use iced::Length::Fill;
use iced::{window, Subscription, Task};
use itertools::Itertools;
use log::{trace, warn};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

#[derive(Default, Clone, PartialEq)]
pub enum Action {
    #[default]
    InitialLoad,
    Idle,
    CreatingInputWire(PortRef, Option<PortRef>),
    CreatingOutputWire(PortRef, Option<PortRef>),
    AddingNode,
}

type UndoStash = Vec<(
    Graph<NodeData, PortType, PortData>,
    OrderMap<ShapeId, Point>,
)>;

pub type PortDataReference<'a> = &'a MutexGuard<'a, PortData>;
pub type PortDataContainer = Arc<Mutex<PortData>>;

#[derive(Serialize, Deserialize)]
pub struct App {
    pub graph: Graph<NodeData, PortType, PortData>,
    pub shapes: workspace::State,
    pub selected_shape: Option<ShapeId>,
    pub cursor_position: Point,
    pub config: f32,
    pub app_theme: AppTheme,
    #[serde(skip)]
    pub debug: bool,
    #[serde(skip)]
    pub show_palette_ui: bool,
    #[serde(skip)]
    pub availble_nodes: Vec<NodeData>,
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
    UpdateNodeTemplate(u32, NodeTemplate),
    AddNode(NodeTemplate),
    DeleteNode(u32),

    QueueCompute(u32),
    ComputeComplete(u32, Instant, Result<OrderMap<String, PortData>, NodeError>),
    ComputeAll,

    //// Application
    Config(f32),
    ThemeValueChange(AppThemeMessage, GuiColorMessage),
    ToggleDebug,
    TogglePaletteUI,
    Save,
    Load,
    ReloadNodes,
    WindowOpen,

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
                let task = match &self.action.clone() {
                    Action::CreatingInputWire(input, Some(output))
                    | Action::CreatingOutputWire(output, Some(input)) => {
                        self.stash_state();
                        self.graph.remove_edge(input);
                        self.graph.add_edge_from_ref(output, input);
                        Task::done(Message::QueueCompute(output.node))
                    }
                    _ => Task::none(),
                };
                self.action = Action::Idle;
                return task;
            }
            Message::PortDelete(port) => {
                self.stash_state();
                self.graph.remove_edge(&port)
            }

            //// Node
            Message::OnSelect(maybe_id) => {
                self.selected_shape = maybe_id;
                if let Some(nx) = maybe_id {
                    return Task::done(Message::QueueCompute(nx));
                }
            }

            Message::UpdateNodeTemplate(id, template) => {
                self.stash_state();
                self.graph.get_mut_node(id).template = template;
                return Task::done(Message::QueueCompute(id));
            }
            Message::AddNode(template) => {
                self.stash_state();
                let id = self.graph.node(template.into());
                self.shapes.shape_positions.insert(id, (100., 500.).into());
            }
            Message::DeleteNode(id) => {
                self.stash_state();
                self.graph.delete_node(id);
                self.shapes.shape_positions.swap_remove(&id);
                self.selected_shape = None;
                //PERF: ideally, we should only execute affected nodes
                return Task::done(Message::ComputeAll);
            }

            //// Application
            Message::Config(v) => {
                println!("\n\n\n CONFIG!!!!!!!!!!!!!!!!!!!!!\n\n\n");
                self.config = v
            }
            Message::ThemeValueChange(tm, tv) => self.app_theme.update(tm, tv),
            Message::ToggleDebug => {
                self.debug = !self.debug;
            }
            Message::TogglePaletteUI => {
                self.show_palette_ui = !self.show_palette_ui;
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
                self.reload_nodes();
                return Task::done(Message::ComputeAll);
            }
            Message::ReloadNodes => {
                self.reload_nodes();
                return Task::done(Message::ComputeAll);
            }
            Message::WindowOpen => {
                if self.action == Action::InitialLoad {
                    self.action = Action::Idle;
                    return Task::done(Message::ComputeAll);
                }
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
                    return Task::done(Message::ComputeAll);
                }
            }
            Message::Redo => {
                if let Some(next) = self.redo_stack.pop() {
                    self.undo_stack
                        .push((self.graph.clone(), self.shapes.shape_positions.clone()));
                    self.graph = next.0;
                    self.shapes.shape_positions = next.1;
                    return Task::done(Message::ComputeAll);
                }
            }
            Message::ComputeAll => {
                let nodes = self.graph.get_roots();
                return Task::batch(
                    nodes
                        .into_iter()
                        .map(|nx| Task::done(Message::QueueCompute(nx))),
                );
            }
            Message::QueueCompute(nx) => {
                {
                    let node = self.graph.get_mut_node(nx);
                    if node.status == NodeStatus::Running {
                        warn!("node already running, requing to try again, {node:?}");
                        return Task::done(Message::QueueCompute(nx));
                    };
                    node.status = NodeStatus::Running;
                    trace!("beginning compute: node {nx}\n{node:?}");
                }
                let node = self.graph.get_node(nx);
                assert_eq!(node.status, NodeStatus::Running);
                let input_guarded = self.graph.get_input_data(&nx);
                let start_inst = Instant::now();
                return Task::perform(
                    Graph::async_compute(nx, node.clone(), input_guarded),
                    move |(nx, res)| Message::ComputeComplete(nx, start_inst, res),
                );
            }
            Message::ComputeComplete(nx, start_inst, result) => {
                let node = self.graph.get_mut_node(nx);
                trace!("compute complete, maybe successful: node {nx}\n{node:?}");
                match &node.status {
                    NodeStatus::Idle => panic!("Node should not be idle here!"),
                    NodeStatus::Running => {}
                    NodeStatus::Error(_node_error) => {}
                }

                match result {
                    Ok(output) => {
                        node.status = NodeStatus::Idle;
                        node.run_time = Some(Instant::now() - start_inst);
                        trace!("compute complete: node {nx}\n{node:?}");

                        self.graph.update_wire_data(nx, output);
                        // attempt to compute children

                        let to_queue: Vec<_> = self
                            .graph
                            .outgoing_edges(&nx)
                            .into_iter()
                            .map(|port_ref| port_ref.node)
                            .unique()
                            .collect(); // don't queue a node multiple times
                        trace!("Queuing nodes for compute {to_queue:?}");
                        return Task::batch(
                            to_queue
                                .into_iter()
                                .map(|node| Task::done(Message::QueueCompute(node))),
                        );
                    }
                    Err(node_error) => {
                        log::error!("{}", node_error);
                        node.status = NodeStatus::Error(node_error);
                        node.run_time = None;
                        self.graph.update_wire_data(nx, [].into());
                        return Task::none();
                    }
                };
            }
        };
        Task::none()
    }

    /// App View
    pub fn view(&self) -> Element<Message, Theme, Renderer> {
        let content: Element<Message, Theme, Renderer> = column![
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
            ],
            match self.show_palette_ui {
                true => column![horizontal_rule(SEPERATOR), self.app_theme.view()],
                false => column![],
            }
        ]
        .into();
        if self.debug {
            content.explain(iced::Color::from_rgba(0.7, 0.7, 0.8, 0.2))
        } else {
            content
        }
    }

    /// stash current app state, and reset the redo stack
    fn stash_state(&mut self) {
        //PERF: cloning the wire data in here will be really expensive when large data is used
        // Need to only store graph structure, and transfer data between undo/redo states.
        // just need to carefully handle invalid data between states
        // Also - likely need to re-execute graph after undo/redo, to make sure everything is up to
        // date!
        self.undo_stack
            .push((self.graph.clone(), self.shapes.shape_positions.clone()));

        // Don't let the stack get too big, especially while we naively store everything
        self.undo_stack.truncate(10);

        self.redo_stack.clear();
    }

    fn reload_nodes(&mut self) {
        //update any existing nodes in the graph that could change based on file changes
        self.graph.nodes_ref().iter().for_each(|nx| {
            let node = self.graph.get_node(*nx);
            if let NodeTemplate::PyNode(old_py_node) = &node.template {
                // get old version's ports
                let old_ports = old_py_node.clone().ports.unwrap_or_default();
                let old_in_ports = old_ports.inputs;
                let old_out_ports = old_ports.outputs;

                // get new node version, reading from disk
                let new_py_node = PyNode::new(&old_py_node.name);
                let new_ports = new_py_node.clone().ports.unwrap_or_default();

                let new_in_ports = new_ports.inputs;
                let new_out_ports = new_ports.outputs;

                // find any nodes that previously existed, but now do not
                let invalid_in = old_in_ports
                    .into_iter()
                    .filter(|(old_name, old_type)| new_in_ports.get(old_name) != Some(old_type))
                    .map(|(old_name, _)| PortRef {
                        node: *nx,
                        name: old_name,
                        io: IO::In,
                    });
                let invalid_out = old_out_ports
                    .into_iter()
                    .filter(|(old_name, old_type)| new_out_ports.get(old_name) != Some(old_type))
                    .map(|(old_name, _)| PortRef {
                        node: *nx,
                        name: old_name,
                        io: IO::Out,
                    });

                // remove invalid edges from graph
                invalid_in.chain(invalid_out).for_each(|p| {
                    warn!(
                        "removing port {:?} from node {:?}",
                        p.name, new_py_node.name
                    );
                    self.graph.remove_edge(&p);
                });

                // update the node with most recent changes
                self.graph
                    .set_node_data(*nx, NodeTemplate::PyNode(new_py_node).into());
            }
        });
        //update list of available nodes
        self.availble_nodes = NodeData::available_nodes();
        //recompute all nodes
        //self.graph.execute_network();
    }
}

pub fn theme(state: &App) -> Theme {
    state.app_theme.clone().into()
}

pub fn subscriptions(_state: &App) -> Subscription<Message> {
    Subscription::batch([
        file_watch_subscription(),
        window::open_events().map(|_| Message::WindowOpen),
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
        }),
    ])
}

impl Default for App {
    fn default() -> App {
        // try to load file
        match read_to_string("network.ron").map(|s| ron::from_str::<App>(&s)) {
            Ok(Ok(app)) => {
                let mut app = app;
                app.availble_nodes = NodeData::available_nodes();
                app.reload_nodes();
                app
            }
            _ => {
                println!("Failed to load file, loading defaults");
                let g = Graph::<NodeData, PortType, PortData>::new();

                let shapes = [];

                Self {
                    debug: false,
                    show_palette_ui: false,
                    config: 50.,

                    selected_shape: None,
                    cursor_position: Default::default(),
                    action: Default::default(),

                    app_theme: Default::default(),
                    availble_nodes: NodeData::available_nodes(),
                    shapes: workspace::State::new(shapes.into()),
                    graph: g,
                    undo_stack: vec![],
                    redo_stack: vec![],
                }
            }
        }
    }
}
