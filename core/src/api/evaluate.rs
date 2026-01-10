use crate::domain::shape::{Rectangle, AreaResult};
use crate::domain::metrics::MetricResult;

pub fn calc_area(rect: Rectangle) -> AreaResult {
    AreaResult {
        area: rect.width * rect.height,
    }
}

pub fn calc_dummy_metric() -> MetricResult {
    MetricResult { result: 100.0 }
}
