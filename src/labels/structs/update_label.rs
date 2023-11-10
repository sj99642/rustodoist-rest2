//! Defines the structure used to update an existing label

use serde::Serialize;
use crate::color::Color;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::labels::Label;
use crate::TodoistUser;

/// The structure used to update an existing label. The name, order, color and is_favorite fields
/// can all be updated. Any field left None will not be updated.
///
/// Once selecting the appropriate options, call the `update` method with the ID of the label to
/// apply the changes.
#[derive(Serialize)]
#[allow(missing_docs)]
pub struct UpdateLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
}


impl UpdateLabel {
    /// Update the label with the given ID.
    pub fn update(&self, user: &TodoistUser, id: &str) -> Result<Label, TodoistAPIError> {
        // Send the API request
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("https://api.todoist.com/rest/v2/labels/{}", id))
            .header("Authorization", String::from("Bearer ") + &user.token)
            .json(&self)
            .send();
        get_from_reqwest_response(response)
    }
}
