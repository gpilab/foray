use iced::{
    widget::{button, text},
    Element,
};

use crate::{app::Message, SYMBOL_FONT};

pub mod node;
pub mod side_bar;
pub mod theme_config;
pub mod wire;

pub const SEPERATOR: f32 = 1.0;
pub fn char_icon_button<'a>(icon: String, enabled: bool, message: Message) -> Element<'a, Message> {
    button(text(icon).font(SYMBOL_FONT))
        .on_press_maybe(if enabled { Some(message) } else { None })
        //.padding([1.0, 4.0])
        .style(button::text)
        .into()
}

pub fn debug_format(
    debug: bool,
    default_text: &dyn derive_more::Display,
    debug_info: &dyn derive_more::Debug,
) -> String {
    match debug {
        true => format!("{default_text} ({debug_info:?})"),
        false => format!("{default_text}"),
    }
}
