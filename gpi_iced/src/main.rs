use gpi_iced::app::App;
use iced::{application, Font, Theme};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    application("gpi_v2", App::update, App::view)
        .antialiasing(true)
        .theme(theme)
        .window_size((800., 600.))
        .decorations(true)
        .font(include_bytes!("../data/Cantarell-VF.otf").as_slice()) // "Canterell"
        .font(include_bytes!("../data/DejaVuMathTeXGyre.ttf").as_slice()) // "DejaVu Math TeX Gyre"
        .default_font(Font::with_name("Canterell"))
        .run()
}

fn theme(_state: &App) -> Theme {
    Theme::Ferra
}
