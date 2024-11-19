use std::time::{self, Duration};

use perf_playground::{test_fft, test_ndarray::run};
fn main() {
    copy_bench();
}

fn fft_bench() {
    let num_runs = 100;
    let total_duration: Duration = (0..num_runs).map(|_| test_fft::run()).sum();
    let s_per_run = total_duration.as_secs_f64() / num_runs as f64;
    println!("ms per run: {}", s_per_run * 1000.)
}

fn copy_bench() {
    // 0 to a billion elements
    (0..10).map(|i| 10usize.pow(i)).for_each(|len| {
        let element_bytes = 4;
        let total_bytes = len * element_bytes;
        let image_size = 300 * 300 * 4;

        println!("{}", human_readable_size(total_bytes));
        println!("\tnum 300x300 images:{}", total_bytes / image_size);

        let start = time::Instant::now();

        let a = vec![0.0f32; len];
        let _b = a.clone();

        let duration = start.elapsed();

        println!("\tcopy duration: {:?}", duration,);
        println!(
            "\tms per image: {:?}",
            (duration.as_secs_f64() * 1000.) / (total_bytes as f64 / (image_size as f64))
        );
        println!(
            "\t100 image fps {}",
            (total_bytes as f64 / (image_size as f64 * 100.)) / (duration.as_secs_f64())
        );
    });
}

fn human_readable_size(value: usize) -> String {
    let mag = (value as f64).log10();
    match mag.floor() as usize / 3 {
        0 => format!("{} bytes", value),
        1 => format!("{} kB", value as f32 / (10.0f32.powi(3))),
        2 => format!("{} MB", value as f32 / (10.0f32.powi(6))),
        3 => format!("{} GB", value as f32 / (10.0f32.powi(9))),
        4 => format!("{} TB", value as f32 / (10.0f32.powi(12))),
        5 => format!("{} PB", value as f32 / (10.0f32.powi(15))),
        _ => panic!("holy smokes!"),
    }
}
