//! A set of functions to view, modify, create and delete Todoist tasks.

/// Define the different kinds of structs used to represent/handle tasks.
mod structs;

use std::collections::HashMap;
use serde_json::Value;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::TodoistUser;

pub use structs::task::Task;
pub use structs::duration::Duration;
pub use structs::due::Due;
pub use structs::new_task::{NewTask, NewDue, NewDuration};

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
        .header("Content-Type", "application/json")
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

