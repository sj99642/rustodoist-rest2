//! Defines the structure used to create a new label.


use serde::Serialize;
use crate::color::Color;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::labels::Label;
use crate::TodoistUser;


/// The structure used to create a new label. Create an instance of this struct with the chosen
/// options, then call the `upload` method to upload it to the Todoist API. The `name` field is
/// mandatory, but the others are optional.
///
/// - If `order` is not specified, the label will be added to the end of the list.
/// - If `color` is not specified, the label will be given the default API color.
/// - If `is_favorite` is not specified, the label will not be marked as a favorite.
#[derive(Serialize)]
#[allow(missing_docs)]
pub struct NewLabel {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
}


impl NewLabel {
    /// Upload the label to the Todoist API. Returns a full `Label` struct.
    pub fn upload(&self, user: &TodoistUser) -> Result<Label, TodoistAPIError> {
        // Send the API request
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.todoist.com/rest/v2/labels")
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(&self)
            .send();
        get_from_reqwest_response(response)
    }
}
