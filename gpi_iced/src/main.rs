use gpi_iced::app::App;
use iced::{application, Theme};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    application("gpi_v2", App::update, App::view)
        .antialiasing(true)
        .theme(theme)
        .window_size((800., 600.))
        .decorations(true)
        .run()
}

fn theme(_state: &App) -> Theme {
    Theme::Ferra
}
