//! API models for Shape input.

#[derive(Debug, Clone)]
pub struct ShapeInput {
    /// 種別: "circle" | "rectangle"
    pub kind: String,

    /// 円の半径（circle のみ）
    pub radius: Option<f64>,

    /// 矩形の幅（rectangle のみ）
    pub width: Option<f64>,

    /// 矩形の高さ（rectangle のみ）
    pub height: Option<f64>,
}
