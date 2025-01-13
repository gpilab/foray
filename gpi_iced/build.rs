use std::{env, ffi::OsString, iter, path::Path};

fn main() {
    // setup env variables for pyo3, so we can specify which python venv should be used
    let venv_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("nodes/.venv");
    let venv_bin = venv_path.join("bin");

    println!(
        "cargo::rustc-env=VIRTUAL_ENV={}",
        venv_path.to_str().unwrap()
    );

    println!(
        "cargo::rustc-env=PATH={}",
        prepend_path(venv_bin.clone()).unwrap().to_str().unwrap()
    );

    println!(
        "cargo::rustc-env=PYO3_PYTHON={}",
        venv_bin.join("python").to_str().unwrap()
    );
}

fn prepend_path<P: AsRef<Path>>(p: P) -> Result<OsString, env::JoinPathsError> {
    let new_path = p.as_ref();
    if let Some(path) = env::var_os("PATH") {
        let old = env::split_paths(&path);
        Ok(env::join_paths(iter::once(new_path.to_owned()).chain(old))?)
    } else {
        Ok(new_path.into())
    }
}
