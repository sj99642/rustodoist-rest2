//! Functions for getting information about your projects, updating, or deleting them.

mod view_style;

use crate::TodoistUser;
use crate::err::TodoistAPIError;
use crate::color::Color;
use crate::general::{get_from_reqwest_response, get_204_from_reqwest_response};
pub use crate::projects::view_style::ViewStyle;

use reqwest;
use serde::Deserialize;
use serde_json::{Value, Map};
use uuid::Uuid;

/// Represents a Todoist project as returned from the API.
///
/// <https://developer.todoist.com/rest/v2/#projects>
#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub parent_id: Option<String>,
    pub order: i32,
    pub comment_count: i32,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub is_inbox_project: bool,
    pub view_style: String,
    pub url: String,
}


/// Return a Vec of all the user's projects.
///
/// <https://developer.todoist.com/rest/v2/#get-all-projects>
pub fn get_projects(user: &TodoistUser) -> Result<Vec<Project>, TodoistAPIError> {
    // Make the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/projects")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();

    // Now interpret this response properly
    get_from_reqwest_response(response)
}


/// Add a new project
///
/// A project name is mandatory, while the other fields are optional, with a default provided by
/// Todoist. For each of `parent_id`, `color`, `is_favorite` and `view_style`, give `None` to use
/// the default, or `Some` to specify an explicit value.
///
/// <https://developer.todoist.com/rest/v2/#create-a-new-project>
pub fn add_new_project(
    user: &TodoistUser,
    name: &str,
    parent_id: Option<&str>,
    color: Option<Color>,
    is_favorite: Option<bool>,
    view_style: Option<ViewStyle>,
) -> Result<Project, TodoistAPIError> {
    // Make the mapping with all the arguments which were chosen
    let mut map = Map::new();
    map.insert("name".to_string(), Value::String(name.to_string()));
    if let Some(parent_id) = parent_id {
        map.insert("parent_id".to_string(), Value::String(parent_id.to_string()));
    }
    if let Some(color) = color {
        map.insert("color".to_string(), Value::String(color.to_str().to_string()));
    }
    if let Some(is_favorite) = is_favorite {
        map.insert("is_favorite".to_string(), Value::Bool(is_favorite));
    }
    if let Some(view_style) = view_style {
        map.insert("view_style".to_string(), Value::String(view_style.to_str().to_string()));
    }

    // Turn this map into JSON, assuming serialisation was successful
    let json = serde_json::to_string(&map);
    let json = match json {
        Ok(serialised) => serialised,
        Err(err) => return Err(TodoistAPIError::SerdeSerialisationError(err)),
    };

    // Add the new project via the API
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.todoist.com/rest/v2/projects")
        .body(json)
        .header("Content-Type", "application/json")
        .header("X-Request-Id", Uuid::new_v4().hyphenated().to_string())
        .header("Authorization", "Bearer ".to_string() + &user.token)
        .send();

    // Unpack the response, which will hopefully be a Project
    get_from_reqwest_response(response)
}


/// Return a single project from its ID.
///
/// https://developer.todoist.com/rest/v2/#get-a-project
pub fn get_project_by_id(user: &TodoistUser, id: &str) -> Result<Project, TodoistAPIError>{
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://api.todoist.com/rest/v2/projects/{}", id))
        .header("Authorization", "Bearer ".to_string() + &user.token)
        .send();
    get_from_reqwest_response(response)
}


/// Update a project with a given ID.
///
/// All individual fields (`name`, `color`, `is_favorite`, `view_style`) are optional, but at least
/// one should be provided for this call to make sense. For each argument, give `None` to leave that
/// field unchanged for ths project, or `Some` to specify a new value for that field.
///
/// Returns the project as it stands after the update.
///
/// <https://developer.todoist.com/rest/v2/#update-a-project>
pub fn update_project_by_id(
    user: &TodoistUser,
    id: &str,
    name: Option<&str>,
    color: Option<Color>,
    is_favorite: Option<bool>,
    view_style: Option<ViewStyle>
) -> Result<Project, TodoistAPIError> {
    // Start by composing the body of the request, which contains the things to change
    let mut map = Map::new();
    if let Some(name) = name {
        map.insert("name".to_string(), Value::String(name.to_string()));
    }
    if let Some(color) = color {
        map.insert("color".to_string(), Value::String(color.to_str().to_string()));
    }
    if let Some(is_favorite) = is_favorite {
        map.insert("is_favorite".to_string(), Value::Bool(is_favorite));
    }
    if let Some(view_style) = view_style {
        map.insert("view_style".to_string(), Value::String(view_style.to_str().to_string()));
    }

    // Turn into JSON
    let json = serde_json::to_string(&map);
    let json = match json {
        Ok(serialised) => serialised,
        Err(err) => return Err(TodoistAPIError::SerdeSerialisationError(err)),
    };

    // Add the new project via the API
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("https://api.todoist.com/rest/v2/projects/{}", id))
        .body(json)
        .header("Content-Type", "application/json")
        .header("X-Request-Id", Uuid::new_v4().hyphenated().to_string())
        .header("Authorization", "Bearer ".to_string() + &user.token)
        .send();

    // Unpack the response, which will hopefully be a Project
    get_from_reqwest_response(response)
}


/// Delete a project with the given ID.
///
/// Just returns Ok(()) if successful.
///
/// <https://developer.todoist.com/rest/v2/#delete-a-project>
pub fn delete_project_by_id(user: &TodoistUser, id: &str) -> Result<(), TodoistAPIError> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(format!("https://api.todoist.com/rest/v2/projects/{}", id))
        .header("Authorization", "Bearer ".to_string() + &user.token)
        .send();
    get_204_from_reqwest_response(response)
}
