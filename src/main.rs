use std::{
    env,
    ffi::OsString,
    iter,
    path::{Path, PathBuf},
};

use glob::glob;
use gpi::app::{subscriptions, theme, App};
use iced::{application, Font};
use itertools::Itertools;
use log::warn;
use pyo3::{types::PyAnyMethods, PyResult, Python};
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    set_environment();

    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    env_logger::init();

    pyo3::prepare_freethreaded_python();

    // configure python to close the program when
    // SIGINT (ctrl-c) is received
    let _ = Python::with_gil(|py| -> PyResult<()> {
        let signal = py.import("signal")?;
        signal
            .getattr("signal")?
            .call1((signal.getattr("SIGINT")?, signal.getattr("SIG_DFL")?))?;
        Ok(())
    });

    application("gpi_v2", App::update, App::view)
        .subscription(subscriptions)
        .theme(theme)
        .window(iced::window::Settings {
            min_size: Some((400., 300.).into()),
            //icon: Some(iced::window::icon::from_rgba(vec![0u8; (32 * 32) * 4], 32, 32).unwrap()),
            ..Default::default()
        })
        .antialiasing(true)
        .window_size((1000., 800.))
        .decorations(true)
        .scale_factor(|_| 1.25)
        .font(include_bytes!("../data/DejaVuMathTeXGyre.ttf").as_slice()) // "DejaVu Math TeX Gyre"
        .font(include_bytes!("../data/CaskaydiaCoveNerdFont.ttf").as_slice())
        .font(include_bytes!("../data/CaskaydiaCove.ttf").as_slice())
        .default_font(Font::with_name("CaskaydiaCove"))
        .run()
}

#[derive(Serialize, Deserialize)]
struct Config {
    venv_dir: PathBuf,
}

fn set_environment() {
    let default_venv = PathBuf::from("/home/john/projects/gpi_v2/scratch/.venv/");
    let default_bin = default_venv.join("bin");
    let default_py = default_bin.join("python");
    env::set_var("VIRTUAL_ENV", &default_venv);
    env::set_var("PYO3_PYTHON", &default_py);

    env::set_var(
        "Path",
        prepend_env("PATH", default_bin).unwrap().to_str().unwrap(),
    );

    // Set PYTHONPATH to appropriate paths in the venv directory
    // needed to address open pyo3 issue https://github.com/PyO3/pyo3/issues/1741
    if let Ok(paths) = glob(
        default_venv
            .join("lib/python3*")
            .to_str()
            .expect("valid python virtual environment directory"),
    ) {
        let paths: Vec<_> = paths.filter_map(|p| p.ok()).collect();
        if paths.len() > 1 {
            warn!("Multiple python versions detected in venv {default_venv:?}, this has not been tested. Unexpected results may occur")
        }
        paths.into_iter().for_each(|path| {
            dbg!(&path);
            env::set_var(
                "PYTHONPATH",
                prepend_env("PYTHONPATH", path.join("site-packages"))
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );
        });
    }
}

/// Create a new env string that has the given value prepended
fn prepend_env<P: AsRef<Path>>(env: &str, p: P) -> Result<OsString, env::JoinPathsError> {
    let new_path = p.as_ref();
    if let Some(path) = env::var_os(env) {
        let old = env::split_paths(&path);
        Ok(env::join_paths(iter::once(new_path.to_owned()).chain(old))?)
    } else {
        Ok(new_path.into())
    }
}
