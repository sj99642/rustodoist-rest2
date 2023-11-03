use crate::TodoistUser;
use crate::err::TodoistAPIError;

use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    id: String,
    name: String,
    color: String,
    parent_id: Option<String>,
    order: i32,
    comment_count: i32,
    is_shared: bool,
    is_favorite: bool,
    is_inbox_project: bool,
    view_style: String,
    url: String,
}

pub fn get_projects(user: &TodoistUser) -> Result<Vec<Project>, TodoistAPIError> {
    // Make the API request
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/projects")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send();

    // Unpack the meaning of the response, making sure there was no error in making the HTTP request
    let response = match response {
        Ok(rsp) => rsp,
        Err(err) => return Err(TodoistAPIError::ReqwestRequestError(err)),
    };

    // We got an HTTP response; was it successful?
    if !response.status().is_success() {
        return Err(TodoistAPIError::UnsuccessfulHTTPStatus(response.status()));
    }

    // We got a successful HTTP response. That means it *should* be proper JSON we can deserialise
    let projects= response.json::<Vec<Project>>();
    match projects {
        Ok(projs) => Ok(projs),
        Err(err) => Err(TodoistAPIError::ReqwestDeserialisationError(err))
    }
}
