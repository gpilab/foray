use gpi_lib::pyo;

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use pyo::{gpipy_compute, initialize_gpipy, Value};

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("python_plugin/");
            let _ = initialize_gpipy(&d);
        });
    }

    #[test]
    fn add_int() {
        initialize();

        let _resut = match gpipy_compute("add_int", vec![Value::Integer(1), Value::Integer(3)]) {
            Ok(Value::Integer(res)) => res,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert!(matches!(Value::Integer(4), _result));
    }

    #[test]
    fn add_array() {
        initialize();
        let result = gpipy_compute(
            "add_int_array",
            vec![
                Value::Vec1(vec![1.0, 2.0, 3.0]),
                Value::Vec1(vec![1.0, 2.0, 3.0]),
            ],
        );

        let val = match result {
            Ok(Value::Vec1(v)) => v,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert!(do_vecs_match(&val, &vec![2.0, 4.0, 6.0]))
    }

    #[test]
    fn load_numpy() {
        initialize();
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/simple.npy");
        let npy_path_in = Value::String(d.to_str().unwrap().into());

        let result = gpipy_compute("read_np", vec![npy_path_in]);
        let val = match result {
            Ok(Value::Vec1(val)) => val,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let _expected = vec![1.0, 2.0, 3.0];
        assert!(do_vecs_match(&val, &_expected));
    }

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
