use gpirs::pyo;

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use gpirs::{
        node::{NodeInputType, NodeOutputType, PythonNode},
        port::PortType,
    };
    use pyo::initialize_gpipy;

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
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
    fn get_input_output() {
        initialize();
        let node = PythonNode::new(&get_path("../nodes/add_int.py")).unwrap();
        let expected_inputs = NodeInputType(HashMap::from([
            ("a".into(), PortType::Integer {}),
            ("b".into(), PortType::Integer {}),
        ]));
        let expected_outputs =
            NodeOutputType(HashMap::from([("out".into(), PortType::Integer {})]));

        assert_eq!(node.input_types, expected_inputs);
        assert_eq!(node.output_types, expected_outputs);
    }

    #[test]
    fn basic_port_types() {
        initialize();
        let node = PythonNode::new(&get_path("../nodes/portType.py")).unwrap();
        let expected_inputs = NodeInputType(HashMap::from([
            ("my_int".into(), PortType::Integer),
            ("my_real".into(), PortType::Real),
            ("my_string".into(), PortType::String),
            ("my_array".into(), PortType::Array(PortType::Integer.into())),
            (
                "my_2d_array".into(),
                PortType::Array(PortType::Array(PortType::Integer.into()).into()),
            ),
            (
                "my_3d_array".into(),
                PortType::Array(
                    PortType::Array(PortType::Array(PortType::Integer.into()).into()).into(),
                ),
            ),
            (
                "my_struct".into(),
                PortType::Struct(HashMap::from([
                    ("nested_int".into(), PortType::Integer),
                    ("nested_real".into(), PortType::Real),
                    (
                        "nested_nested".into(),
                        PortType::Struct(HashMap::from([(
                            "my_double_nested_string".into(),
                            PortType::String,
                        )])),
                    ),
                ])),
            ),
        ]));
        let expected_outputs =
            NodeOutputType(HashMap::from([("out".into(), PortType::Integer {})]));

        assert_eq!(node.input_types, expected_inputs);
        assert_eq!(node.output_types, expected_outputs);
    }
}
