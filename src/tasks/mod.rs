//! A set of functions to view, modify, create and delete Todoist tasks.


pub mod structs;

use std::collections::HashMap;
use serde_json::Value;
use crate::err::TodoistAPIError;
use crate::general::get_from_reqwest_response;
use crate::TodoistUser;

pub use structs::task::Task;
pub use structs::duration::Duration;
pub use structs::due::Due;


/// Return a Vec of all the user's active tasks, optionally filtered down. CURRENTLY BROKEN.
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
/// - `project_id` - Only return tasks from this project.
/// - `section_id` - Only return tasks from this section.
/// - `label` - Only return tasks with this label.
///
/// The order of precedence for how the API applies these arguments is:
/// 1. `filter` (with or without `lang`)
/// 2. `ids`
/// 3. `label`/`project_id`/`section_id`
///
/// If a filter is provided, then all the remaining arguments will be ignored (except for the `lang`).
/// If `ids` is provided, then all the remaining arguments will be ignored.
/// `label`, `project_id` and `section_id` are only used if neither `filter` nor `ids` are provided.
pub fn get_active_tasks_pure(
    user: &TodoistUser,
    filter: Option<&str>,
    lang: Option<&str>,
    ids: Option<Vec<&str>>,
    project_id: Option<&str>,
    section_id: Option<&str>,
    label: Option<&str>,
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
        args.insert("ids".to_string(), Value::Array(ids.into_iter().map(|id| Value::String(id.to_string())).collect()));
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

    // Turn this map into JSON, assuming serialisation was successful
    let json = serde_json::to_string(&args);
    let json = match json {
        Ok(serialised) => serialised,
        Err(err) => return Err(TodoistAPIError::SerdeSerialisationError(err)),
    };

    // Make the API request
    let client = reqwest::blocking::Client::new();
    let request = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .body(json)
        .header("Authorization", String::from("Bearer ") + &user.token)
        .header("Content-Type", "application/json");
    println!("Request: {:#?}", request);
    let response = request.send();
    println!("Response: {:#?}", response);

    // Now interpret this response properly
    get_from_reqwest_response(response)
}


/// Get all the user's active tasks, filtered down by some string filter (see
/// <https://todoist.com/help/articles/introduction-to-filters-V98wIH>.
///
/// This is based on `get_active_tasks_pure`.
pub fn get_active_tasks_filtered(
    user: &TodoistUser,
    filter: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    get_active_tasks_pure(user, Some(filter), None, None, None, None, None)
}


/// Downloads all of the user's active tasks
pub fn get_all_active_tasks(user: &TodoistUser) -> Result<Vec<Task>, TodoistAPIError> {
    // Make the API request
    let client = reqwest::blocking::Client::new();
    let request = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .header("Content-Type", "application/json")
        .send();
    get_from_reqwest_response(request)
}


/// Get all the user's active tasks which match the IDs in the given list.
///
/// This is meant to be based on `get_active_tasks_pure`, but for now is based on get_alL_active_tasks.
pub fn get_active_tasks_by_id(
    user: &TodoistUser,
    ids: Vec<&str>) -> Result<Vec<Task>, TodoistAPIError>
{
    Ok(get_all_active_tasks(user)?
        .into_iter()
        .filter(|task| ids.contains(&task.id.as_str()))
        .collect())
}


/// Get all the user's active tasks which are in the given project.
///
/// This should be based on `get_active_tasks_pure`, but is actually based on `get_all_active_tasks`.
pub fn get_active_tasks_by_project(
    user: &TodoistUser,
    project_id: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    Ok(get_all_active_tasks(user)?
        .into_iter()
        .filter(|task| task.project_id == project_id)
        .collect())
}


/// Get all the user's active tasks which are in the given section.
///
/// This should be based on `get_active_tasks_pure`, but is actually based on `get_all_active_tasks`.
pub fn get_active_tasks_by_section(
    user: &TodoistUser,
    section_id: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    Ok(get_all_active_tasks(user)?
        .into_iter()
        .filter(|task| task.section_id == Some(section_id.to_string()))
        .collect())
}


/// Get all the user's active tasks which have the given label.
///
/// This is supposed to be based on `get_active_tasks_pure`, but is actually based on `get_all_active_tasks`.
pub fn get_active_tasks_by_label(
    user: &TodoistUser,
    label: &str) -> Result<Vec<Task>, TodoistAPIError>
{
    Ok(get_all_active_tasks(user)?
        .into_iter()
        .filter(|task| task.labels.contains(&label.to_string()))
        .collect())
}




