#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use gpi_framework::port::{PortValue, PrimitiveValue};
    use gpipy::{
        pyo::{gpipy_compute, initialize_gpipy},
        python_node::{PyPortValue, PyPrimitiveValue},
    };

    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
            //let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            //d.push("python_plugin/");
            let _ = initialize_gpipy(&get_path("python_plugin/"));
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

        let _result = match gpipy_compute(
            "add_int",
            vec![
                PortValue::Primitive(PrimitiveValue::Integer(1)),
                PortValue::Primitive(PrimitiveValue::Integer(3)),
            ],
        ) {
            Ok(PortValue::Primitive(PyPrimitiveValue::Integer(res))) => res,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert_eq!(4, _result);
        assert_ne!(1, _result);
    }

    #[test]
    fn add_array() {
        initialize();
        let result = gpipy_compute(
            "add_int_array",
            vec![
                PortValue::Vec1(vec![
                    PrimitiveValue::Real(1.0),
                    PrimitiveValue::Real(2.0),
                    PrimitiveValue::Real(3.0),
                ]),
                PortValue::Vec1(vec![
                    PrimitiveValue::Real(1.0),
                    PrimitiveValue::Real(2.0),
                    PrimitiveValue::Real(3.0),
                ]),
            ],
        );

        let val = match result {
            Ok(PortValue::Vec1(v)) => v,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        assert!(do_vecs_match(
            &val,
            &vec![
                PrimitiveValue::Real(2.0),
                PrimitiveValue::Real(4.0),
                PrimitiveValue::Real(6.0)
            ]
        ))
    }

    #[test]
    fn load_numpy() {
        initialize();
        let npy_path_in = PortValue::Primitive(PrimitiveValue::String(
            get_path("tests/simple.npy").to_str().unwrap().into(),
        ));

        let result = gpipy_compute("read_np", vec![npy_path_in]);
        let val = match result {
            Ok(PortValue::Vec1(val)) => val,
            Ok(_) => panic!("Unexpected return value from python"),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let _expected = vec![
            PrimitiveValue::Real(1.),
            PrimitiveValue::Real(2.),
            PrimitiveValue::Real(3.),
        ];
        assert!(do_vecs_match(&val, &_expected));
    }
    #[test]
    fn load_numpy_2d() {
        initialize();
        let npy_path_in = PortValue::Primitive(PrimitiveValue::String(
            get_path("tests/2dArray.npy").to_str().unwrap().into(),
        ));

        let result = gpipy_compute("read_np", vec![npy_path_in]);
        let val = match result {
            Ok(PortValue::Vec2(val)) => val,
            Ok(e) => panic!("Unexpected return value from python. {:?}", e),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let _expected = vec![
            vec![PrimitiveValue::Real(1.0), PrimitiveValue::Real(2.0)],
            vec![PrimitiveValue::Real(3.0), PrimitiveValue::Real(4.0)],
        ];

        assert!(do_vecs_match(&val, &_expected));
    }
    #[test]
    fn load_numpy_4d() {
        initialize();
        let npy_path_in = PortValue::Primitive(PrimitiveValue::String(
            get_path("tests/t2vol.npy").to_str().unwrap().into(),
        ));
        let result = gpipy_compute("read_np", vec![npy_path_in]);
        let val = match result {
            Ok(PortValue::Vec4(val)) => val,
            Ok(e) => panic!("Unexpected return value from python. {:?}", e),
            Err(e) => panic!("Failed in Python: {}", e),
        };
        let dim = (val.len(), val[0].len(), val[0][0].len(), val[0][0][0].len());
        assert_eq!(dim, (3, 436, 436, 2));
    }

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
