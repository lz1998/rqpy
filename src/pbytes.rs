use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[derive(Clone,Default)]
pub struct PBytes(pub Bytes);


impl<'a> FromPyObject<'a> for PBytes {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        Ok(PBytes(Bytes::from(<PyBytes as PyTryFrom>::try_from(obj)?.as_bytes().to_vec())))
    }
}

impl IntoPy<PyObject> for PBytes {
    fn into_py(self, py: Python) -> PyObject {
        PyBytes::new(py, &self.0).to_object(py)
    }
}