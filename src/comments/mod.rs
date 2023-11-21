//! Comments can be added to tasks or projects. From the REST API, it is possible to:
//!
//! - Get all comments in a given project or task (`get_comments_in_project()` and `get_comments_in_task()`)
//! - Get a single comment by its ID (`get_comment_by_id()`)
//! - Create a new comment (`NewComment::upload()`)
//! - Delete a comment by its ID (`delete_comment_by_id()`)
//! - Update the textual content of a comment (`update_comment_content()`)*
//!
//! * There is no `UpdateComment` struct, as once a comment has been made the only thing that
//!   can be changed is its content.
//!
//! Comments downloaded from the API can contain an attachment, but there is no way to upload
//! an attachment to Todoist's servers using the REST API.


mod structs;

pub use crate::comments::structs::comment::Comment;
pub use crate::comments::structs::attachment::{self, Attachment};
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;

use crate::TodoistUser;


/// Get all the comments attached to the given project
pub fn get_comments_in_project(user: &TodoistUser, project_id: &str) -> Result<Vec<Comment>, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/comments")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .query(&[("project_id", project_id)])
        .send();
    get_from_reqwest_response(response)
}


/// Get all the comments attached to the given task
pub fn get_comments_in_task(user: &TodoistUser, task_id: &str) -> Result<Vec<Comment>, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/comments")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .query(&[("task_id", task_id)])
        .send();
    get_from_reqwest_response(response)
}

/// Get a single comment by its ID
pub fn get_comment_by_id(user: &TodoistUser, comment_id: &str) -> Result<Comment, TodoistAPIError> {
    // Send the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://api.todoist.com/rest/v2/comment/{}", comment_id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}
