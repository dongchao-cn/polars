mod exitable;
#[cfg(feature = "pymethods")]
mod general;
#[cfg(feature = "pymethods")]
mod serde;
mod sink;
pub mod visit;
pub mod visitor;

#[cfg(not(target_arch = "wasm32"))]
pub use exitable::PyInProcessQuery;
use polars::prelude::LazyFrame;
use pyo3::pyclass;
pub use sink::{PyPartitioning, SinkTarget};

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyLazyFrame {
    pub ldf: LazyFrame,
}

impl From<LazyFrame> for PyLazyFrame {
    fn from(ldf: LazyFrame) -> Self {
        PyLazyFrame { ldf }
    }
}
