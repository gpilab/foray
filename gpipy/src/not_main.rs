use std::error::Error;

use gpipy::pyo;
use gpipy::python_node::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = std::env::current_dir().unwrap();
    path.push("python_plugin");
    let _ = pyo::initialize_gpipy(&path);
    let result = pyo::gpipy_compute("add_int", vec![Value::Integer(1), Value::Integer(3)]);

    match result {
        Ok(val) => println!("Success!\nvalue: {:?}", val),
        Err(e) => {
            println!("Failure :(\nerror: {}", e)
        }
    };
    Ok(())
}
