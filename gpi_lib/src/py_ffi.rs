use numpy::{ndarray::Dim, PyArray, PyArray2};
use pyo3::{
    types::{IntoPyDict, PyAnyMethods, PyDict},
    Bound, PyErr, Python,
};

pub fn py_ffi<'a>(
    py: Python<'a>,
    arr: &'a Bound<'a, PyArray<i32, Dim<[usize; 2]>>>,
) -> Result<pyo3::Bound<'a, PyArray<i32, Dim<[usize; 2]>>>, PyErr> {
    let np = py.import_bound("numpy")?;
    let globals = [("np", np)].into_py_dict_bound(py);
    let locals = [("arr", arr.clone())].into_py_dict_bound(py);

    py.eval_bound(&format!("arr.fill(7)"), Some(&globals), Some(&locals))?;

    Ok(locals
        .get_item("arr")
        .unwrap()
        .downcast_into::<PyArray2<i32>>()?)
}

pub fn py1<'a>() {
    //-> Result<Bound<'a, i32>, PyErr> {
    let _ = Python::with_gil(|py| {
        let np = py.import_bound("numpy")?;
        let globals = [("np", np)].into_py_dict_bound(py);
        let locals = PyDict::new_bound(py).into_py_dict_bound(py);

        py.eval_bound(&format!("myVar=13"), Some(&globals), Some(&locals))?;
        Ok::<_, PyErr>(())
        //Ok(locals.get_item("test").unwrap().downcast_into::<i32>()?)
    });
}

pub fn create_py_array() {}

#[cfg(test)]
mod test {
    use super::*;
    use numpy::PyArrayMethods;
    use pyo3::prepare_freethreaded_python;

    #[test]
    fn ffi() {
        prepare_freethreaded_python();
        Python::with_gil(|py| {
            let arr = unsafe {
                let py_arr = PyArray2::<i32>::new_bound(py, [100, 100], false);

                for i in 0..100 {
                    for j in 0..100 {
                        numpy::PyArrayMethods::uget_raw(&py_arr, [i, j]).write((i * j) as i32);
                    }
                }
                py_arr
            };
            let bind = arr.readonly();
            let result = bind.as_array();
            assert_eq!(result.get([1, 1]), Some(&1));
            assert_eq!(result.len(), 10000);
            let bind = py_ffi(py, &arr).unwrap().readonly();
            let result = bind.as_array();
            assert_eq!(result.len(), 10000);
            assert_eq!(result.get([1, 1]), Some(&7));
        });
    }
    #[test]
    fn py1<'a>() {
        prepare_freethreaded_python();
        let _ = Python::with_gil(|py| {
            let np = py.import_bound("numpy")?;
            let globals = [("np", np)].into_py_dict_bound(py);
            let locals = PyDict::new_bound(py).into_py_dict_bound(py);

            py.eval_bound(
                &format!("print('hello',myVar)"),
                Some(&globals),
                Some(&locals),
            )?;

            //assert_eq!(, 7);
            Ok::<_, PyErr>(())
        });
        assert!(true)
    }
}
