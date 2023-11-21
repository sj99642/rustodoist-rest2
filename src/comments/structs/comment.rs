use serde::Deserialize;

/// Defines the `Comment` struct, which is returned from API calls to represent comments in
/// Todoist.
#[derive(Debug, Deserialize)]
pub struct Comment {
    id: String,
    task_id: Option<String>,
    project_id: Option<String>,
    posted_at: String,
    content: String,
    attachment: Option<Attachment>,
}
