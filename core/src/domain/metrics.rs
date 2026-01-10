//! Shape-related calculations.

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub value: f64,
}

use super::shape::{Shape, Circle, Rectangle};

pub fn area(shape: &Shape) -> Area {
    match shape {
        Shape::Circle(circle) => Area {
            value: area_circle(circle),
        },
        Shape::Rectangle(rect) => Area {
            value: area_rectangle(rect),
        },
    }
}


fn area_circle(circle: &Circle) -> f64 {
    std::f64::consts::PI * circle.radius * circle.radius
}

fn area_rectangle(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shape::{Shape, Circle, Rectangle};

    #[test]
    fn circle_area_is_calculated_correctly() {
        let shape = Shape::Circle(Circle { radius: 1.0 });
        let result = area(&shape);
        assert!((result.value - std::f64::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn rectangle_area_is_calculated_correctly() {
        let shape = Shape::Rectangle(Rectangle {
            width: 2.0,
            height: 3.0,
        });

        let result = area(&shape);

        assert_eq!(result.value, 6.0);
    }
}
