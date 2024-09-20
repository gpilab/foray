//use rustfft::{num_complex::Complex, FftPlanner};

//#[tauri::command]
//pub fn py_add(a: PortValue, b: PortValue) -> PortValue {
//    let res = gpipy_compute("add_int", hash![a.into(), b.into()]);
//    match res {
//        Ok(pv) => pv,
//        Err(e) => panic!("Unexpected return value from python! {}", e),
//    }
//}
//
//#[tauri::command]
//pub fn py_add_array(a: PortValue, b: PortValue) -> PortValue {
//    let res = gpipy_compute("add_int_array", vec![a.into(), b.into()]);
//
//    match res {
//        Ok(pv) => pv,
//        Err(e) => panic!("Unexpected return value from python! {}", e),
//    }
//}

//#[tauri::command]
//pub fn fft(signal: Vec<f32>) -> Vec<f32> {
//    // Convert the normalized signal to complex numbers
//    let mut input: Vec<Complex<f32>> = signal
//        .into_iter()
//        .map(|x| Complex { re: x, im: 0.0 })
//        .collect();
//
//    // Plan and compute the FFT
//    let mut planner = FftPlanner::new();
//    let fft = planner.plan_fft_forward(input.len());
//    fft.process(&mut input);
//
//    // Calculate magnitudes of the FFT output
//    let res: Vec<f32> = input.iter().map(|c| c.norm()).collect();
//    let len = res.len();
//
//    let max_value = res.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
//    res.into_iter()
//        .map(|x| x / max_value)
//        .take(len / 2)
//        .collect()
//}
