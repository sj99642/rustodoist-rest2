use std::collections::HashMap;
use rustodoist::TodoistUser;
use rustodoist::projects::{get_projects, add_new_project};

use serde_json;
use rustodoist::color::Color;

use std::fs;

fn main() {
    let token = fs::read_to_string("token.txt").expect("Unable to read file").trim().to_string();
    let user = TodoistUser::new(token);
    // let projects = get_projects(&user);




    let new_project = add_new_project(
        &user,
        "New project",
        None,
        Some(Color::Lavender),
        Some(false),
        None
    );
    println!("{:?}", new_project);
}
