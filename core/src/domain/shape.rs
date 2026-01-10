//! Shape domain models.

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
