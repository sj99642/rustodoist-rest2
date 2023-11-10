use serde::Deserialize;

/// Defines a Todoist Section. There's no need to create one of these youself, but
/// instances of this are returned from most section-related API calls.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Section {
    pub id: String,
    pub project_id: String,
    pub order: i32,
    pub name: String,
}
