use gpi_iced::app::{subscriptions, theme, App};
use iced::{application, Font};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    application("gpi_v2", App::update, App::view)
        .subscription(subscriptions)
        .antialiasing(true)
        .theme(theme)
        .window(iced::window::Settings {
            min_size: Some((400., 300.).into()),
            //icon: Some(icon::from_rgba(vec![0u8; (32 * 32) * 4], 32, 32).unwrap()),
            ..Default::default()
        })
        .window_size((1000., 800.))
        .decorations(true)
        .scale_factor(|_| 1.25)
        .font(include_bytes!("../data/Cantarell-VF.otf").as_slice()) // "Canterell"
        .font(include_bytes!("../data/DejaVuMathTeXGyre.ttf").as_slice()) // "DejaVu Math TeX Gyre"
        .default_font(Font::with_name("Canterell"))
        .run()
}
