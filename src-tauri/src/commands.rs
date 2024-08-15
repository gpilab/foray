use gpi_lib::pyo::{gpipy_compute, Value};
use rustfft::{num_complex::Complex, FftPlanner};
use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct DynamicMessage<'a> {
    /// name of the python node module. i.e. "add_int"
    node_type: &'a str,
    /// list of input `Values` to pass to `node_type`'s python "compute" function
    inputs: Vec<Value>,
    //output: Vec<Value>,
}

#[tauri::command]
pub fn dynamic_command(message: DynamicMessage) -> Value {
    println!(
        "node type: {}, inputs: {:?}",
        message.node_type, message.inputs
    );
    let res = gpipy_compute(message.node_type, message.inputs).unwrap();
    match res {
        Value::Other(v) => panic!("Unexpected return value from python: {}", v),
        _ => res,
    }
}

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
        _ => panic!("Unexpected return value from python"),
    }
}
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
pub fn hello_backend() -> String {
    "Hello from the backend!".to_string()
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use gpi_lib::pyo::initialize_gpipy;

    use super::*;
    #[test]
    fn add_int() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("../gpi_lib/python_plugin/");
        let _ = initialize_gpipy(&d);
        let _result = dynamic_command(DynamicMessage {
            node_type: "add_int",
            inputs: vec![Value::Integer(1), Value::Integer(3)],
        });
        matches!(Value::Integer(4), _result);
    }
}
