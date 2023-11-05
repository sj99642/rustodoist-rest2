use serde::Serialize;
use crate::color::Color;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::projects::{Project, ViewStyle};
use crate::TodoistUser;

#[derive(Debug, Serialize)]
pub struct NewProject {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<ViewStyle>,
}


impl NewProject {
    /// Upload a new project with these attributes to the Todoist API.
    pub fn upload(&self, user: &TodoistUser) -> Result<Project, TodoistAPIError> {
        // Make the API request
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.todoist.com/rest/v2/projects")
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(&self)
            .send();

        // Now interpret this response properly
        get_from_reqwest_response(response)
    }
}
