use crate::app::Message;
use crate::graph::GraphNode;
use crate::nodes::linspace::LinspaceConfig;
use crate::nodes::math_nodes::binary_operation;
use crate::nodes::plot::Plot;
use crate::nodes::{constant, GUINode, PortData, PortType};
use iced::widget::text;
use ordermap::OrderMap;
use strum::{EnumIter, VariantNames};

#[derive(Clone, Debug, EnumIter, VariantNames)]
pub enum NodeData {
    Identity,
    Constant(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Linspace(LinspaceConfig),
    Plot(Plot),
}

impl GraphNode<NodeData, PortType, PortData> for NodeData {
    fn inputs(&self) -> OrderMap<String, PortType> {
        match self {
            NodeData::Identity => [("a".to_string(), PortType::Real)].into(),
            NodeData::Constant(_constant_node) => [].into(),
            NodeData::Add => [
                ("a".to_string(), PortType::Real),
                ("b".to_string(), PortType::Real),
            ]
            .into(),
            NodeData::Subtract => [
                ("a".to_string(), PortType::Real),
                ("b".to_string(), PortType::Real),
            ]
            .into(),
            NodeData::Multiply => [
                ("a".to_string(), PortType::Real),
                ("b".to_string(), PortType::Real),
            ]
            .into(),
            NodeData::Divide => [
                ("a".to_string(), PortType::Real),
                ("b".to_string(), PortType::Real),
            ]
            .into(),
            NodeData::Linspace(_) => [].into(),
            NodeData::Plot(_) => [
                ("x".to_string(), PortType::Real),
                ("y".to_string(), PortType::Real),
            ]
            .into(),
        }
    }

    fn outputs(&self) -> OrderMap<String, PortType> {
        match self {
            NodeData::Identity => [("out".to_string(), PortType::Real)].into(),
            NodeData::Constant(_constant_node) => [("out".to_string(), PortType::Real)].into(),
            NodeData::Add => [("out".to_string(), PortType::Real)].into(),
            NodeData::Subtract => [("out".to_string(), PortType::Real)].into(),
            NodeData::Multiply => [("out".to_string(), PortType::Real)].into(),
            NodeData::Divide => [("out".to_string(), PortType::Real)].into(),
            NodeData::Linspace(_) => [("out".to_string(), PortType::Real)].into(),
            NodeData::Plot(_) => [].into(),
        }
    }

    fn compute(
        &self,
        inputs: OrderMap<String, &std::cell::RefCell<PortData>>,
    ) -> OrderMap<String, PortData> {
        match self {
            NodeData::Identity => [("out".to_string(), inputs["a"].borrow().clone())].into(),
            NodeData::Constant(value) => {
                [("out".to_string(), PortData::Real(vec![*value].into()))].into()
            }
            NodeData::Add => binary_operation(inputs, Box::new(|a, b| a + b)),
            NodeData::Subtract => binary_operation(inputs, Box::new(|a, b| a + b)),
            NodeData::Multiply => binary_operation(inputs, Box::new(|a, b| a + b)),
            NodeData::Divide => binary_operation(inputs, Box::new(|a, b| a + b)),
            NodeData::Linspace(linspace_config) => linspace_config.compute(inputs),
            NodeData::Plot(_) => [].into(),
        }
    }
}

impl GUINode for NodeData {
    fn name(&self) -> String {
        match self {
            NodeData::Identity => "Identity".to_string(),
            NodeData::Constant(_value) => "Constant".to_string(),
            NodeData::Add => "Add".to_string(),
            NodeData::Subtract => "Subtract".to_string(),
            NodeData::Multiply => "Multiply".to_string(),
            NodeData::Divide => "Divide".to_string(),
            NodeData::Linspace(_linspace_config) => "Linspace".to_string(),
            NodeData::Plot(_) => "Plot".to_string(),
        }
    }

    fn view<'a>(
        &'a self,
        id: u32,
        input_data: Option<OrderMap<String, &std::cell::RefCell<PortData>>>,
    ) -> iced::Element<'a, Message> {
        match self {
            NodeData::Constant(value) => constant::view(id, *value),
            NodeData::Linspace(linspace_config) => linspace_config.view(id),
            NodeData::Plot(plot) => plot.view(id, input_data),
            NodeData::Add => text("+").size(20).into(),
            NodeData::Subtract => text("-").size(20).into(),
            NodeData::Multiply => text("*").size(20).into(),
            NodeData::Divide => text("/").size(20).into(),
            _ => text(self.name()).into(),
        }
    }
}
