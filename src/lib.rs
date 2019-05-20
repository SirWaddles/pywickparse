#[macro_use] extern crate cpython;
extern crate john_wick_parse;

mod pyserde;

use std::cell::RefCell;
use std::fs;
use std::io::Write;
use cpython::{PyResult, PyObject, PyBytes, PyString, PyList, PyErr, Python, ToPyObject, PythonObject};
use john_wick_parse::{assets, read_texture as read_texture_asset};
use john_wick_parse::archives::PakExtractor as NativeExtractor;

py_module_initializer!(pywick, initpywick, PyInit_pywick, |py, m| {
    m.add(py, "__doc__", "Python Bindings for the JohnWickParse library")?;
    m.add(py, "read_asset", py_fn!(py, read_asset(asset_path: String)))?;
    m.add(py, "read_texture", py_fn!(py, read_texture_to_file(asset_path: String, texture_path: String)))?;
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

fn read_texture_to_file(p: Python, asset_path: String, texture_path: String) -> PyResult<bool> {
    let package = match assets::Package::from_file(&asset_path) {
        Ok(data) => data,
        Err(err) => return custom_err(p, format!("Package error: {}", err)),
    };

    let texture_data = match read_texture_asset(package) {
        Ok(data) => data,
        Err(err) => return custom_err(p, format!("Texture error: {}", err)),
    };

    let mut file = fs::File::create(texture_path).unwrap();
    file.write_all(&texture_data).unwrap();
    Ok(true)
}

py_class!(class PakExtractor |py| {
    data extract: RefCell<NativeExtractor>;

    def __new__(_cls, path: String, key: String) -> PyResult<PakExtractor> {
        let extractor = match NativeExtractor::new(&path, &key) {
            Ok(data) => data,
            Err(err) => return custom_err(py, format!("Error loading pak file: {}", err)),
        };
        PakExtractor::create_instance(py, RefCell::new(extractor))
    }

    def get_file_list(&self) -> PyResult<PyList> {
        let filenames: Vec<PyObject> = self.extract(py).borrow().get_entries().into_iter().map(|v| v.get_filename().to_owned().to_py_object(py).into_object()).collect();
        Ok(PyList::new(py, &filenames[..]))
    }

    def get_file(&self, index: u32) -> PyResult<PyBytes> {
        let mut extractor = self.extract(py).borrow_mut();
        let file = extractor.get_entries().get(index as usize).unwrap().clone();
        let data = extractor.get_file(&file);
        Ok(PyBytes::new(py, &data))
    }

    def get_mount_point(&self) -> PyResult<PyString> {
        let extractor = self.extract(py).borrow();
        Ok(PyString::new(py, extractor.get_mount_point()))
    }
});