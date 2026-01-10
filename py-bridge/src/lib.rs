use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rust_bridge_core::domain::shape::{Rectangle, Shape};
use rust_bridge_core::domain::metrics::area;

#[pyclass]
#[derive(Clone)]
pub struct PyRectangle {
    #[pyo3(get, set)]
    pub width: f64,
    #[pyo3(get, set)]
    pub height: f64,
}

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

#[pymodule]
fn py_bridge(py: Python, m: &PyModule) -> PyResult<()> {
    // PyO3 0.20 以降は add_class と add_function を m.add_class::<T>() ではなく add_class 形式で呼ぶ
    m.add_class::<PyRectangle>()?;
    m.add_function(wrap_pyfunction!(calc_rectangle_area, m)?)?;
    Ok(())
}
