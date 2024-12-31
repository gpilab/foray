use crate::{app::Message, node_data::NodeData};
use iced::{
    widget::{container, row, slider, text},
    Alignment::Center,
    Element,
    Length::Fill,
};

pub fn view<'a>(id: u32, value: f64) -> Element<'a, Message> {
    container(
        row![
            text(value),
            slider(-100.0..=100., value, move |value| {
                Message::UpdateNodeData(id, NodeData::Constant(value))
            })
            .width(40.),
        ]
        .align_y(Center)
        .spacing(10.)
        .padding([0., 10.]),
    )
    .center_y(Fill)
    .align_right(Fill)
    .into()
}
