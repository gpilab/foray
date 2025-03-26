use crate::config::Config;
use crate::file_watch::file_watch_subscription;
use crate::graph::{Graph, PortRef, IO};
use crate::gui_node::GuiGraph;
use crate::interface::add_node::add_node_panel;
use crate::interface::node_config::NodeUIWidget;
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
use crate::StableMap;

use iced::advanced::graphics::core::Element;
use iced::event::listen_with;
use iced::keyboard::key::Named;
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{column, *};
use iced::Event::Keyboard;
use iced::Length::Fill;
use iced::{mouse, window, Subscription, Task};
use indexmap::IndexMap;
use itertools::Itertools;
use log::{error, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::once;
use std::mem::discriminant;
use std::time::{Duration, Instant};

#[derive(Default, Clone, PartialEq)]
pub enum Action {
    #[default]
    InitialLoad,
    Idle,
    DragPan(Vector),
    DragNode(Vec<(u32, Vector)>),
    CreatingInputWire(PortRef, Option<PortRef>),
    CreatingOutputWire(PortRef, Option<PortRef>),
    AddingNode,
}

type UndoStash = Vec<(
    Graph<NodeData, PortType, PortData>,
    IndexMap<ShapeId, Point>,
)>;

pub struct App {
    pub network: Network,
    pub selected_shapes: HashSet<ShapeId>,
    pub cursor_position: Point,
    pub app_theme: AppTheme,
    pub config: Config,
    pub modifiers: Modifiers,
    pub queued_nodes: HashSet<u32>,
    //#[serde(skip)]
    //pub compute_task_handles: HashMap<u32, iced::task::Handle>,
    pub debug: bool,
    pub show_palette_ui: bool,
    pub available_nodes: Vec<NodeData>,
    pub action: Action,
    pub undo_stack: UndoStash,
    pub redo_stack: UndoStash,
}
impl Default for App {
    fn default() -> Self {
        let config = Config::default();
        config.setup_environment();
        let app_theme = Config::load_theme();

        let network =
            match read_to_string("networks/network.ron").map(|s| ron::from_str::<Network>(&s)) {
                Ok(Ok(network)) => network,
                Ok(Err(e)) => {
                    error!("Could not open file {e}");
                    warn!("creating default file");
                    Network::default()
                }
                Err(e) => {
                    error!("Could not parse file {e}");
                    warn!("creating default file");
                    Network::default()
                }
            };

        let available_nodes = NodeData::available_nodes(config.nodes_dir());
        App {
            network,
            config,

            debug: false,
            show_palette_ui: false,
            available_nodes,
            selected_shapes: Default::default(),
            cursor_position: Default::default(),
            action: Default::default(),
            queued_nodes: Default::default(),
            //compute_task_handles: Default::default(),
            app_theme,
            modifiers: Default::default(),
            undo_stack: vec![],
            redo_stack: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Network {
    pub graph: GuiGraph,
    pub shapes: workspace::State,
}

#[derive(Clone, derive_more::Debug)]
pub enum Message {
    //// Workspace
    OnMove(Point),
    ScrollPan(Vector),

    //// Port
    PortStartHover(PortRef),
    PortEndHover(PortRef),
    PortPress(PortRef),
    PortRelease,
    PortDelete(PortRef),

    //// Node
    OnCanvasDown(Option<ShapeId>),
    OnCanvasUp,
    OpenAddNodeUi,
    AddNode(NodeTemplate),

    UpdateNodeTemplate(u32, NodeTemplate),
    UpdateNodeParameter(u32, String, NodeUIWidget),
    DeleteSelectedNodes,

    QueueCompute(u32),
    ComputeComplete(
        u32,
        #[debug(skip)] Result<(StableMap<String, PortData>, NodeData), NodeError>,
    ),
    ComputeAll,

    //// Application
    AnimationTick,
    ThemeValueChange(AppThemeMessage, GuiColorMessage),
    ToggleDebug,
    TogglePaletteUI,
    Save,
    Load,
    ReloadNodes,
    WindowOpen,
    ModifiersChanged(Modifiers),

    //// Focus
    FocusNext,
    FocusPrevious,
    Cancel,

    //// History
    Undo,
    Redo,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnMove(_) => {}
            _ => trace!("---Message--- {:?}", message),
        }
        match message {
            Message::Cancel => self.action = Action::Idle,
            Message::OnMove(cursor_position) => {
                self.cursor_position = cursor_position;

                // Update node position if currently dragging
                match &self.action {
                    Action::DragNode(offsets) => {
                        offsets.iter().for_each(|(id, offset)| {
                            *self
                                .network
                                .shapes
                                .shape_positions
                                .get_mut(id)
                                .expect("Shape index must exist") =
                                (cursor_position + self.network.shapes.camera.position) + *offset
                        });
                    }
                    Action::DragPan(offset) => {
                        self.network.shapes.camera.position =
                            -cursor_position.to_vector() + *offset;
                    }
                    _ => (),
                }
            }

            Message::ScrollPan(delta) => {
                self.network.shapes.camera.position.x -= delta.x * 2.;
                self.network.shapes.camera.position.y -= delta.y * 2.;
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
                        self.network.graph.remove_edge(input);
                        self.network.graph.add_edge_from_ref(output, input);
                        Task::done(Message::QueueCompute(output.node))
                    }
                    _ => Task::none(),
                };
                self.action = Action::Idle;
                return task;
            }
            Message::PortDelete(port) => {
                self.stash_state();
                self.network.graph.remove_edge(&port)
            }

            //// Node
            Message::OnCanvasDown(clicked_id) => {
                //TODO: break this logic down into pure functions
                //// Clicked on a node
                if let Some(nx) = clicked_id {
                    self.selected_shapes = if self.modifiers.command() {
                        //// Create new nodes on Command + Click
                        self.stash_state();
                        let selected_shapes = if self.selected_shapes.contains(&nx) {
                            // If clicked node is already selected, copy all selected nodes,
                            self.selected_shapes.clone()
                        } else {
                            // Otherwise, only copy the clicked node
                            [nx].into()
                        };
                        selected_shapes
                            .iter()
                            .map(|id| {
                                let pos = self.network.shapes.shape_positions[id] + [5., 5.].into();
                                let new_node =
                                    self.network.graph.get_node(*id).template.duplicate().into();
                                // *Mutably* add new node to graph
                                let new_id = self.network.graph.node(new_node);
                                // *Mutably* add new position
                                self.network.shapes.shape_positions.insert(new_id, pos);
                                new_id
                            })
                            .collect()
                    } else if self.modifiers.shift() {
                        //// Select Multiple nodes if shift is held
                        self.selected_shapes
                            .clone()
                            .into_iter()
                            .chain(once(nx))
                            .collect()
                    } else if !self.selected_shapes.contains(&nx) {
                        //// Select Single Node if an unselected node is clicked
                        [nx].into()
                    } else {
                        //// Otherwise keep selection the same
                        self.selected_shapes.clone()
                    };

                    let offsets = self
                        .selected_shapes
                        .iter()
                        .map(|id| {
                            (
                                *id,
                                (self.network.shapes.shape_positions[id]
                                    - (self.cursor_position + self.network.shapes.camera.position)),
                            )
                        })
                        .collect();

                    //// Start Drag
                    self.action = Action::DragNode(offsets);

                    //// Move selected shape to the top
                    self.network.shapes.shape_positions.move_index(
                        self.network
                            .shapes
                            .shape_positions
                            .get_index_of(&nx)
                            .expect("id exists"),
                        0,
                    );
                    return Task::done(Message::QueueCompute(nx));
                } else
                //// Clicked on the canvas background
                {
                    //// Clear selected shapes
                    self.selected_shapes = Default::default();

                    //// Start Pan
                    self.action = Action::DragPan(
                        self.network.shapes.camera.position + self.cursor_position.to_vector(),
                    );
                }
            }
            Message::OnCanvasUp => {
                // TODO: push undo stack if shape has moved
                match self.action {
                    Action::DragNode(..) => self.action = Action::Idle,
                    Action::DragPan(_) => self.action = Action::Idle,
                    _ => (),
                }
            }
            Message::OpenAddNodeUi => self.action = Action::AddingNode,

            Message::UpdateNodeTemplate(id, new_template) => {
                let old_template = &self.network.graph.get_node(id).template;
                if *old_template != new_template {
                    self.stash_state();
                    // Now we can aquire mutable reference
                    let old_template = &mut self.network.graph.get_mut_node(id).template;
                    *old_template = new_template;
                    return Task::done(Message::QueueCompute(id));
                };
            }
            Message::UpdateNodeParameter(id, name, updated_widget) => {
                self.stash_state();
                let old_template = &mut self.network.graph.get_mut_node(id).template;
                // TODO: support all node types, not just py_node
                if let NodeTemplate::PyNode(node) = old_template {
                    node.parameters
                        .as_mut()
                        .expect("parameters must exist if they are being edited")
                        .insert(name, updated_widget);
                    return Task::done(Message::QueueCompute(id));
                }
            }
            Message::AddNode(template) => {
                self.stash_state();
                let id = self.network.graph.node(template.into());
                self.selected_shapes = [id].into();
                self.network.shapes.shape_positions.insert_before(
                    0,
                    id,
                    self.cursor_position + self.network.shapes.camera.position,
                );
                self.action = Action::DragNode(vec![(id, [0.0, 0.0].into())])
            }
            Message::DeleteSelectedNodes => {
                if !self.selected_shapes.is_empty() {
                    self.stash_state();
                    self.selected_shapes.iter().for_each(|id| {
                        self.network.graph.delete_node(*id);
                        self.network.shapes.shape_positions.swap_remove(id);
                    });
                    self.selected_shapes = [].into();
                    //PERF: ideally, we should only execute affected nodes
                    return Task::done(Message::ComputeAll);
                }
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
                    "networks/network.ron",
                    ron::ser::to_string_pretty(
                        &self.network,
                        ron::ser::PrettyConfig::default().compact_arrays(true),
                    )
                    .unwrap(),
                )
                .expect("Could not save to file");
            }
            Message::Load => {
                self.network = ron::from_str(
                    &read_to_string("networks/network.ron").expect("Could not read file"),
                )
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
            Message::ModifiersChanged(m) => {
                self.modifiers = m;
            }

            //// Focus
            Message::FocusNext => return focus_next(),
            Message::FocusPrevious => return focus_previous(),

            //// History
            Message::Undo => {
                if let Some(prev) = self.undo_stack.pop() {
                    self.redo_stack.push((
                        self.network.graph.clone(),
                        self.network.shapes.shape_positions.clone(),
                    ));
                    self.network.graph = prev.0;
                    self.network.shapes.shape_positions = prev.1;
                    return Task::done(Message::ComputeAll);
                }
            }
            Message::Redo => {
                if let Some(next) = self.redo_stack.pop() {
                    self.undo_stack.push((
                        self.network.graph.clone(),
                        self.network.shapes.shape_positions.clone(),
                    ));
                    self.network.graph = next.0;
                    self.network.shapes.shape_positions = next.1;
                    return Task::done(Message::ComputeAll);
                }
            }
            Message::ComputeAll => {
                let nodes = self.network.graph.get_roots();
                trace!("Queuing root nodes: {nodes:?}");
                return Task::batch(
                    nodes
                        .into_iter()
                        .map(|nx| Task::done(Message::QueueCompute(nx))),
                );
            }
            Message::QueueCompute(nx) => {
                //// Modify node status
                {
                    let node = self.network.graph.get_mut_node(nx);

                    // Re-queue
                    if let NodeStatus::Running(..) = node.status {
                        trace!("Re-queue, {} #{nx}", node.template);
                        self.queued_nodes.insert(nx);
                        return Task::none();
                    };

                    node.status = NodeStatus::Running(Instant::now());
                    trace!("Beginning compute: {} #{nx}", node.template,);
                }

                //// Queue compute
                let node = self.network.graph.get_node(nx);
                return Task::perform(
                    Graph::async_compute(nx, node.clone(), self.network.graph.get_input_data(&nx)),
                    move |(nx, res)| Message::ComputeComplete(nx, res),
                );
            }
            Message::ComputeComplete(nx, result) => {
                match result {
                    Ok((output, node)) => {
                        // Assert that status is what is expected
                        let run_time = match &node.status {
                            NodeStatus::Idle => panic!("Node should not be idle here!"),
                            NodeStatus::Running(start_inst) => Instant::now() - *start_inst,
                            NodeStatus::Error(_node_error) => panic!("Node should not be Error, compute should have returned an Error result and node.status is set to Error in the match arm below"),
                        };

                        trace!("Compute complete: {} #{nx}, {run_time:.1?}", node.template,);

                        //// Update wire
                        self.network.graph.update_wire_data(nx, output);

                        //// Update node
                        self.network.graph.set_node_data(
                            nx,
                            NodeData {
                                status: NodeStatus::Idle,
                                run_time: Some(run_time),
                                // We *don't* update template here for some nodes
                                // because that causes stuttery behaviour for
                                // fast update scenarios like the slider of the 'constant'
                                // node. alternatively, canceling in progress compute tasks
                                // might address this, and may be necessary in the future.
                                // similar to TODO: below
                                template: match node.template {
                                    NodeTemplate::RustNode(RustNode::Constant(_)) => {
                                        self.network.graph.get_node(nx).template.clone()
                                    }
                                    NodeTemplate::PyNode(_) => {
                                        self.network.graph.get_node(nx).template.clone()
                                    }
                                    _ => node.template,
                                },
                            },
                        );

                        //// Queue children for compute
                        let to_queue: Vec<_> = self
                            .network
                            .graph
                            .outgoing_edges(&nx)
                            .into_iter()
                            .map(|port_ref| port_ref.node)
                            .unique() // Don't queue a child multiple times
                            // TODO: instead of requeing after compute is done,
                            // potentially abort the running compute task, and restart
                            // immediately when new input data is received
                            .chain(once(self.queued_nodes.remove(&nx).then_some(nx)).flatten()) // Re-execute node if it got queued up in the meantime
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
                        let node = self.network.graph.get_mut_node(nx);
                        error!("Compute failed {node:?},{}", node_error);
                        node.status = NodeStatus::Error(node_error);
                        node.run_time = None;

                        //// Update Wire
                        self.network.graph.update_wire_data(nx, [].into());

                        return Task::none();
                    }
                };
            }
        };
        Task::none()
    }

    /// App View
    pub fn view(&self) -> Element<Message, Theme, Renderer> {
        let content = column![
            row![
                side_bar(self),
                vertical_rule(SEPERATOR),
                container(
                    workspace(
                        &self.network.shapes,
                        //// Node view
                        |id| self.node_content(id),
                        //// Wires paths
                        |wire_end_node, points| self.wire_curve(wire_end_node, points),
                    )
                    .on_cursor_move(Message::OnMove)
                    .on_press(Message::OnCanvasDown)
                    .on_release(Message::OnCanvasUp)
                    .pan(Message::ScrollPan)
                )
                .height(Fill)
                .width(Fill)
            ],
            match self.show_palette_ui {
                true => column![horizontal_rule(SEPERATOR), self.app_theme.view()],
                false => column![],
            }
        ];

        let modal = container(add_node_panel(&self.available_nodes)).center(Fill);
        // Potentially add a modal
        let output: Element<Message, Theme, Renderer> = match self.action {
            Action::AddingNode => stack![
                content,
                // Barrier to stop interaction
                mouse_area(
                    container(text(""))
                        .center(Fill)
                        .style(container::transparent)
                )
                // Stop any mouseover interactions from showing,
                .interaction(mouse::Interaction::Idle)
                .on_press(Message::Cancel),
                modal
            ]
            .into(),
            _ => content.into(),
        };

        // Potentially add a specific mouse cursor
        let output = match self.action {
            Action::DragNode(_) => mouse_area(output)
                .interaction(mouse::Interaction::Move)
                .into(),
            _ => output,
        };

        if self.debug {
            output.explain(iced::Color::from_rgba(0.7, 0.7, 0.8, 0.2))
        } else {
            output
        }
    }

    /// Stash current app state, and reset the redo stack
    fn stash_state(&mut self) {
        let mut graph_snap_shot = self.network.graph.clone();
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
            .push((graph_snap_shot, self.network.shapes.shape_positions.clone()));

        // Don't let the stack get too big
        self.undo_stack.truncate(10);

        self.redo_stack.clear();
    }

    /// Read node definitions from disk, and copies node configuration (parameters and port connections) forward.
    /// *Does not trigger the compute function of any nodes.*
    fn reload_nodes(&mut self) {
        // Update any existing nodes in the graph that could change based on file changes
        self.network.graph.nodes_ref().iter().for_each(|nx| {
            let node = self.network.graph.get_node(*nx).clone();
            if let NodeTemplate::PyNode(old_py_node) = node.template {
                let PyNode {
                    name: _node_name,
                    path,
                    ports: old_ports,
                    parameters: old_parameters,
                } = old_py_node;
                //// Read new node from disk
                let mut new_py_node = PyNode::new(path);

                //// Update Parameters
                new_py_node.parameters = {
                    // If Ok, copy old parameters to new parameters
                    if let (Ok(new_parameters), Ok(old_param)) =
                        (new_py_node.parameters.clone(), old_parameters)
                    {
                        // Only keep old values that are still present in the new parameters list
                        Ok(new_parameters
                            .clone()
                            .into_iter()
                            .chain(old_param.into_iter().filter(|(k, v)| {
                                if let Some(new_v) = new_parameters.get(k) {
                                    discriminant(v) == discriminant(new_v)
                                } else {
                                    false
                                }
                            }))
                            .collect())
                    } else {
                        new_py_node.parameters
                    }
                };

                //// Update Ports, and Graph Edges
                {
                    let new_ports = new_py_node.ports.clone().unwrap_or_default();

                    let new_in_ports = new_ports.inputs;
                    let new_out_ports = new_ports.outputs;

                    // Get old version's ports
                    let old_ports = old_ports.unwrap_or_default();
                    let old_in_ports = old_ports.inputs;
                    let old_out_ports = old_ports.outputs;

                    // Find any nodes that previously existed, but now doesn't
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
                        .filter(|(old_name, old_type)| {
                            new_out_ports.get(old_name) != Some(old_type)
                        })
                        .map(|(old_name, _)| PortRef {
                            node: *nx,
                            name: old_name,
                            io: IO::Out,
                        });

                    // Remove invalid edges from Graph
                    invalid_in.chain(invalid_out).for_each(|p| {
                        warn!(
                            "Removing port {:?} from node {:?}",
                            p.name, new_py_node.name
                        );
                        self.network.graph.remove_edge(&p);
                    });
                }

                // Update Graph Node
                self.network
                    .graph
                    .set_node_data(*nx, NodeTemplate::PyNode(new_py_node).into());
            }
        });
        // Update list of available nodes
        self.available_nodes = NodeData::available_nodes(self.config.nodes_dir());
    }
}

pub fn theme(state: &App) -> Theme {
    state.app_theme.clone().into()
}

pub fn subscriptions(state: &App) -> Subscription<Message> {
    Subscription::batch([
        file_watch_subscription(state.config.nodes_dir().clone()),
        window::open_events().map(|_| Message::WindowOpen),
        listen_with(|event, _status, _id| match event {
            Keyboard(keyboard::Event::ModifiersChanged(m)) => Some(Message::ModifiersChanged(m)),
            Keyboard(KeyPressed { key, modifiers, .. }) => match key {
                Key::Named(Named::Tab) => {
                    if modifiers.contains(Modifiers::SHIFT) {
                        Some(Message::FocusPrevious)
                    } else {
                        Some(Message::FocusNext)
                    }
                }
                Key::Named(Named::Delete) => Some(Message::DeleteSelectedNodes),
                Key::Named(Named::Escape) => Some(Message::Cancel),
                Key::Character(smol_str) => {
                    if modifiers.control() && smol_str == "a" {
                        Some(Message::OpenAddNodeUi)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }),
        // Refresh for animation while nodes are actively running
        if state.network.graph.running_nodes().is_empty() {
            Subscription::none()
        } else {
            iced::time::every(Duration::from_micros(1_000_000 / 16)).map(|_| Message::AnimationTick)
        },
    ])
}
