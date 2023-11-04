//! Simple wrapper for Todoist's REST v2 API.
//!
//! <https://developer.todoist.com/rest/v2>
//!
//! # Getting Started
//!
//! Before using the Todoist API, you must have an API token. To find the API token for your own
//! user account, go to https://app.todoist.com/app/settings/integrations/developer and
//! view the text box under "API token". This should be a 40-byte hexadecimal number.
//!
//! Once you have your API token, you can create a `TodoistUser` struct, then pass a reference
//! to this to any function in the crate, which will perform the relevant underlying API call.
//!
//! # Examples
//! Here's a basic example which gets and prints all of the user's projects:
//! ```
//! use rustodoist::{TodoistUser, projects};
//! let user = TodoistUser::new("a2a72c2f394b265bb798d5dc4ef55be51443d519");
//! let user_projects = projects::get_projects(&user).expect("Couldn't load project list");
//! println!("{:?}", user_projects);
//! ```
//!
//! Other functions work in a very similar way. A more complex example would be adding a new
//! project. Imagine we want to make a new project with the name "Make Todoist Integration".
//! We want it to be Magenta in colour, and we want it to appear as a board. But we won't want to
//! give it a parent ID (so it will be a top-level project), and we don't want it to be a favourite
//! (so we can either explicitly specify `false` to the API, or we can just not mention it).
//!
//! The full code to do this would be:
//! ```
//! use rustodoist::{TodoistUser, projects, color::Color, projects::ViewStyle};
//! let user = TodoistUser::new("a2a72c2f394b265bb798d5dc4ef55be51443d519");
//! let new_project = projects::add_new_project(
//!     &user,                      // Must give a reference so Todoist knows who we are
//!     "Make Todoist Integration", // Project name is a mandatory &str
//!     None,                       // Don't specify a parent ID, meaning this will be a top-level project
//!     Some(Color::Lavender),      // Explicitly specify Lavender as the colour
//!     None,                       // Don't specify if it's a favorite, meaning it won't be
//!     Some(ViewStyle::Board)      // Set to be a Board view (default would have been List)
//! ).expect("Unable to create new project");
//! println!("Created new project: {:?}", new_project);
//! ```
//!
//! This is a fairly representative sample of how interaction with projects, tasks, sections,
//! comments and labels works.
//!
//! Since basically all of the functions in this crate make an API call which could fail for any
//! number of reasons, they all return a Result with a TodoistAPIError error type. See the `err`
//! module for information on those failure types.
//!

pub mod projects;

/// Defines the error types returned by this crate's functions.
pub mod err;

/// Represents the different colours Todoist offers for projects, labels and filters.
pub mod color;

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
