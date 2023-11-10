//! Sections live within projects. Sections are simpler than projects, needing only an order
//! number, name and project ID to fully describe them.
//!
//! As with other object types, you can download a list of all sections or get an individual one
//! by its ID. You can also create a new section, or update or delete a section whose ID you know.
//!
//! - To get all the user's sections, use `get_all_sections()`
//! - To get all sections in a project, use `get_all_sections_in_project()`
//! - To get a section by its ID, use `get_section_by_id()`
//! - To create a new section, make a `NewSection` struct and call `upload()` on it
//! - To update a section whose ID you know, use `update_section()`*
//! - To delete a section whose ID you know, run `delete_section_by_id()`
//!
//! * Note that updating a section is different to updating other objects. Once a section is made,
//!   the only thing that can be updated is its name. Thus there's no reason for a dedicated
//!   struct, and we just have a dedicated function instead.


mod structs;

pub use structs::section::Section;
use crate::err::TodoistAPIError;
use crate::general::{get_204_from_reqwest_response, get_from_reqwest_response};
use crate::TodoistUser;

/// Return a `Vec` of all sections in every project
pub fn get_all_sections(user: &TodoistUser) -> Result<Vec<Section>, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/sections")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}

/// Return a `Vec` of all sections in the given project
pub fn get_all_sections_in_project(
    user: &TodoistUser,
    project_id: &str,
) -> Result<Vec<Section>, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!(
            "https://api.todoist.com/rest/v2/sections?project_id={}",
            project_id
        ))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}


/// Get an individual Section by its ID.
pub fn get_section_by_id(user: &TodoistUser, id: &str) -> Result<Section, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://api.todoist.com/rest/v2/sections/{}", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}


/// Delete the section with the given ID.
pub fn delete_section_by_id(user: &TodoistUser, id: &str) -> Result<(), TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(format!("https://api.todoist.com/rest/v2/sections/{}", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_204_from_reqwest_response(response)
}


/// Change the section's name
pub fn update_section(user: &TodoistUser, id: &str, new_name: &str) -> Result<Section, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let mut map = std::collections::HashMap::new();
    map.insert("name", new_name);
    let response = client
        .post(format!("https://api.todoist.com/rest/v2/sections/{}", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .json(&map)
        .send();
    get_from_reqwest_response(response)
}
