use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rust_bridge_core::domain::shape::Rectangle;
use rust_bridge_core::domain::metrics::area;

#[pyfunction]
fn calc_rectangle_area_py(_py: Python, rect: &PyAny) -> PyResult<f64> {
    let width: f64 = rect.get_item("width")?.extract()?;
    let height: f64 = rect.get_item("height")?.extract()?;
    let r = Rectangle { width, height };
    Ok(area(&r))
}

#[pymodule]
fn py_bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_rectangle_area_py, m)?)?;
    Ok(())
}