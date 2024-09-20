Getting python and rust to co-operate can be tricky. 

There are some issues particularly with macos that make this more difficult than it needs to be.
https://github.com/PyO3/pyo3/issues/1741

On macos, pyo3 won't find the currently used virtual environment path.
As a workaround, we can manually inject the virtual library path into the python interpreter from rust
```
        let sys = PyModule::import_bound(py, "sys")?;
        let syspath: &PyList = sys.getattr("path")?.extract()?;
        syspath.insert(
            0,
            &Path::new(
                "/Users/jechristens3/projects/gpi_v2/gpirs/.venv/lib/python3.12/site-packages/",
            ),
        )?;
```

This won't work for "development" installs though (`pip install -e .` or `maturin develop`)



