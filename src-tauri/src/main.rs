// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rustfft::{num_complex::Complex, FftPlanner};
// use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    tauri::Builder::default()
        // .setup(|app| {
        //     //let window = app.get_window("main").unwrap();
        //
        //     // #[cfg(target_os = "macos")]
        //     // apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        //     //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
        //     //
        //     // #[cfg(target_os = "windows")]
        //     // apply_blur(&window, Some((18, 18, 18, 125)))
        //     //     .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
        //
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![greet, fft, hello_backend])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn fft(signal: Vec<f32>) -> Vec<f32> {
    // Convert the normalized signal to complex numbers
    let mut input: Vec<Complex<f32>> = signal
        .into_iter()
        .map(|x| Complex { re: x, im: 0.0 })
        .collect();

    // Plan and compute the FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(input.len());
    fft.process(&mut input);

    // Calculate magnitudes of the FFT output
    let res: Vec<f32> = input.iter().map(|c| c.norm()).collect();
    let len = res.len();

    let max_value = res.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    res.into_iter()
        .map(|x| x / max_value)
        .take(len / 2)
        .collect()
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn hello_backend() -> String {
    "Hello from the backend!".to_string()
}
