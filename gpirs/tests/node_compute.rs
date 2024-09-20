#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use gpirs::{
        node::NodeInputValue,
        port::PortValue,
        pyo::{gpipy_compute, initialize_gpipy},
    };

    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
            //let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let _ = initialize_gpipy();
        });
    }
    /// gets a path object from a string representing apath relative
    /// to `gpipy/`
    fn get_path(relative_path: &str) -> PathBuf {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(relative_path);
        d
    }

    #[test]
    fn add_int() {
        initialize();
        let inputs = NodeInputValue(HashMap::from([
            ("a".into(), PortValue::Integer(1)),
            ("b".into(), PortValue::Integer(3)),
        ]));
        let _result = match gpipy_compute("add_int", inputs) {
            //TODO: create helper function in node to create in/out objects
            Ok(res) => res,
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert_eq!(PortValue::Integer(4), _result.get_first());
        assert_ne!(PortValue::Integer(1), _result.get_first());
    }

    #[test]
    fn add_array() {
        initialize();
        let inputs = NodeInputValue::new(vec![
            ("a".into(), vec![1, 2, 3].into()),
            ("b".into(), vec![1, 2, 3].into()),
        ]);
        let result = gpipy_compute("add_int_array", inputs);

        let val = match result {
            Ok(v) => v,
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output = vec![2, 4, 6].into();
        assert_eq!(val.get_first(), expected_output);
    }

    #[test]
    fn load_numpy() {
        initialize();
        let inputs = NodeInputValue::new(vec![(
            "a".into(),
            PortValue::String(get_path("tests/simple.npy").to_str().unwrap().into()),
        )]);

        let result = gpipy_compute("read_np", inputs);
        let val = match result {
            Ok(v) => v,
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output = vec![1.0, 2.0, 3.0].into();
        assert_eq!(val.get_first(), expected_output);
    }
    #[test]
    fn load_numpy_2d() {
        initialize();
        let inputs = NodeInputValue::new(vec![(
            "a".into(),
            PortValue::String(get_path("tests/2dArray.npy").to_str().unwrap().into()),
        )]);

        let result = gpipy_compute("read_np", inputs);
        let val = match result {
            Ok(val) => val,
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output = PortValue::Array(vec![vec![1.0, 2.0].into(), vec![3.0, 4.0].into()]);

        assert_eq!(val.get_first(), expected_output);
    }

    #[test]
    fn load_numpy_4d() {
        initialize();
        let inputs = NodeInputValue::new(vec![(
            "a".into(),
            PortValue::String(get_path("tests/t2vol.npy").to_str().unwrap().into()),
        )]);
        let result = gpipy_compute("read_np", inputs);
        println!("Done reading, now processing");
        let val = match result {
            Ok(node_output_value) => match node_output_value.get_first() {
                PortValue::Array(_) => node_output_value.get_first(),
                _ => panic!(
                    "Unexpected return value from python. {:?}",
                    node_output_value
                ),
            },
            Err(e) => panic!("Failed in Python: {}", e),
        };
        println!("Done processing");
        let dim = val.dim(); //(val.len(), val[0].len(), val[0][0].len(), val[0][0][0].len());
        assert_eq!(dim, vec![3, 436, 436, 2]);
    }
}
