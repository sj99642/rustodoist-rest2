//! Functions for working with labels.
//!
//! Labels can be downloaded (either all at once or by ID), created, updated and deleted:
//!
//! - To get all personal labels, use `get_all_personal_labels()`
//! - To get a label by its ID, use `get_label_by_id()`
//! - To create a new label, make a `NewLabel` struct and call `upload()` on it
//! - To update a label whose ID you know, make an `UpdateLabel` struct and call `update()` on it
//! - To delete a label whose ID you know, run `delete_label_by_id()`


mod structs;

pub use structs::label::Label;
pub use structs::new_label::NewLabel;

use crate::err::TodoistAPIError;
use crate::general::{get_204_from_reqwest_response, get_from_reqwest_response};
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


pub fn delete_label_by_id(user: &TodoistUser, id: &str) -> Result<(), TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(format!("https://api.todoist.com/rest/v2/labels/{}", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_204_from_reqwest_response(response)
}
