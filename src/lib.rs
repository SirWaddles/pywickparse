#[macro_use] extern crate cpython;
extern crate john_wick_parse;

mod pyserde;

use cpython::{PyResult, PyObject, PyList, PyErr, Python, ToPyObject, PythonObject};
use john_wick_parse::{assets, read_texture as read_texture_asset};
use john_wick_parse::archives::PakExtractor as NativeExtractor;

py_module_initializer!(pywick, initpywick, PyInit_pywick, |py, m| {
    m.add(py, "__doc__", "Python Bindings for the JohnWickParse library")?;
    m.add(py, "read_asset", py_fn!(py, read_asset(asset_path: String)))?;
    m.add_class::<PakExtractor>(py)?;
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

py_class!(class PakExtractor |py| {
    data extract: NativeExtractor;

    def __new__(_cls, path: String, key: String) -> PyResult<PakExtractor> {
        let extractor = match NativeExtractor::new(&path, &key) {
            Ok(data) => data,
            Err(err) => return custom_err(py, format!("Error loading pak file: {}", err)),
        };
        PakExtractor::create_instance(py, extractor)
    }

    def get_file_list(&self) -> PyResult<PyList> {
        let filenames: Vec<PyObject> = self.extract(py).get_entries().into_iter().map(|v| v.get_filename().to_owned().to_py_object(py).into_object()).collect();
        Ok(PyList::new(py, &filenames[..]))
    }
});