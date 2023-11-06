use serde::Serialize;
use crate::color::Color;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::projects::{Project, ViewStyle};
use crate::TodoistUser;

/// A struct representing an update to a project. Distinct from the `NewProject` struct as it is
/// not possible after a project's creation to change its parent using the REST v2 API.
///
/// Construct an instance of this struct as appropriate, then call its `upload()` method, giving
/// the ID of the project to update. If successful, returns a full `Project` struct as returned
/// by the API.
///
/// Note that the `id` is given as an argument to `upload()`, not as a field of the struct itself.
#[derive(Debug, Serialize)]
pub struct UpdateProject {
    /// The name of the project. If unspecified, the name will not be changed.
    /// #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The colour of the project. If unspecified, the colour will not be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    /// Whether the project is a favourite. If unspecified, the favourite status will not be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,

    /// The view style of the project. If unspecified, the view style will not be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_style: Option<ViewStyle>,
}


impl UpdateProject {
    /// Upload a new project with these attributes to the Todoist API.
    pub fn upload(&self, user: &TodoistUser, id: &str) -> Result<Project, TodoistAPIError> {
        // Make the API request
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("https://api.todoist.com/rest/v2/projects/{}", id))
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(&self)
            .send();

        // Now interpret this response properly
        get_from_reqwest_response(response)
    }
}
