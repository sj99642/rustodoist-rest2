use serde::Deserialize;
use crate::color::Color;

/// Represents a label, as returned by the API
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: Color,
    pub order: i32,
    pub is_favorite: bool,
}
