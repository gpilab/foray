#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use gpipy::{
        node::{NodeInputValue, NodeOutputType, NodeOutputValue},
        port::PortValue,
        pyo::{gpipy_compute, initialize_gpipy},
    };

    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
            //let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let _ = initialize_gpipy(&get_path("../nodes/"));
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
            Ok(PortValue::Integer(res)) => res,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert_eq!(4, _result);
        assert_ne!(1, _result);
    }

    #[test]
    fn add_array() {
        initialize();
        let inputs = NodeInputValue(HashMap::from([
            (
                "a".into(),
                PortValue::Vec(vec![1.0.into(), 2.0.into(), 3.0.into()]),
            ),
            (
                "b".into(),
                PortValue::Vec(vec![1.0.into(), 2.0.into(), 3.0.into()]),
            ),
        ]));
        println!("starting compute");
        let result = gpipy_compute("add_int_array", inputs);
        println!("ending compute");

        let val = match result {
            Ok(v) => v,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output = PortValue::Vec(vec![2.0.into(), 4.0.into(), 6.0.into()]);
        assert_eq!(val, expected_output);
        println!("ending compare");
    }

    #[test]
    fn load_numpy() {
        initialize();
        let inputs = NodeInputValue(HashMap::from([(
            "a".into(),
            PortValue::String(get_path("tests/simple.npy").to_str().unwrap().into()),
        )]));

        let result = gpipy_compute("read_np", inputs);
        let val = match result {
            Ok(v) => v,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output = PortValue::Vec(vec![1.0.into(), 2.0.into(), 3.0.into()]);
        assert_eq!(val, expected_output);
    }
    #[test]
    fn load_numpy_2d() {
        initialize();
        let inputs = NodeInputValue(HashMap::from([(
            "a".into(),
            PortValue::String(get_path("tests/2dArray.npy").to_str().unwrap().into()),
        )]));

        let result = gpipy_compute("read_np", inputs);
        let val = match result {
            Ok(val) => val,
            Ok(e) => panic!("Unexpected return value from python. {:?}", e),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let expected_output = PortValue::Vec2(vec![
            vec![1.0.into(), 2.0.into()],
            vec![3.0.into(), 4.0.into()],
        ]);

        assert_eq!(val, expected_output);
    }

    //#[test]
    //fn load_numpy_4d() {
    //    initialize();
    //    let npy_path_in = PortValue::String(get_path("tests/t2vol.npy").to_str().unwrap().into());
    //    println!("Reading large npy array");
    //    let result = gpipy_compute("read_np", vec![npy_path_in]);
    //    println!("Done reading, now processing");
    //    let val = match result {
    //        Ok(port_value) => match &port_value {
    //            PortValue::Vec(_) => port_value,
    //            _ => panic!("Unexpected return value from python. {:?}", port_value),
    //        },
    //        Err(e) => panic!("Failed in Python: {}", e),
    //    };
    //    println!("Done processing");
    //    let dim = val.dim(); //(val.len(), val[0].len(), val[0][0].len(), val[0][0][0].len());
    //    assert_eq!(dim, (3, 436, 436, 2));
    //}

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
