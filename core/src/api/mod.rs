//! API models for external bridges.
//!
//! シリアライズ可能で安定した
//! 境界インターフェース定義。

// MessagePack / PyO3 向けの IF が入る
pub mod shape;
pub mod result;

use crate::domain::{
    metrics::Area,
    shape::{Circle, Rectangle, Shape},
};

use shape::ShapeInput;
use result::AreaResult;

impl TryFrom<ShapeInput> for Shape {
    type Error = String;

    fn try_from(input: ShapeInput) -> Result<Self, Self::Error> {
        match input.kind.as_str() {
            "circle" => {
                let radius = input.radius.ok_or("radius is required for circle")?;
                Ok(Shape::Circle(Circle { radius }))
            }
            "rectangle" => {
                let width = input.width.ok_or("width is required for rectangle")?;
                let height = input.height.ok_or("height is required for rectangle")?;
                Ok(Shape::Rectangle(Rectangle { width, height }))
            }
            _ => Err("unknown shape kind".to_string()),
        }
    }
}


impl From<Area> for AreaResult {
    fn from(area: Area) -> Self {
        AreaResult {
            result: area.value,
        }
    }
}

use crate::domain::metrics;

pub fn calculate_area(input: ShapeInput) -> Result<AreaResult, String> {
    let shape = Shape::try_from(input)?;
    let area = metrics::area(&shape);
    Ok(area.into())
}
