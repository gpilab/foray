use pyo3::ffi::c_str;

use numpy::{ndarray::Dim, PyArray, PyArray1};
use pyo3::{
    types::{IntoPyDict, PyAnyMethods},
    Bound, Python,
};

pub fn get_array(py: Python<'_>) -> Bound<'_, PyArray<i32, Dim<[usize; 1]>>> {
    let np = py.import("numpy").unwrap();
    let locals = [("np", np)].into_py_dict(py).unwrap();
    let pyarray = py
        .eval(
            c_str!("np.absolute(np.array([-1, -2, -3], dtype='int32'))"),
            Some(&locals),
            None,
        )
        .unwrap()
        .downcast_into::<PyArray1<i32>>()
        .unwrap();

    pyarray
}

#[cfg(test)]
mod test {
    use numpy::PyArrayMethods;
    use pyo3::prepare_freethreaded_python;

    use super::*;
    #[test]
    fn simple_ndarray() {
        prepare_freethreaded_python();

        Python::with_gil(|py| {
            let res = get_array(py).readonly();
            let slice = res.as_slice().unwrap();

            assert_eq!(&[1, 2, 3], slice);
        })
    }
}
