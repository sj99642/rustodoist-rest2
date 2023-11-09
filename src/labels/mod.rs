//! Functions for working with labels.
//!
//! Labels can be downloaded (either all at once or by ID), created, updated and deleted.
//!


mod structs;

pub use structs::label::Label;
pub use structs::new_label::NewLabel;

use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::TodoistUser;


/// Get a Vec of all the user's personal labels.
pub fn get_all_personal_labels(user: &TodoistUser) -> Result<Vec<Label>, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://api.todoist.com/rest/v2/labels")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}

/// Get an individual Label by its ID.
pub fn get_label_by_id(user: &TodoistUser, id: &str) -> Result<Label, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://api.todoist.com/rest/v2/labels/".to_string() + id)
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}
