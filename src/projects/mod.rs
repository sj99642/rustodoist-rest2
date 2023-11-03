use std::error::Error;
use crate::TodoistUser;

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

pub fn get_projects(user: &TodoistUser) -> Result<Vec<Project>, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.todoist.com/rest/v2/projects")
        .header("Authorization", String::from("Bearer ") + &user.token)
        .send()?;
    let projects: Vec<Project> = response.json()?;
    Ok(projects)
}
