use crate::app::Message;
use crate::graph::GraphNode;
use crate::nodes::linspace::LinspaceConfig;
use crate::nodes::math_nodes::{binary_operation, unary_operation};
use crate::nodes::plot::Plot;
use crate::nodes::{constant, default_node_size, GUINode, PortData, PortType};
use crate::OrderMap;
use iced::widget::text;
use iced::Font;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, VariantNames};

#[derive(Clone, Debug, EnumIter, VariantNames, Serialize, Deserialize)]
pub enum NodeData {
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
    Join,
    Reverse,
}

impl GraphNode<NodeData, PortType, PortData> for NodeData {
    fn inputs(&self) -> OrderMap<String, PortType> {
        let binary_in = [
            ("a".to_string(), PortType::Real),
            ("b".to_string(), PortType::Real),
        ]
        .into();
        let unary_in = [("a".to_string(), PortType::Real)].into();

        match self {
            NodeData::Identity => [("a".to_string(), PortType::Real)].into(),
            NodeData::Constant(_constant_node) => [].into(),
            NodeData::Add => binary_in,
            NodeData::Subtract => binary_in,
            NodeData::Multiply => binary_in,
            NodeData::Divide => binary_in,
            NodeData::Join => binary_in,
            NodeData::Reverse => unary_in,
            NodeData::Cos => unary_in,
            NodeData::Sin => unary_in,
            NodeData::Sinc => unary_in,
            NodeData::Linspace(_) => [].into(),
            NodeData::Plot(_) => [
                ("x".to_string(), PortType::Real),
                ("y".to_string(), PortType::Real),
            ]
            .into(),
        }
    }

    fn outputs(&self) -> OrderMap<String, PortType> {
        let real_out = [("out".to_string(), PortType::Real)].into();
        match self {
            NodeData::Identity => real_out,
            NodeData::Constant(_constant_node) => real_out,
            NodeData::Add => real_out,
            NodeData::Subtract => real_out,
            NodeData::Multiply => real_out,
            NodeData::Divide => real_out,
            NodeData::Join => real_out,
            NodeData::Reverse => real_out,
            NodeData::Cos => real_out,
            NodeData::Sin => real_out,
            NodeData::Sinc => real_out,
            NodeData::Linspace(_) => real_out,
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
            NodeData::Subtract => binary_operation(inputs, Box::new(|a, b| a - b)),
            NodeData::Multiply => binary_operation(inputs, Box::new(|a, b| a * b)),
            NodeData::Divide => binary_operation(inputs, Box::new(|a, b| a / b)),
            NodeData::Join => binary_operation(
                inputs,
                Box::new(|a, b| a.iter().chain(b).copied().collect()),
            ),
            NodeData::Reverse => {
                unary_operation(inputs, Box::new(|a| a.iter().rev().copied().collect()))
            }
            NodeData::Cos => unary_operation(inputs, Box::new(|a| a.cos())),
            NodeData::Sin => unary_operation(inputs, Box::new(|a| a.sin())),
            NodeData::Sinc => unary_operation(
                inputs,
                Box::new(|a| {
                    a.map(|x| match x {
                        0. => 1.,
                        _ => x.sin() / x,
                    })
                }),
            ),
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
            NodeData::Join => "Join".to_string(),
            NodeData::Reverse => "Reverse".to_string(),
            NodeData::Cos => "cos".to_string(),
            NodeData::Sin => "sin".to_string(),
            NodeData::Sinc => "sinc".to_string(),
            NodeData::Linspace(_linspace_config) => "Linspace".to_string(),
            NodeData::Plot(_) => "Plot".to_string(),
        }
    }

    fn view<'a>(
        &'a self,
        id: u32,
        input_data: Option<OrderMap<String, &std::cell::RefCell<PortData>>>,
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
            NodeData::Constant(value) => (dft, constant::view(id, *value)),
            NodeData::Linspace(linspace_config) => (dft, linspace_config.view(id)),
            NodeData::Plot(plot) => (dft * 2., plot.view(id, input_data)),
            NodeData::Add => (dft, operation("+")),
            NodeData::Subtract => (dft, operation("−")),
            NodeData::Multiply => (dft, operation("×")),
            NodeData::Divide => (dft, operation("÷")),
            NodeData::Cos => (dft, trig("cos(α)")),
            NodeData::Sin => (dft, trig("sin(α)")),
            NodeData::Sinc => (dft, trig("sinc(α)")),
            _ => (dft, text(self.name()).into()),
        }
    }

    fn config_view<'a>(
        &'a self,
        id: u32,
        input_data: Option<OrderMap<String, &std::cell::RefCell<PortData>>>,
    ) -> Option<iced::Element<'a, Message>> {
        match self {
            NodeData::Plot(plot) => plot.config_view(id, input_data),
            _ => None,
        }
    }
}
