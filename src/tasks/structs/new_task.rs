use serde::{Deserialize, Serialize};

/// Used to define the creation of a new task.
///
/// `content` must be specified in the creation of a new task, but all other fields are optional.
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct NewTask {
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub due: Option<NewDue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub duration: Option<NewDuration>,
}


impl NewTask {

    /// Create a new `NewTask` with the given content, and all other fields set to `None`.
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
            duration: None
        }
    }
}


/// Represents the new due date of a task. The API allows one of:
/// * A due string and, optionally, a corresponding language (defaulting to english)
/// * A date in the format `YYYY-MM-DD`, relative to the user's timezone
/// * A datetime in RFC3339 format, in UTC.
/// These options are mutually exclusive.
#[derive(Debug, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum NewDue {
    String { due_string: String, due_lang: Option<String> },
    Date { due_date: String },
    Datetime { due_datetime: String },
}

/// Represents the duration of a new task. The API allows no duration to be specified,
/// or alternatively allows both a duration and a duration unit. The duration unit can be
/// either "days" or "minutes".
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct NewDuration {
    pub duration: u32,
    pub duration_unit: DurationUnit,
}


/// Represents the possible duration units. The API allows either "days" or "minutes".
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum DurationUnit {
    Days,
    Minutes,
}

