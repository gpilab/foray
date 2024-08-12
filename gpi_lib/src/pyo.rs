use gpipy::gpipy as pyModule;
use gpipy::GpiNode;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::Path;

pub fn run_plugin() -> Result<(), Box<dyn std::error::Error>> {
    //"export" our API module to the python runtime
    pyo3::append_to_inittab!(pyModule);
    //spawn runtime
    pyo3::prepare_freethreaded_python();

    let path = Path::new("./python_plugin/");

    Python::with_gil(|py| {
        //add the current directory to import path of Python
        #[allow(deprecated)]
        let syspath: &PyList = py.import("sys")?.getattr("path")?.extract()?;
        syspath.insert(0, &path)?;
        //println!("Import path is: {:?}", syspath);

        // load python_plugin/gadget_init_plugin.py
        println!("importing first module...");
        let plugin = PyModule::import_bound(py, "gpi_node_1")?;
        println!("importing second module...");
        let plugin2 = PyModule::import_bound(py, "gadget_init_plugin")?;

        //// INIT
        // and call start function there, which will return a python reference to an object
        let gpi_node_init = plugin.getattr("start")?.call0()?;
        let _gadget_init = plugin2.getattr("start")?.call0()?;

        //now we extract (i.e. mutably borrow) the rust struct from python object
        {
            //this scope will have mutable access to the returned object, which will be dropped on
            //scope exit so Python can access it again.
            let mut gpi_node_init_rs: PyRefMut<'_, GpiNode> = gpi_node_init.extract()?;
            // we can now modify it as if it was a native rust struct
            // which includes access to rust-only fields that are not visible to python
            gpi_node_init_rs.a = 3;
            gpi_node_init_rs.b = 4;

            println!(
                "\nGPI node inputs - a: {:?},b: {:?}",
                gpi_node_init_rs.a, gpi_node_init_rs.b
            );
        }

        //// COMPUTE
        println!("passing to python ...");
        let gadget = plugin.getattr("compute")?.call1((gpi_node_init,))?;
        println!("...returned from python");

        // mutably borrow again in a new scope
        {
            //this scope will have mutable access to the gadget instance, which will be dropped on
            //scope exit so Python can access it again.
            let mut gpi_node_rs: PyRefMut<'_, GpiNode> = gadget.extract()?;
            // we can now modify it as if it was a native rust struct
            //which includes access to rust-only fields that are not visible to python
            println!(
                "Output calculated from python GPI node (a+b): {:?}",
                gpi_node_rs.out
            );
            //gpi_node_rs.rustonly.clear();
            gpi_node_rs.out = gpi_node_rs.out * 2
        }

        //any modifications we make to rust object are reflected on Python object as well
        let res: usize = gadget.getattr("out")?.extract()?;
        println!("After doubling output in rust: {res}");

        //// VIEW
        Ok(())
    })
}
