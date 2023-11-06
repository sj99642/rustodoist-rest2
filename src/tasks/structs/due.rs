use serde::Deserialize;

/// Represents the due date of a task.
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
