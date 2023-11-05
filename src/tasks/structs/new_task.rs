use serde::Serialize;

/// Used to define the creation of a new task.
///
/// `content` must be specified in the creation of a new task, but all other fields are optional.
#[derive(Debug, Serialize)]
pub struct NewTask {
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub section_id: Option<String>,
    pub parent_id: Option<String>,
    pub order: Option<i32>,
    pub labels: Option<Vec<String>>,
    pub priority: Option<u8>,
    #[serde(flatten)]
    pub due: Option<NewDue>,
    pub assignee_id: Option<String>,
    pub duration: Option<u32>,
    pub duration_unit: Option<String>,
}


impl NewTask {
    pub fn new(content: &str) -> NewTask {
        NewTask {
            content: content.to_string(),
            description: None,
            project_id: None,
            section_id: None,
            parent_id: None,
            order: None,
            labels: None,
            priority: None,
            due: None,
            assignee_id: None,
            duration: None,
            duration_unit: None,
        }
    }
}


#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NewDue {
    String { due_string: String, due_lang: Option<String> },
    Date { due_date: String },
    Datetime { due_datetime: String },
}

