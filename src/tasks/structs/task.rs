use serde::Deserialize;

/// Defines a task as returned by the API. There is no meaning in constructing a task like this
/// directly (make a `NewTask` or `UpdateTask` struct to make those API calls). Instances of this
/// are returned by most task-related API calls to show the current state of the task after
/// the call is finished.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
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


/// Represents the duration of a task. The possible `unit` values are "minute" or "day".
/// The "amount" field is the number of minutes or days, depending on the unit.
///
/// If creating a new task, use NewDuration instead.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Duration {
    pub amount: u32,
    pub unit: String,
}


/// Represents the due date of a task deserialized from the API.
///
/// If creating a new task, use `NewDue` instead.
#[derive(Debug, Deserialize)]
pub struct Due {
    /// The string form of the due date, as entered by a user. Its format is arbitrary and user-
    /// selected.
    pub string: String,

    /// A date in the format `YYYY-MM-DD`, corrected to the user's timezone.
    pub date: String,

    /// Whether the task has a recurring due date. If `is_recurring` is `true`, then when this task
    /// is completed, the next instance of the task will be created with a due-date calculated based
    /// on the natural-language content of the `string` field.
    pub is_recurring: bool,

    /// Only returned if an exact due time is set. Conforms to
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format.
    pub datetime: Option<String>,

    /// Only returned if an exact due time is set. It will either be in an IANA Timezone Database
    /// format (e.g. "Europe/London"), or a UTC offset format ("UTC±HH:MM").
    pub timezone: Option<String>,
}

