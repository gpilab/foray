use iced::{widget::text, Element};

use crate::app::Message;

pub fn view<'a>(id: u32) -> Element<'a, Message> {
    text("plot").into()
}
