use std::error::Error;

use app::pyo;

//#[derive(FromPyObject)]
//struct RustyStruct {
//    my_string: String,
//}

fn main() -> Result<(), Box<dyn Error>> {
    pyo::run_plugin()
    //let _ = Python::with_gil(|py| {
    //    let np = py.import_bound("numpy")?;
    //    let globals: PyDict = [
    //        (
    //            "myVar",
    //            RustyStruct {
    //                my_string: "abc".into(),
    //            },
    //        ),
    //        ("np", np),
    //        ("myVar", "a"),
    //    ]
    //    .into_py_dict_bound(py);
    //    let locals = PyDict::new_bound(py).into_py_dict_bound(py);
    //
    //    py.eval_bound(
    //        &format!("print('my global var:',myVar)"),
    //        Some(&globals),
    //        Some(&locals),
    //    )?;
    //    Ok::<_, PyErr>(())
    //    //Ok(locals.get_item("test").unwrap().downcast_into::<i32>()?)
    //});
    //
    //Python::with_gil(|py| {
    //    let np = py.import_bound("numpy")?;
    //    let globals = [("np", np)].into_py_dict_bound(py);
    //    let locals = PyDict::new_bound(py).into_py_dict_bound(py);
    //
    //    py.eval_bound(
    //        &format!("print('hello',myVar)"),
    //        Some(&globals),
    //        Some(&locals),
    //    )?;
    //
    //    //assert_eq!(, 7);
    //    Ok::<_, PyErr>(())
    //})
}
