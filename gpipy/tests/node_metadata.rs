use gpipy::pyo;

use gpipy::python_node::gpipy as pyModule;

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use gpipy::node::PythonNode;
    use pyo::initialize_gpipy;

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();
    fn initialize() {
        INIT.call_once(|| {
            pyo3::append_to_inittab!(pyModule);
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
    fn get_input_output() {
        initialize();
        //let node = PythonNode::new(&get_path("python_plugin/add_int.py")).unwrap();
        let node = PythonNode::new(&get_path("python_plugin/add_int.py")).unwrap();
        let expected_inputs = vec!["Integer".to_string(), "Integer".to_string()];
        let expected_outputs = vec!["Integer".to_string()];

        assert!(do_vecs_match(&expected_inputs, &node.input_types));
        assert!(do_vecs_match(&expected_outputs, &node.output_types));
    }
    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
