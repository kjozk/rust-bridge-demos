use pyo3::prelude::*;
use rust_bridge_core::domain::shape::{Rectangle, Shape};
use rust_bridge_core::domain::metrics::area;

/// Python側で使うRectangle構造体
#[pyclass]
#[derive(Clone)]
pub struct PyRectangle {
    #[pyo3(get, set)]
    pub width: f64,
    #[pyo3(get, set)]
    pub height: f64,
}

#[pymethods]
impl PyRectangle {
    #[new]
    fn new(width: f64, height: f64) -> Self {
        PyRectangle { width, height }
    }
}

/// Pythonから呼ぶ関数
#[pyfunction]
fn calc_rectangle_area(rect: PyRef<PyRectangle>) -> PyResult<f64> {
    let r = Rectangle {
        width: rect.width,
        height: rect.height,
    };
    let shape = Shape::Rectangle(r);
    let result = area(&shape);
    Ok(result.value)
}

/// モジュール定義
#[pymodule]
fn py_bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRectangle>()?;
    m.add_function(wrap_pyfunction!(calc_rectangle_area, m)?)?;
    Ok(())
}
