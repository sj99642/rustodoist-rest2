use serde::Deserialize;
use crate::color::Color;
use crate::projects::ViewStyle;

/// Represents a Todoist project.
///
/// Instances of this are created based on responses to API requests, and you don't make them
/// yourself, since not all of the fields are something that can be controlled. To make a new
/// project, see NewProject, or to update a project see UpdateProject.
#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Color,
    pub parent_id: Option<String>,
    pub order: i32,
    pub comment_count: i32,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub view_style: ViewStyle,
    pub url: String,
}
