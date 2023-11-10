//! Functions for working with labels.
//!
//! Labels can be downloaded (either all at once or by ID), created, updated and deleted:
//!
//! - To get all personal labels, use `get_all_personal_labels()`
//! - To get a label by its ID, use `get_label_by_id()`
//! - To create a new label, make a `NewLabel` struct and call `upload()` on it
//! - To update a label whose ID you know, make an `UpdateLabel` struct and call `update()` on it
//! - To delete a label whose ID you know, run `delete_label_by_id()`
//!
//! In addition, Todoist has the concept of shared labels. The information about these is less
//! sophisticated than for personal labels; you can get a list of all of them, rename one or
//! delete one. There is no `struct` for shared labels, as they're just represented as a string.
//! They also have no ID.
//!
//! - To get a list of all shared labels, use `get_all_shared_labels()`
//! - To rename a shared label, use `rename_shared_label()`
//! - To remove a shared label from all tasks to which it is applied, use `delete_shared_label()`


mod structs;

pub use structs::label::Label;
pub use structs::new_label::NewLabel;
pub use structs::update_label::UpdateLabel;

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


/// Delete the label with the given ID.
pub fn delete_label_by_id(user: &TodoistUser, id: &str) -> Result<(), TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(format!("https://api.todoist.com/rest/v2/labels/{}", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_204_from_reqwest_response(response)
}


/// Get a Vec of all the user's shared labels, just as strings.
pub fn get_all_shared_labels(user: &TodoistUser) -> Result<Vec<String>, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/labels/shared")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .query(&[("omit_personal", "false")])
        .send();
    get_from_reqwest_response(response)
}


/// Rename a shared label.
pub fn rename_shared_label(user: &TodoistUser, old_name: &str, new_name: &str) -> Result<(), TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.todoist.com/rest/v2/labels/shared/rename")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .header("Content-Type", "application/json")
        .body(format!("{{\"old_name\": \"{}\", \"new_name\": \"{}\"}}", old_name, new_name))
        .send();
    get_204_from_reqwest_response(response)
}


/// Remove a shared label from all tasks to which it is applied
pub fn remove_shared_label(user: &TodoistUser, name: &str) -> Result<(), TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.todoist.com/rest/v2/labels/shared/remove")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .header("Content-Type", "application/json")
        .body(format!("{{\"name\": \"{}\"}}", name))
        .send();
    get_204_from_reqwest_response(response)
}
