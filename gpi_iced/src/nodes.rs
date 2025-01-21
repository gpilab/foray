use std::time::{Duration, Instant};

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
pub enum NodeTemplate {
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
        &self,
        inputs: OrderMap<String, &std::cell::RefCell<PortData>>,
    ) -> Result<OrderMap<String, PortData>, NodeError> {
        Ok(match &self.template {
            NodeTemplate::Identity => [(
                "out".to_string(),
                inputs
                    .get("a")
                    .ok_or(NodeError::Input("input 'a' not found".to_string()))?
                    .borrow()
                    .clone(),
            )]
            .into(),
            NodeTemplate::Constant(value) => {
                [("out".to_string(), PortData::Real(vec![*value].into()))].into()
            }
            NodeTemplate::Add => binary_operation(inputs, Box::new(|a, b| a + b))?,
            NodeTemplate::Subtract => binary_operation(inputs, Box::new(|a, b| a - b))?,
            NodeTemplate::Multiply => binary_operation(inputs, Box::new(|a, b| a * b))?,
            NodeTemplate::Divide => binary_operation(inputs, Box::new(|a, b| a / b))?,
            NodeTemplate::Join => binary_operation(
                inputs,
                Box::new(|a, b| a.iter().chain(b).copied().collect()),
            )?,
            NodeTemplate::Reverse => {
                unary_operation(inputs, Box::new(|a| a.iter().rev().copied().collect()))?
            }
            NodeTemplate::Cos => unary_operation(inputs, Box::new(|a| a.cos()))?,
            NodeTemplate::Sin => unary_operation(inputs, Box::new(|a| a.sin()))?,
            NodeTemplate::Sinc => unary_operation(
                inputs,
                Box::new(|a| {
                    a.map(|x| match x {
                        0. => 1.,
                        _ => x.sin() / x,
                    })
                }),
            )?,
            NodeTemplate::Linspace(linspace_config) => linspace_config.compute(inputs),
            NodeTemplate::Plot(_) => [].into(),
            NodeTemplate::Plot2D(_) => [].into(),
            NodeTemplate::PyNode(py_node) => py_node.compute(inputs)?,
        })
    }

    pub fn available_nodes() -> Vec<NodeData> {
        let nodes = NodeTemplate::iter()
            .flat_map(|template| template.template_variants())
            .collect();
        trace!("loading available nodes:\n{:?}", nodes);
        nodes
    }
}
impl NodeTemplate {
    /// A node can produce any number of "templates" which will be used to populate the
    /// list of selectable new nodes that can be created.
    /// Notably, PyNode will produce a dynamic number of nodes,
    /// depending on what nodes are found in the filesystem at runtime.
    pub fn template_variants(&self) -> Vec<NodeData> {
        match &self {
            NodeTemplate::PyNode(_) => {
                let py_nodes = load_node_names();
                py_nodes
                    .into_iter()
                    .map(|name| NodeTemplate::PyNode(PyNode::new(&name)).into())
                    .collect()
            }
            _ => vec![self.clone().into()],
        }
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
            NodeTemplate::Identity => [("a".to_string(), PortType::Real)].into(),
            NodeTemplate::Constant(_constant_node) => [].into(),
            NodeTemplate::Add => binary_in,
            NodeTemplate::Subtract => binary_in,
            NodeTemplate::Multiply => binary_in,
            NodeTemplate::Divide => binary_in,
            NodeTemplate::Join => binary_in,
            NodeTemplate::Reverse => unary_in,
            NodeTemplate::Cos => unary_in,
            NodeTemplate::Sin => unary_in,
            NodeTemplate::Sinc => unary_in,
            NodeTemplate::Linspace(_) => [].into(),
            NodeTemplate::Plot(_) => [
                ("x".to_string(), PortType::Real),
                ("y".to_string(), PortType::Real),
            ]
            .into(),
            NodeTemplate::Plot2D(_) => [("a".to_string(), PortType::Real2d)].into(),
            NodeTemplate::PyNode(py_node) => py_node.ports.clone().unwrap_or_default().inputs,
        }
    }

    fn outputs(&self) -> OrderMap<String, PortType> {
        let real_out = [("out".to_string(), PortType::Real)].into();
        match &self.template {
            NodeTemplate::Identity => real_out,
            NodeTemplate::Constant(_constant_node) => real_out,
            NodeTemplate::Add => real_out,
            NodeTemplate::Subtract => real_out,
            NodeTemplate::Multiply => real_out,
            NodeTemplate::Divide => real_out,
            NodeTemplate::Join => real_out,
            NodeTemplate::Reverse => real_out,
            NodeTemplate::Cos => real_out,
            NodeTemplate::Sin => real_out,
            NodeTemplate::Sinc => real_out,
            NodeTemplate::Linspace(_) => real_out,
            NodeTemplate::Plot(_) => [].into(),
            NodeTemplate::Plot2D(_) => [].into(),
            NodeTemplate::PyNode(py_node) => py_node.ports.clone().unwrap_or_default().outputs,
        }
    }

    fn compute(
        self,
        inputs: OrderMap<String, &std::cell::RefCell<PortData>>,
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
            NodeTemplate::Identity => "Identity".to_string(),
            NodeTemplate::Constant(_value) => "Constant".to_string(),
            NodeTemplate::Add => "Add".to_string(),
            NodeTemplate::Subtract => "Subtract".to_string(),
            NodeTemplate::Multiply => "Multiply".to_string(),
            NodeTemplate::Divide => "Divide".to_string(),
            NodeTemplate::Join => "Join".to_string(),
            NodeTemplate::Reverse => "Reverse".to_string(),
            NodeTemplate::Cos => "cos".to_string(),
            NodeTemplate::Sin => "sin".to_string(),
            NodeTemplate::Sinc => "sinc".to_string(),
            NodeTemplate::Linspace(_linspace_config) => "Linspace".to_string(),
            NodeTemplate::Plot(_) => "Plot".to_string(),
            NodeTemplate::Plot2D(_) => "Plot 2D".to_string(),
            NodeTemplate::PyNode(py_node) => {
                py_node.path.file_stem().unwrap().to_string_lossy().into()
            }
        }
    }

    fn view<'a>(
        &'a self,
        id: u32,
        input_data: OrderMap<String, &std::cell::RefCell<PortData>>,
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
            NodeTemplate::Constant(value) => (dft, constant::view(id, *value)),
            NodeTemplate::Linspace(linspace_config) => (dft, linspace_config.view(id)),
            NodeTemplate::Plot(plot) => (dft * 2., plot.view(id, input_data)),
            NodeTemplate::Plot2D(plot) => (
                (dft.width * 2., dft.width * 2.).into(),
                plot.view(id, input_data),
            ),
            NodeTemplate::Add => (dft, operation("+")),
            NodeTemplate::Subtract => (dft, operation("−")),
            NodeTemplate::Multiply => (dft, operation("×")),
            NodeTemplate::Divide => (dft, operation("÷")),
            NodeTemplate::Cos => (dft, trig("cos(α)")),
            NodeTemplate::Sin => (dft, trig("sin(α)")),
            NodeTemplate::Sinc => (dft, trig("sinc(α)")),
            _ => (dft, text(self.name()).into()),
        }
    }

    fn config_view<'a>(
        &'a self,
        id: u32,
        input_data: OrderMap<String, &std::cell::RefCell<PortData>>,
    ) -> Option<iced::Element<'a, Message>> {
        match &self {
            NodeTemplate::Plot(plot) => plot.config_view(id, input_data),
            NodeTemplate::Plot2D(plot) => plot.config_view(id, input_data),
            _ => None,
        }
    }
}

fn load_node_names() -> Vec<String> {
    use glob::glob;
    glob("[!.]*/*.py")
        .expect("valid glob")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .collect()
}
