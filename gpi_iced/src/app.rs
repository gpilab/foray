use crate::file_watch::file_watch_subscription;
use crate::graph::{Graph, PortRef, IO};
use crate::interface::theme_config::{AppThemeMessage, GuiColorMessage};
use crate::interface::{side_bar::side_bar, SEPERATOR};
use crate::math::{Point, Vector};
use crate::nodes::port::PortData;
use crate::nodes::port::PortType;
use crate::nodes::status::{NodeError, NodeStatus};
use crate::nodes::{NodeData, NodeTemplate, RustNode};
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
use log::{error, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::once;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::time::{Duration, Instant};

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

pub type PortDataReference<'a> = RwLockReadGuard<'a, PortData>;
pub type PortDataContainer = Arc<RwLock<PortData>>;

#[derive(Serialize, Deserialize)]
pub struct App {
    pub graph: Graph<NodeData, PortType, PortData>,
    pub shapes: workspace::State,
    pub selected_shape: Option<ShapeId>,
    pub cursor_position: Point,
    pub config: f32,
    pub app_theme: AppTheme,
    #[serde(skip)]
    pub queued_nodes: HashSet<u32>,
    //#[serde(skip)]
    //pub compute_task_handles: HashMap<u32, iced::task::Handle>,
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

#[derive(Clone, derive_more::Debug)]
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
    ComputeComplete(
        u32,
        #[debug(skip)] Result<(OrderMap<String, PortData>, NodeData), NodeError>,
    ),
    ComputeAll,

    //// Application
    Config(f32),
    AnimationTick,
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
        trace!("---Message--- {:?}", message);
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
                    // Move selected node to the front
                    self.shapes.shape_positions.move_index(
                        self.shapes
                            .shape_positions
                            .get_index_of(&nx)
                            .expect("id exists"),
                        0,
                    );
                    return Task::done(Message::QueueCompute(nx));
                }
            }

            Message::UpdateNodeTemplate(id, new_template) => {
                let old_template = &self.graph.get_node(id).template;
                if *old_template != new_template {
                    self.stash_state();
                    // now we can aquire mutable reference
                    let old_template = &mut self.graph.get_mut_node(id).template;
                    *old_template = new_template;
                    return Task::done(Message::QueueCompute(id));
                };
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
            Message::AnimationTick => {}
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
                trace!("Queuing root nodes: {nodes:?}");
                return Task::batch(
                    nodes
                        .into_iter()
                        .map(|nx| Task::done(Message::QueueCompute(nx))),
                );
            }
            Message::QueueCompute(nx) => {
                //// modify node status
                {
                    let node = self.graph.get_mut_node(nx);

                    // re-queue
                    if let NodeStatus::Running(..) = node.status {
                        trace!("Re-queue, {} #{nx}", node.template);
                        self.queued_nodes.insert(nx);
                        return Task::none();
                    };

                    node.status = NodeStatus::Running(Instant::now());
                    trace!("Beginning compute: {} #{nx}", node.template,);
                }

                //// queue compute
                let node = self.graph.get_node(nx);
                return Task::perform(
                    Graph::async_compute(nx, node.clone(), self.graph.get_input_data(&nx)),
                    move |(nx, res)| Message::ComputeComplete(nx, res),
                );
            }
            Message::ComputeComplete(nx, result) => {
                match result {
                    Ok((output, node)) => {
                        // assert that status is what is expected
                        let run_time = match &node.status {
                            NodeStatus::Idle => panic!("Node should not be idle here!"),
                            NodeStatus::Running(start_inst) => Instant::now() - *start_inst,
                            NodeStatus::Error(_node_error) => panic!("Node should not be Error, compute should have returned an Error result and node.status is set to Error in the match arm below"),
                        };

                        trace!("Compute complete: {} #{nx}, {run_time:.1?}", node.template,);

                        //// Update wire
                        self.graph.update_wire_data(nx, output);

                        //// Update node
                        self.graph.set_node_data(
                            nx,
                            NodeData {
                                status: NodeStatus::Idle,
                                run_time: Some(run_time),
                                // we *don't* update template here for some nodes
                                // because that causes stuttery behaviour for
                                // fast update scenarios like the slider of the 'constant'
                                // node. alternatively, canceling in progress compute tasks
                                // might address this, and may be necessary in the future.
                                // similar to TODO: below
                                template: match node.template {
                                    NodeTemplate::RustNode(RustNode::Constant(_)) => {
                                        self.graph.get_node(nx).template.clone()
                                    }
                                    _ => node.template,
                                },
                            },
                        );

                        //// Queue children for compute
                        let to_queue: Vec<_> = self
                            .graph
                            .outgoing_edges(&nx)
                            .into_iter()
                            .map(|port_ref| port_ref.node)
                            .unique() // don't queue a child multiple times
                            // TODO: instead of requeing after compute is done,
                            // potentially abort the running compute task, and restart
                            // immediately when new input data is received
                            .chain(once(self.queued_nodes.remove(&nx).then_some(nx)).flatten()) // re-execute node if it got queued up in the meantime
                            .collect();
                        trace!("Queuing children for compute {to_queue:?}");
                        return Task::batch(
                            to_queue
                                .into_iter()
                                .map(|node| Task::done(Message::QueueCompute(node))),
                        );
                    }
                    Err(node_error) => {
                        //// Update Node
                        let node = self.graph.get_mut_node(nx);
                        error!("Compute failed {node:?},{}", node_error);
                        node.status = NodeStatus::Error(node_error);
                        node.run_time = None;

                        //// Update Wire
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

    /// Stash current app state, and reset the redo stack
    fn stash_state(&mut self) {
        let mut graph_snap_shot = self.graph.clone();
        // We don't want to stash any node.status "running" values
        let running_nodes: Vec<_> = graph_snap_shot
            .nodes_ref()
            .into_iter()
            .filter(|nx| {
                matches!(
                    graph_snap_shot.get_node(*nx).status,
                    NodeStatus::Running(..)
                )
            })
            .collect();
        for nx in running_nodes {
            graph_snap_shot.get_mut_node(nx).status = NodeStatus::Idle;
        }

        self.undo_stack
            .push((graph_snap_shot, self.shapes.shape_positions.clone()));

        // Don't let the stack get too big
        self.undo_stack.truncate(10);

        self.redo_stack.clear();
    }

    /// Re-calculates node definitions.
    /// *Does not recompute any nodes.*
    fn reload_nodes(&mut self) {
        // Update any existing nodes in the graph that could change based on file changes
        self.graph.nodes_ref().iter().for_each(|nx| {
            let node = self.graph.get_node(*nx);
            if let NodeTemplate::PyNode(old_py_node) = &node.template {
                // Get old version's ports
                let old_ports = old_py_node.clone().ports.unwrap_or_default();
                let old_in_ports = old_ports.inputs;
                let old_out_ports = old_ports.outputs;

                // Get new node version, reading from disk
                let new_py_node = PyNode::new(&old_py_node.name);
                let new_ports = new_py_node.clone().ports.unwrap_or_default();

                let new_in_ports = new_ports.inputs;
                let new_out_ports = new_ports.outputs;

                // Find any nodes that previously existed, but now do not
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

                // Remove invalid edges from graph
                invalid_in.chain(invalid_out).for_each(|p| {
                    warn!(
                        "Removing port {:?} from node {:?}",
                        p.name, new_py_node.name
                    );
                    self.graph.remove_edge(&p);
                });

                // Update the node with most recent changes
                self.graph
                    .set_node_data(*nx, NodeTemplate::PyNode(new_py_node).into());
            }
        });
        // Update list of available nodes
        self.availble_nodes = NodeData::available_nodes();
    }
}

pub fn theme(state: &App) -> Theme {
    state.app_theme.clone().into()
}

pub fn subscriptions(state: &App) -> Subscription<Message> {
    let animation_running = if state
        .graph
        .nodes_ref()
        .into_iter()
        .map(|nx| state.graph.get_node(nx))
        .filter(|node| matches!(node.status, NodeStatus::Running(..)))
        .collect::<Vec<_>>()
        .is_empty()
    {
        Subscription::none()
    } else {
        iced::time::every(Duration::from_micros(1_000_000 / 16)).map(|_| Message::AnimationTick)
    };

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
        animation_running,
    ])
}

impl Default for App {
    fn default() -> App {
        // Try to load file
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
                    queued_nodes: Default::default(),
                    //compute_task_handles: Default::default(),
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
