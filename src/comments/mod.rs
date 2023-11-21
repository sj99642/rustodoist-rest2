//! Comments can be added to tasks, projects or sections. From the REST API, it is possible to:
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
