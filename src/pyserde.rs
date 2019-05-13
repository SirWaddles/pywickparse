use serde::ser;
use serde::ser::Serialize;
use std::fmt;
use cpython::{PyObject, PyBytes, PyList, Python, ToPyObject, PythonObject};

#[derive(Debug)]
pub enum PyserError {
    SerdeError,
    NotImplemented,
}

impl fmt::Display for PyserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for PyserError { }

pub type PyserResult<T> = Result<T, PyserError>;

impl ser::Error for PyserError {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        PyserError::SerdeError
    }
}

pub fn to_value<'p, V>(p: Python<'p>, v: V) -> PyserResult<PyObject>
    where V: Serialize + Sized
{
    let serializer = Serializer {p};
    Err(PyserError::NotImplemented)
}

pub struct Serializer<'p> {
    p: Python<'p>,
}

pub struct ArraySerializer<'p> {
    p: Python<'p>,
    list: PyList,
}

impl<'p> ArraySerializer<'p> {
    fn new(p: Python<'p>) -> Self {
        Self {
            p, 
            list: PyList::new(p, &[]),
        }
    }
}

impl<'p> ser::SerializeSeq for ArraySerializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_element<T>(&mut self, value: &T) -> PyserResult<()>
        where T: Serialize + Sized
    {
        let value = to_value(self.p, value)?;
        self.list.insert_item(self.p, self.list.len(self.p), value);
        Ok(())
    }

    fn end(self) -> PyserResult<PyObject> {
        Ok(self.list.into_object())
    }
}

impl<'p> ser::SerializeTuple for ArraySerializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_element<T>(&mut self, value: &T) -> PyserResult<()> {
        ser::SerializeSeq::serialize_element(self, value)
    }
}

impl<'p> ser::Serializer for Serializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    type SerializeSeq = ArraySerializer<'p>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_i8(self, v: i8) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_i16(self, v: i16) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_i32(self, v: i32) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_i64(self, v: i64) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_u8(self, v: u8) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_u16(self, v: u16) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_u32(self, v: u32) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_u64(self, v: u64) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_f32(self, v: f32) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_f64(self, v: f64) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_char(self, v: char) -> PyserResult<PyObject> {
        let mut b = [0; 4];
        let result = v.encode_utf8(&mut b);
        Ok(result.to_py_object(self.p).into_object())
    }

    fn serialize_str(self, v: &str) -> PyserResult<PyObject> {
        Ok(v.to_py_object(self.p).into_object())
    }

    fn serialize_bytes(self, v: &[u8]) -> PyserResult<PyObject> {
        let bytes = PyBytes::new(self.p, v);
        Ok(bytes.into_object())
    }

    fn serialize_none(self) -> PyserResult<PyObject> {
        Ok(None.to_py_object(self.p).into_object())
    }

    fn serialize_some<T>(self, value: &T) -> PyserResult<PyObject>
        where T: Serialize + Sized
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> PyserResult<PyObject> {
        Ok(None.to_py_object(self.p).into_object())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> PyserResult<PyObject> {
        Ok(None.to_py_object(self.p).into_object())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> PyserResult<PyObject> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Ok(None.to_py_object(self.p).into_object())

    }

    fn serialize_seq(self, _len: Option<usize>) -> PyserResult<Self::SerializeSeq> {
        Ok(ArraySerializer::new(self.p))
    }

    fn serialize_tuple(self, _len: usize) -> PyserResult<Self::SerializeTuple> {
        Ok(ArraySerializer::new(self.p))
    }
}