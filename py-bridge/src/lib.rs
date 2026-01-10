use pyo3::prelude::*;
use rust_bridge_core::domain::shape::Rectangle;
use rust_bridge_core::domain::metrics::area_rectangle;

// Python モジュールとして公開
#[pymodule]
fn py_bridge(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "calc_rectangle_area")]
    fn calc_rectangle_area_py(_py: Python, rect: &PyAny) -> PyResult<f64> {
        // Python dict から Rectangle に変換
        let width: f64 = rect.get_item("width")?.extract()?;
        let height: f64 = rect.get_item("height")?.extract()?;
        let r = Rectangle { width, height };

        let area = area_rectangle(&r);
        Ok(area)
    }

    Ok(())
}
