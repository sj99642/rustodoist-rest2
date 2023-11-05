use serde::Deserialize;
use crate::tasks::structs::due::Due;
use crate::tasks::structs::duration::Duration;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub section_id: Option<String>,
    pub content: String,
    pub description: String,
    pub is_completed: bool,
    pub labels: Vec<String>,
    pub parent_id: Option<String>,
    pub order: i32,
    pub priority: u8,
    pub due: Option<Due>,
    pub url: String,
    pub comment_count: u32,
    pub created_at: String,
    pub creator_id: String,
    pub assignee_id: Option<String>,
    pub assigner_id: Option<String>,
    pub duration: Option<Duration>,
}
