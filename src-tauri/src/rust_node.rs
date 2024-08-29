use gpi_lib::pyo::{gpipy_compute, Value};
use rustfft::{num_complex::Complex, FftPlanner};

#[tauri::command]
pub fn py_add(a: i32, b: i32) -> i32 {
    let res = gpipy_compute("add_int", vec![Value::Integer(a), Value::Integer(b)]).unwrap();
    match res {
        Value::Integer(v) => v,
        _ => panic!("Unexpected return value from python"),
    }
}

#[tauri::command]
pub fn py_add_array(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let res = gpipy_compute("add_int_array", vec![Value::Vec1(a), Value::Vec1(b)]).unwrap();
    match res {
        Value::Vec1(v) => v,
        _ => panic!("Unexpected return value from python!"),
    }
}

#[tauri::command]
pub fn fft(signal: Vec<f32>) -> Vec<f32> {
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
