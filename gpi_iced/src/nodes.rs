use std::sync::Mutex;
use std::time::{Duration, Instant};

pub mod async_compute;
pub mod constant;
pub mod linspace;
pub mod math_nodes;
pub mod plot;
pub mod plot_complex;
pub mod port;
pub mod status;

use crate::app::Message;
use crate::graph::GraphNode;
use crate::gui_node::GUINode;
use crate::interface::node::default_node_size;
use crate::nodes::linspace::LinspaceConfig;
use crate::nodes::math_nodes::{binary_operation, unary_operation};
use crate::nodes::plot::Plot;
use crate::nodes::plot_complex::Plot2D;
use crate::python::py_node::PyNode;
use crate::OrderMap;
use iced::widget::text;
use iced::Font;
use log::trace;
use port::{PortData, PortType};
use serde::{Deserialize, Serialize};
use status::{NodeError, NodeStatus};
use strum::{EnumIter, IntoEnumIterator, VariantNames};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeData {
    pub template: NodeTemplate,
    #[serde(skip)]
    pub status: NodeStatus,
    #[serde(skip)]
    pub run_time: Option<Duration>,
}

#[derive(Clone, Debug, EnumIter, VariantNames, Serialize, Deserialize)]
pub enum RustNode {
    Identity,
    Constant(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Cos,
    Sin,
    Sinc,
    Linspace(LinspaceConfig),
    Plot(Plot),
    Plot2D(Plot2D),
    Join,
    Reverse,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeTemplate {
    RustNode(RustNode),
    PyNode(PyNode),
}

impl From<NodeData> for NodeTemplate {
    fn from(value: NodeData) -> Self {
        value.template
    }
}
impl From<NodeTemplate> for NodeData {
    fn from(template: NodeTemplate) -> Self {
        NodeData {
            template,
            status: NodeStatus::Idle,
            run_time: None,
        }
    }
}

impl NodeData {
    fn fallible_compute(
        &mut self,
        inputs: OrderMap<String, &Mutex<PortData>>,
    ) -> Result<OrderMap<String, PortData>, NodeError> {
        Ok(match &mut self.template {
            NodeTemplate::RustNode(rust_node) => match rust_node {
                RustNode::Identity => [(
                    "out".to_string(),
                    inputs
                        .get("a")
                        .ok_or(NodeError::Input("input 'a' not found".to_string()))?
                        .lock()
                        .unwrap()
                        .clone(),
                )]
                .into(),
                RustNode::Constant(value) => {
                    [("out".to_string(), PortData::Real(vec![*value].into()))].into()
                }
                RustNode::Add => binary_operation(inputs, Box::new(|a, b| a + b))?,
                RustNode::Subtract => binary_operation(inputs, Box::new(|a, b| a - b))?,
                RustNode::Multiply => binary_operation(inputs, Box::new(|a, b| a * b))?,
                RustNode::Divide => binary_operation(inputs, Box::new(|a, b| a / b))?,
                RustNode::Join => binary_operation(
                    inputs,
                    Box::new(|a, b| a.iter().chain(b).copied().collect()),
                )?,
                RustNode::Reverse => {
                    unary_operation(inputs, Box::new(|a| a.iter().rev().copied().collect()))?
                }
                RustNode::Cos => unary_operation(inputs, Box::new(|a| a.cos()))?,
                RustNode::Sin => unary_operation(inputs, Box::new(|a| a.sin()))?,
                RustNode::Sinc => unary_operation(
                    inputs,
                    Box::new(|a| {
                        a.map(|x| match x {
                            0. => 1.,
                            _ => x.sin() / x,
                        })
                    }),
                )?,
                RustNode::Linspace(linspace_config) => linspace_config.compute(inputs),
                RustNode::Plot(_) => [].into(),
                RustNode::Plot2D(plot_2d) => plot_2d.input_changed(inputs),
            },

            NodeTemplate::PyNode(py_node) => py_node.compute(inputs)?,
        })
    }

    pub fn available_nodes() -> Vec<NodeData> {
        let nodes = RustNode::iter()
            .map(|template| template.template_variants())
            .chain(PyNode::template_variants())
            .collect();

        trace!("loading available nodes:\n{:?}", nodes);
        nodes
    }
}
impl RustNode {
    /// A node can produce any number of "templates" which will be used to populate the
    /// list of selectable new nodes that can be created.
    /// Notably, PyNode will produce a dynamic number of nodes,
    /// depending on what nodes are found in the filesystem at runtime.
    pub fn template_variants(&self) -> NodeData {
        NodeTemplate::RustNode(self.clone()).into()
    }
}
impl PyNode {
    pub fn template_variants() -> Vec<NodeData> {
        let py_nodes = load_node_names();
        py_nodes
            .into_iter()
            .map(|name| NodeTemplate::PyNode(PyNode::new(&name)).into())
            .collect()
    }
}

impl GraphNode<NodeData, PortType, PortData> for NodeData {
    fn inputs(&self) -> OrderMap<String, PortType> {
        let binary_in = [
            ("a".to_string(), PortType::Real),
            ("b".to_string(), PortType::Real),
        ]
        .into();
        let unary_in = [("a".to_string(), PortType::Real)].into();

        match &self.template {
            NodeTemplate::RustNode(rn) => match rn {
                RustNode::Identity => [("a".to_string(), PortType::Real)].into(),
                RustNode::Constant(_constant_node) => [].into(),
                RustNode::Add => binary_in,
                RustNode::Subtract => binary_in,
                RustNode::Multiply => binary_in,
                RustNode::Divide => binary_in,
                RustNode::Join => binary_in,
                RustNode::Reverse => unary_in,
                RustNode::Cos => unary_in,
                RustNode::Sin => unary_in,
                RustNode::Sinc => unary_in,
                RustNode::Linspace(_) => [].into(),
                RustNode::Plot(_) => [
                    ("x".to_string(), PortType::Real),
                    ("y".to_string(), PortType::Real),
                ]
                .into(),
                RustNode::Plot2D(_) => [("a".to_string(), PortType::Real2d)].into(),
            },
            NodeTemplate::PyNode(py_node) => py_node.ports.clone().unwrap_or_default().inputs,
        }
    }

    fn outputs(&self) -> OrderMap<String, PortType> {
        let real_out = [("out".to_string(), PortType::Real)].into();
        match &self.template {
            NodeTemplate::RustNode(rn) => match rn {
                RustNode::Identity => real_out,
                RustNode::Constant(_constant_node) => real_out,
                RustNode::Add => real_out,
                RustNode::Subtract => real_out,
                RustNode::Multiply => real_out,
                RustNode::Divide => real_out,
                RustNode::Join => real_out,
                RustNode::Reverse => real_out,
                RustNode::Cos => real_out,
                RustNode::Sin => real_out,
                RustNode::Sinc => real_out,
                RustNode::Linspace(_) => real_out,
                RustNode::Plot(_) => [].into(),
                RustNode::Plot2D(_) => [].into(),
            },
            NodeTemplate::PyNode(py_node) => py_node.ports.clone().unwrap_or_default().outputs,
        }
    }

    fn compute(
        mut self,
        inputs: OrderMap<String, &Mutex<PortData>>,
    ) -> (OrderMap<String, PortData>, Self) {
        let start = Instant::now();

        // execute compute and handle errors
        let (output, node) = match self.fallible_compute(inputs) {
            Ok(output) => (
                output,
                NodeData {
                    status: NodeStatus::Idle,
                    run_time: Some(Instant::now() - start),
                    template: self.template,
                },
            ),
            Err(node_error) => {
                log::error!("{}", node_error);
                (
                    [].into(),
                    NodeData {
                        status: NodeStatus::Error(node_error),
                        run_time: None,
                        template: self.template,
                    },
                )
            }
        };

        (output, node)
    }
}

impl GUINode for NodeTemplate {
    fn name(&self) -> String {
        match &self {
            NodeTemplate::RustNode(rn) => match rn {
                RustNode::Identity => "Identity".to_string(),
                RustNode::Constant(_value) => "Constant".to_string(),
                RustNode::Add => "Add".to_string(),
                RustNode::Subtract => "Subtract".to_string(),
                RustNode::Multiply => "Multiply".to_string(),
                RustNode::Divide => "Divide".to_string(),
                RustNode::Join => "Join".to_string(),
                RustNode::Reverse => "Reverse".to_string(),
                RustNode::Cos => "cos".to_string(),
                RustNode::Sin => "sin".to_string(),
                RustNode::Sinc => "sinc".to_string(),
                RustNode::Linspace(_linspace_config) => "Linspace".to_string(),
                RustNode::Plot(_) => "Plot".to_string(),
                RustNode::Plot2D(_) => "Plot 2D".to_string(),
            },
            NodeTemplate::PyNode(py_node) => {
                py_node.path.file_stem().unwrap().to_string_lossy().into()
            }
        }
    }

    fn view<'a>(
        &'a self,
        id: u32,
        input_data: OrderMap<String, &Mutex<PortData>>,
    ) -> (iced::Size, iced::Element<'a, Message>) {
        let dft = default_node_size();

        let operation = |s| {
            text(s)
                .font(Font::with_name("DejaVu Math TeX Gyre"))
                .size(30)
                .into()
        };
        let trig = |s| {
            text(s)
                .size(20)
                .font(Font::with_name("DejaVu Math TeX Gyre"))
                .into()
        };

        match self {
            NodeTemplate::RustNode(rn) => match rn {
                RustNode::Constant(value) => (dft, constant::view(id, *value)),
                RustNode::Linspace(linspace_config) => (dft, linspace_config.view(id)),
                RustNode::Plot(plot) => (dft * 2., plot.view(id, input_data)),
                RustNode::Plot2D(plot) => (
                    (dft.width * 2., dft.width * 2.).into(),
                    plot.view(id, input_data),
                ),
                RustNode::Add => (dft, operation("+")),
                RustNode::Subtract => (dft, operation("−")),
                RustNode::Multiply => (dft, operation("×")),
                RustNode::Divide => (dft, operation("÷")),
                RustNode::Cos => (dft, trig("cos(α)")),
                RustNode::Sin => (dft, trig("sin(α)")),
                RustNode::Sinc => (dft, trig("sinc(α)")),

                _ => (dft, text(self.name()).into()),
            },
            NodeTemplate::PyNode(_) => (dft, text(self.name()).into()),
        }
    }

    fn config_view<'a>(
        &'a self,
        id: u32,
        input_data: OrderMap<String, &Mutex<PortData>>,
    ) -> Option<iced::Element<'a, Message>> {
        match &self {
            NodeTemplate::RustNode(rn) => match rn {
                RustNode::Plot(plot) => plot.config_view(id, input_data),
                RustNode::Plot2D(plot) => plot.config_view(id, input_data),
                _ => None,
            },
            _ => None,
        }
    }
}

fn load_node_names() -> Vec<String> {
    use glob::glob;

    glob(&(env!("CARGO_MANIFEST_DIR").to_string() + "/nodes/*.py"))
        .expect("valid glob")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .collect()
}
