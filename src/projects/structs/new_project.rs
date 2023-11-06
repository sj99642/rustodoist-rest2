use serde::Serialize;
use crate::color::Color;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::projects::{Project, ViewStyle};
use crate::TodoistUser;

/// Represents the creation of a new Todoist project.
///
/// Create an instance of this struct with the appropriate field values, then run its `upload()`
/// method to make the API call. If it is successful, a full `Project` struct will be returned.
#[derive(Debug, Serialize)]
pub struct NewProject {
    /// The only required field is the name of the project
    pub name: String,

    /// The ID of the parent project. If unspecified, this will be a top-level project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    /// The colour of the project. The default, if left `None`, is Charcoal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    /// Whether the project is a favourite. The default, if left `None`, is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,

    /// The view style of the project. The default, if left `None`, is `List`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<ViewStyle>,
}


impl NewProject {
    /// Upload a new project with these attributes to the Todoist API.
    ///
    /// If successful, returns a full Project struct as returned by the API.
    pub fn upload(&self, user: &TodoistUser) -> Result<Project, TodoistAPIError> {
        // Make the API request
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.todoist.com/rest/v2/projects")
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(self)
            .send();

        // Now interpret this response properly
        get_from_reqwest_response(response)
    }
}
