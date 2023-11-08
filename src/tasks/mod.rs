//! Functions for getting information about tasks, updating, deleting, or closing/reopening them.
//!
//! The operations provided for tasks by the API are:
//! - Get all active tasks (`tasks::get_all_active_tasks()`)
//! - Get active tasks filtered by text filter, project/section/label, or task ID
//!   (`tasks::get_active_tasks_by...`)
//! - Get an individual task by its ID (`task::get_individual_task_by_id()`)
//! - Create a new task (`tasks::NewTask::upload()`)
//! - Update an existing task (`tasks::UpdateTask::upload()`)
//! - Delete a task (`project::delete_task_by_id()`)
//! - Close/complete a task (`project::close_task_by_id()`)
//! - Reopen a closed task (`project::reopen_task_by_id()`)
//!
//! The first two return a `Vec` of `Task` structs. The next three each return a single `Task`.
//! The final ones just return `()` in the case of success.
//!
//! Creating a new task, or updating an existing one, is done by creating an instance of
//! `NewTask` or `UpdateTask` respectively. Optional fields are represented by `Option` types.
//! Create the struct as you need, then run its `update()` method to make the API call. If it is
//! successful, a full `Task` struct will be returned showing the new state of the task.

/// Define the different kinds of structs used to represent/handle tasks.
mod structs;

use std::collections::HashMap;
use serde_json::Value;
use crate::err::TodoistAPIError;
use crate::general::{get_from_reqwest_response, get_204_from_reqwest_response};
use crate::TodoistUser;

pub use structs::task::{Task, Due, Duration};
pub use structs::new_task::{NewTask, NewDue, NewDuration};
pub use structs::update_task::UpdateTask;

/// Return a Vec of all the user's active tasks, optionally filtered down.
///
/// Note that the other `get_active_tasks` functions may be easier to use as they don't require
/// all of the arguments, which cannot all be used together anyway.
///
/// This does not currently work, because I haven't yet worked out how to send the arguments to
/// this GET request. Some of the other get_active_tasks functions work, because we download all the
/// tasks then filter them afterwards.
///
/// Works according to the API description at
/// <https://developer.todoist.com/rest/v2?shell#get-active-tasks>. None of the arguments are
/// mandatory. If all arguments are None, then this will return all active tasks.
///
/// # Arguments
/// - `filter` - A filter string (see <https://todoist.com/help/articles/introduction-to-filters-V98wIH>).
/// - `lang` - The language to use for the filter, if not English.
/// - `ids` - An explicit list of task IDs to return.
/// `label` - Only return tasks with this label.
/// - `project_id` - Only return tasks from this project.
/// - `section_id` - Only return tasks from this section.
///
/// The order of precedence for how the API applies these arguments is:
/// 1. `filter` (with or without `lang`)
/// 2. `ids`
/// 3. `label`
/// 4. `project_id`
/// 5. `section_id`
///
/// If a filter is provided, then all the remaining arguments will be ignored (except for the `lang`).
/// If `ids` is provided, then all the remaining arguments will be ignored, etc. So only one argument
/// is ever used.
fn get_active_tasks_pure(
    user: &TodoistUser,
    filter: Option<&str>,
    lang: Option<&str>,
    ids: Option<Vec<&str>>,
    label: Option<&str>,
    project_id: Option<&str>,
    section_id: Option<&str>,
) -> Result<Vec<Task>, TodoistAPIError> {
    // Construct the arguments of the request
    let mut args = HashMap::new();
    if let Some(filter) = filter {
        args.insert("filter".to_string(), Value::String(filter.to_string()));
    }
    if let Some(lang) = lang {
        args.insert("lang".to_string(), Value::String(lang.to_string()));
    }
    if let Some(ids) = ids {
        args.insert("ids".to_string(), Value::String(ids.join(",")));
    }
    if let Some(project_id) = project_id {
        args.insert("project_id".to_string(), Value::String(project_id.to_string()));
    }
    if let Some(section_id) = section_id {
        args.insert("section_id".to_string(), Value::String(section_id.to_string()));
    }
    if let Some(label) = label {
        args.insert("label".to_string(), Value::String(label.to_string()));
    }

    // Make the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .query(&args)
        .send();

    // Now interpret this response properly
    get_from_reqwest_response(response)
}


/// Get all the user's active tasks which match the given string filter, in English (see
/// <https://todoist.com/help/articles/introduction-to-filters-V98wIH>.
pub fn get_active_tasks_filtered(
    user: &TodoistUser,
    filter: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, Some(filter), None, None, None, None, None)
}


/// Get all the user's active tasks which match the given string filter, written in the given language.
///
/// The language must be given as an IETF language tag (e.g. "es", "fr", "de"; see
/// <https://en.wikipedia.org/wiki/IETF_language_tag>).
pub fn get_active_tasks_filtered_non_english(
    user: &TodoistUser,
    filter: &str,
    lang: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, Some(filter), Some(lang), None, None, None, None)
}


/// Get all the tasks with an ID in this list.
pub fn get_active_tasks_by_id(
    user: &TodoistUser,
    ids: Vec<&str>) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, None, None, Some(ids), None, None, None)
}


/// Get all the user's active tasks which are in the given project.
pub fn get_active_tasks_by_project(
    user: &TodoistUser,
    project_id: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, None, None, None, None, Some(project_id), None)
}


/// Get all the user's active tasks which are in the given section.
pub fn get_active_tasks_by_section(
    user: &TodoistUser,
    section_id: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, None, None, None, None, None, Some(section_id))
}


/// Get all the user's active tasks which have the given label.
pub fn get_active_tasks_by_label(
    user: &TodoistUser,
    label: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, None, None, None, Some(label), None, None)
}


/// Get all the user's active tasks.
pub fn get_all_active_tasks(user: &TodoistUser) -> Result<Vec<Task>, TodoistAPIError> {
    get_active_tasks_pure(user, None, None, None, None, None, None)
}


/// Get the individual task with the given ID.
pub fn get_individual_task_by_id(
    user: &TodoistUser,
    id: &str
) -> Result<Task, TodoistAPIError> {
    // Make the API request and interpret the response
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://api.todoist.com/rest/v2/tasks/{}", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_from_reqwest_response(response)
}


/// Mark a task as complete. Returns an empty tuple in case of success.
pub fn close_task_by_id(
    user: &TodoistUser,
    id: &str,
) -> Result<(), TodoistAPIError> {
    // Make the API request and interpret the response
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("https://api.todoist.com/rest/v2/tasks/{}/close", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_204_from_reqwest_response(response)
}


/// Reopen a closed task. Returns an empty tuple in case of success.
pub fn reopen_task_by_id(
    user: &TodoistUser,
    id: &str,
) -> Result<(), TodoistAPIError> {
    // Make the API request and interpret the response
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("https://api.todoist.com/rest/v2/tasks/{}/reopen", id))
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();
    get_204_from_reqwest_response(response)
}
