use gpipy::pyo;

use gpipy::python_node::gpipy as pyModule;

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use gpipy::{
        node::{NodeInputType, NodeOutputType, PythonNode},
        port::PortType,
    };
    use pyo::initialize_gpipy;

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
            pyo3::append_to_inittab!(pyModule);
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
    fn get_input_output() {
        initialize();
        let node = PythonNode::new(&get_path("../nodes/add_int.py")).unwrap();
        let expected_inputs = NodeInputType(HashMap::from([
            ("a".into(), PortType::Integer),
            ("b".into(), PortType::Integer),
        ]));
        let expected_outputs = NodeOutputType(HashMap::from([("out".into(), PortType::Integer)]));

        assert_eq!(node.input_types, expected_inputs);
        assert_eq!(node.output_types, expected_outputs);
        //assert!(do_vecs_match(&expected_inputs, &node.input_types));
        //assert!(do_vecs_match(&expected_outputs, &node.output_types));
    }
    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
