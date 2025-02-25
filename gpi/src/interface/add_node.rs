use crate::gui_node::GUINode;
use crate::nodes::NodeData;
use crate::{app::Message, style};
use iced::*;
use widget::{column, *};

use super::node::NODE_RADIUS;

pub(crate) fn node_list_view<'a>(templates: &[NodeData]) -> Element<'a, Message> {
    container(container(
        column(templates.iter().map(|node| {
            button(
                row![
                    horizontal_space().width(10.),
                    container(text(node.template.name())).padding(4.0),
                    horizontal_rule(0.0),
                    container(vertical_rule(1.0)).height(30.),
                ]
                .align_y(Center),
            )
            .padding(0.)
            .on_press(Message::AddNode(node.clone().into()))
            .width(Fill)
            .style(style::button::list)
            .into()
        }))
        .width(Fill),
    ))
    .center_x(Fill)
    .into()
}

pub fn add_node_panel(available_nodes: &[NodeData]) -> Element<'_, Message> {
    let node_list = node_list_view(available_nodes);
    container(column![
        container(text("New Node").size(20.))
            .center_x(Fill)
            .padding(5.),
        horizontal_rule(3.0),
        container(scrollable(node_list).spacing(2.)).padding([0.0, 2.0]),
    ])
    .style(|t| container::Style {
        border: border::rounded(NODE_RADIUS),
        ..container::bordered_box(t)
    })
    .width(500.)
    .height(400.)
    .into()
}
