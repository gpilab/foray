// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use gpi_lib::pyo;
use tauri::api::cli::Matches;

mod commands;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            parse_cli(app.get_cli_matches().unwrap());
            initialize_python();
            // TODO: use path supplied from config, or have core nodes already installed in python
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::fft,
            commands::hello_backend,
            commands::py_add,
            commands::py_add_array,
            commands::dynamic_command
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn parse_cli(matches: Matches) {
    if let Some(_help) = matches.args.get("help") {
        println!("help called");
    }
}

fn initialize_python() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../gpi_lib/python_plugin/");
    let _ = pyo::initialize_gpipy(&d);
}
