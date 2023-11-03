use crate::TodoistUser;
use crate::err::TodoistAPIError;

use reqwest;
use serde::Deserialize;

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
