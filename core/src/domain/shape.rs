//! Shape domain models.
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AreaResult {
    pub area: f64,
}
