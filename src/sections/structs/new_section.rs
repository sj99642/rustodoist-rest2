use serde::Serialize;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::sections::Section;
use crate::TodoistUser;

/// The structure to represent a new section to be created.
///
/// A name must be given, along with the project that the section lives in. If
/// no order is provided, the API will put this section at the bottom
/// of the project.
#[derive(Serialize, Debug)]
pub struct NewSection {
    pub name: String,
    pub project_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: i32,
}


impl NewSection {
    /// Upload the new section to the Todoist API.
    pub fn upload(&self, user: &TodoistUser) -> Result<Section, TodoistAPIError> {
        // Send the API request
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.todoist.com/rest/v2/sections")
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(self)
            .send();
        get_from_reqwest_response(response)
    }
}
