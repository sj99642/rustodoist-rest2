use serde::Serialize;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::tasks::{NewDue, NewDuration, Task};
use crate::TodoistUser;

/// Used to make an update to a task. Any field which is `None` will not be updated.
///
/// Call the `update()` method when you are ready to upload the changes to the API.
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct UpdateTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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


impl UpdateTask {
    /// Upload the changes defined in the struct to the API, specifying the ID of the task
    /// you want to update.
    pub fn upload(
        &self,
        user: &TodoistUser,
        task_id: &str,
    ) -> Result<Task, TodoistAPIError> {
        // Make the API request and interpret the response
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("https://api.todoist.com/rest/v2/tasks/{}", task_id))
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(&self)
            .send();
        get_from_reqwest_response(response)
    }
}
