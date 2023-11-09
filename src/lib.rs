//! Simple wrapper for Todoist's REST v2 API.
//!
//! <https://developer.todoist.com/rest/v2>
//!
//! # Getting Started
//!
//! Before using the Todoist API, you must have an API token. To find the API token for your own
//! user account, go to <https://app.todoist.com/app/settings/integrations/developer> and
//! view the text box under "API token". This should be a 40-byte hexadecimal number.
//!
//! Once you have your API token, you can create a `TodoistUser` struct, then pass a reference
//! to this to any function in the crate, which will perform the relevant underlying API call.
//!
//! # Examples
//! Here's a basic example which gets and prints all of the user's projects:
//! ```
//! use rustodoist_rest2::{TodoistUser, projects};
//! let user = TodoistUser::new("a2a72c2f394b265bb798d5dc4ef55be51443d519");
//! let user_projects = projects::get_projects(&user).expect("Couldn't load project list");
//! println!("{:?}", user_projects);
//! ```
//!
//! There are five different kinds of objects: projects, tasks, sections, comments and labels.
//! For each object type, there are functions to get a list of all objects of that type, to get
//! a single object by its ID, to add a new object, to update an existing object, and to delete
//! an existing object. The "getter" functions are located in the top level of their relevant
//! module, prefixed with `get_`. To update an existing object or create a new one, there will be
//! a struct for that purpose; fill in the fields you want to change, then run upload() on it.
//!
//! For example, imagine we want to make a new project with the name "Make Todoist Integration".
//! We want it to be Magenta in colour, and we want it to appear as a board. But we won't want to
//! give it a parent ID (so it will be a top-level project), and we don't want it to be a favourite
//! (so we can either explicitly specify `false` to the API, or we can just not mention it). To do
//! this, we create a new `NewProject` struct, fill in the fields we want, then call its
//! `upload()` method. The full code to do this would be:
//! ```
//! use rustodoist_rest2::{TodoistUser, projects, color::Color, projects::ViewStyle};
//! let user = TodoistUser::new("a2a72c2f394b265bb798d5dc4ef55be51443d519");
//! let new_project = projects::NewProject {
//!     name: "Make Todoist Integration".to_string(),
//!     parent_id: None,
//!     color: Some(Color::Magenta),
//!     is_favorite: None,
//!     view_style: Some(ViewStyle::Board),
//! };
//! let created_project = new_project.upload(&user).expect("Unable to create new project");
//! println!("Created new project: {:?}", created_project);
//! ```
//!
//! All of the `upload()` functions return the newly created or updated object, as returned
//! by the API.
//!
//! For each of the five object types, full descriptions of the functions and types are given in
//! the top-level corresponding module.
//!
//! Since basically all of the functions in this crate make an API call which could fail for any
//! number of reasons, they all return a Result with a TodoistAPIError error type. See the `err`
//! module for information on those failure types.


#![warn(missing_docs)]


pub mod projects;

/// Defines the error types returned by this crate's functions.
pub mod err;

/// Represents the different colours Todoist offers for projects, labels and filters.
pub mod color;

pub mod tasks;


pub mod labels;

mod general;


/// Represents a Todoist user, holding the user's API token.
///
/// Create an instance with `TodoistUser::new(<api_token>)`. A reference to a `TodoistUser` must be
/// passed to any function which makes an API request, in order to authenticate the request with
/// Todoist's servers.
pub struct TodoistUser {
    token: String,
}

impl TodoistUser {
    /// Create a new `TodoistUser`, passing in your API token.
    pub fn new(api_token: &str) -> TodoistUser {
        TodoistUser {
            token: api_token.to_string()
        }
    }
}

#[cfg(test)]
mod tests {


}
