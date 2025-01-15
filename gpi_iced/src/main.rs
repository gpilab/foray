use gpi_iced::app::{subscriptions, theme, App};
use iced::{application, Font};

pub fn main() -> iced::Result {
    assert_environment();

    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    env_logger::init();

    pyo3::prepare_freethreaded_python();

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
        .font(include_bytes!("../data/DejaVuMathTeXGyre.ttf").as_slice()) // "DejaVu Math TeX Gyre"
        .font(include_bytes!("../data/CaskaydiaCoveNerdFont.ttf").as_slice())
        .font(include_bytes!("../data/CaskaydiaCove.ttf").as_slice())
        .default_font(Font::with_name("CaskaydiaCove"))
        .run()
}

/// Ensure that environment variables are correctly set for the python venv
/// located at `nodes/.venv`
//
/// The build script `build.rs` is repsonsible for setting these variables
///
/// This will need to be updated when the location of the user's venv will
/// be located
fn assert_environment() {
    let venv_dir = env!("CARGO_MANIFEST_DIR").to_string() + "/nodes/.venv";
    let venv_bin = venv_dir.clone() + "/bin";
    let python_bin = venv_bin.clone() + "/python";

    let env_path = env!("PATH");
    assert!(env_path.contains(&venv_bin));
    let env_venv = env!("VIRTUAL_ENV");
    assert_eq!(env_venv, venv_dir);
    let env_python_bin = env!("PYO3_PYTHON");
    assert_eq!(env_python_bin, python_bin);
}
