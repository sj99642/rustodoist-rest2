//! Functions for getting information about your projects, updating, or deleting them.
//!
//! The fundamental operations provided for projects by the API are:
//! - Get all projects (`project::get_projects()`)
//! - Get an project by its ID (`project::get_project_by_id()`)
//! - Create a new project (`project::NewProject::upload()`)
//! - Update a project (`project::UpdateProject::upload()`)
//! - Delete a project (`project::delete_project_by_id()`)
//!
//! The first returns a `Vec` of `Project` structs. The next three each deal with an individual
//! project, and return a single `Project` struct representing the current state of that project
//! after the API request has been made. The final one returns nothing. All functions can return
//! a `TodoistAPIError`.
//!
//! Creating a new project, or updating an existing one, is done by creating an instance of
//! `NewProject` or `UpdateProject` respectively. Optional fields are represented by `Option` types.
//! Create the struct as you need, then run its `update()` method to make the API call. If it is
//! successful, a full `Project` struct will be returned showing the new state of the project.

mod structs;

use crate::TodoistUser;
use crate::err::TodoistAPIError;
use crate::color::Color;
use crate::general::{get_from_reqwest_response, get_204_from_reqwest_response};

pub use crate::projects::structs::view_style::ViewStyle;
pub use crate::projects::structs::project::Project;
pub use crate::projects::structs::new_project::NewProject;

use reqwest;
use serde::Deserialize;
use serde_json::{Value, Map};
use uuid::Uuid;



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


/// Return a single project from its ID.
///
/// <https://developer.todoist.com/rest/v2/#get-a-project>
pub fn get_project_by_id(user: &TodoistUser, id: &str) -> Result<Project, TodoistAPIError> {
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
