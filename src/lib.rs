#[macro_use] extern crate cpython;
extern crate john_wick_parse;

mod pyserde;

use cpython::{PyResult, PyObject, PyErr, Python};
use john_wick_parse::{assets, read_texture as read_texture_asset};
use john_wick_parse::archives::PakExtractor;

py_module_initializer!(libpywick, initlibpywick, PyInit_libpywick, |py, m| {
    m.add(py, "__doc__", "Python Bindings for the JohnWickParse library")?;
    m.add(py, "read_asset", py_fn!(py, read_asset(asset_path: String)))?;
    Ok(())
});

fn custom_err<T>(py: Python, s: String) -> PyResult<T> {
    Err(PyErr::new::<cpython::exc::TypeError, _>(py, s))
}

fn read_asset(p: Python, asset_path: String) -> PyResult<PyObject> {
    let package = match assets::Package::from_file(&asset_path) {
        Ok(data) => data,
        Err(err) => return custom_err(p, format!("Could not load path: {}", err)),
    };

    let asset = match pyserde::to_value(p, package) {
        Ok(data) => data,
        Err(err) => return custom_err(p, format!("Serialization error: {}", err)),
    };

    Ok(asset)
}
