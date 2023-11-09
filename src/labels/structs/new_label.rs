//! Defines the structure used to create a new label.


use serde::Serialize;
use crate::color::Color;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::labels::Label;
use crate::TodoistUser;

#[derive(Serialize)]
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
