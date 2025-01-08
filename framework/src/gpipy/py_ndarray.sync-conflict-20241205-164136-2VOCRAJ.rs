use numpy::{
    ndarray::{ArrayBase, ArrayView, ArrayViewMut, ArrayViewMut1, CowArray, Dim},
    PyArray, PyArray1, PyArrayMethods, PyReadonlyArray,
};
use pyo3::{
    types::{IntoPyDict, PyAnyMethods},
    Bound, PyResult, Python,
};

struct MyArray<'a>(CowArray<'a, i32, Dim<[usize; 1]>>);

fn get_array<'a>(py: Python<'a>) -> Bound<'a, PyArray<i32, Dim<[usize; 1]>>> {
    //-> CowArray<'a, i32, Dim<[usize; 1]>> {
    let np = py.import_bound("numpy").unwrap();
    let locals = [("np", np)].into_py_dict_bound(py);
    let pyarray = py
        .eval_bound(
            "np.absolute(np.array([-1, -2, -3], dtype='int32'))",
            Some(&locals),
            None,
        )
        .unwrap()
        .downcast_into::<PyArray1<i32>>()
        .unwrap();

    pyarray

    //let mut rw = pyarray.readwrite();
    //Ok(rw.as_array_mut())
    //let slice = readonly.as_slice()?;
    //assert_eq!(slice, &[1, 2, 3]);
    //Ok(readonly.to_vec().unwrap())
}

#[cfg(test)]
mod test {
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
