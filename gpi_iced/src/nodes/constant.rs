use crate::{app::Message, node_data::NodeData};
use iced::{
    widget::{column, container, slider, text},
    Alignment::Center,
    Element,
    Length::Fill,
};

pub fn view<'a>(id: u32, value: f64) -> Element<'a, Message> {
    container(
        column![
            text(format!("{value:.1}")),
            slider(0.1..=20., value, move |value| {
                Message::UpdateNodeData(id, NodeData::Constant(value))
            })
            .step(0.1)
            .width(Fill),
        ]
        .align_x(Center)
        .padding([0., 10.]),
    )
    .center_y(Fill)
    .align_right(Fill)
    .into()
}
