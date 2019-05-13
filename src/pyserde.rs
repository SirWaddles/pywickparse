use serde::ser;
use serde::ser::Serialize;
use std::fmt;
use cpython::{PyObject, PyBytes, PyList, PyDict, Python, ToPyObject, PythonObject};

#[derive(Debug)]
pub enum PyserError {
    SerdeError,
    NotImplemented,
    Other,
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
    let value = v.serialize(serializer)?;
    Ok(value)
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
        where T: ?Sized + Serialize
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

    fn serialize_element<T>(&mut self, value: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> PyserResult<PyObject> {
        ser::SerializeSeq::end(self)
    }
}

impl<'p> ser::SerializeTupleStruct for ArraySerializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_field<T>(&mut self, value: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> PyserResult<PyObject> {
        ser::SerializeSeq::end(self)
    }
}

pub struct TupleVariantSerializer {

}

impl ser::SerializeTupleVariant for TupleVariantSerializer {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_field<T>(&mut self, _value: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        Err(PyserError::NotImplemented)
    }

    fn end(self) -> PyserResult<PyObject> {
        Err(PyserError::NotImplemented)
    }
}

pub struct MapSerializer<'p> {
    p: Python<'p>,
    keys: Vec<PyObject>,
    values: Vec<PyObject>,
}

impl<'p> MapSerializer<'p> {
    fn new(p: Python<'p>) -> Self {
        Self {
            p,
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<'p> ser::SerializeMap for MapSerializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_key<T>(&mut self, key: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        let key_obj = to_value(self.p, key)?;
        self.keys.push(key_obj);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        let value_obj = to_value(self.p, value)?;
        self.keys.push(value_obj);
        Ok(())
    }

    fn end(self) -> PyserResult<PyObject> {
        let dict = PyDict::new(self.p);
        for i in 0..self.keys.len() {
            dict.set_item(self.p, &self.keys[i], &self.values[i]).map_err(|_err| PyserError::Other)?;
        }

        Ok(dict.into_object())
    }
}

pub struct StructSerializer<'p> {
    p: Python<'p>,
    dict: PyDict,
}

impl<'p> StructSerializer<'p> {
    fn new(p: Python<'p>) -> Self {
        Self {
            p,
            dict: PyDict::new(p),
        }
    }
}

impl<'p> ser::SerializeStruct for StructSerializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        let value = to_value(self.p, value)?;
        self.dict.set_item(self.p, key, value).map_err(|_err| PyserError::Other)?;
        Ok(())
    }

    fn end(self) -> PyserResult<PyObject> {
        Ok(self.dict.into_object())
    }
}

pub struct StructVariantSerializer {

}

impl ser::SerializeStructVariant for StructVariantSerializer {
    type Ok = PyObject;
    type Error = PyserError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> PyserResult<()>
        where T: ?Sized + Serialize
    {
        Err(PyserError::NotImplemented)
    }

    fn end(self) -> PyserResult<PyObject> {
        Err(PyserError::NotImplemented)
    }
}

impl<'p> ser::Serializer for Serializer<'p> {
    type Ok = PyObject;
    type Error = PyserError;

    type SerializeSeq = ArraySerializer<'p>;
    type SerializeTuple = ArraySerializer<'p>;
    type SerializeTupleStruct = ArraySerializer<'p>;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer<'p>;
    type SerializeStruct = StructSerializer<'p>;
    type SerializeStructVariant = StructVariantSerializer;

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
        let val: Option<PyObject> = None;
        Ok(val.to_py_object(self.p).into_object())
    }

    fn serialize_some<T>(self, value: &T) -> PyserResult<PyObject>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> PyserResult<PyObject> {
        let val: Option<PyObject> = None;
        Ok(val.to_py_object(self.p).into_object())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> PyserResult<PyObject> {
        let val: Option<PyObject> = None;
        Ok(val.to_py_object(self.p).into_object())
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
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let val: Option<PyObject> = None;
        Ok(val.to_py_object(self.p).into_object())

    }

    fn serialize_seq(self, _len: Option<usize>) -> PyserResult<Self::SerializeSeq> {
        Ok(ArraySerializer::new(self.p))
    }

    fn serialize_tuple(self, _len: usize) -> PyserResult<Self::SerializeTuple> {
        Ok(ArraySerializer::new(self.p))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> PyserResult<Self::SerializeTupleStruct> {
        Ok(ArraySerializer::new(self.p))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> PyserResult<Self::SerializeTupleVariant> {
        Err(PyserError::NotImplemented)
    }

    fn serialize_map(self, _len: Option<usize>) -> PyserResult<Self::SerializeMap> {
        Ok(MapSerializer::new(self.p))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> PyserResult<Self::SerializeStruct> {
        Ok(StructSerializer::new(self.p))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> PyserResult<Self::SerializeStructVariant> {
        Err(PyserError::NotImplemented)
    }
}