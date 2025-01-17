pub mod node;
pub mod side_bar;
pub mod theme_config;
pub mod wire;

pub const SEPERATOR: f32 = 1.0;

pub fn debug_format(
    debug: bool,
    default_text: &dyn derive_more::Display,
    debug_info: &dyn derive_more::Debug,
) -> String {
    match debug {
        true => format!("{default_text}{debug_info:?}"),
        false => format!("{default_text}"),
    }
}
