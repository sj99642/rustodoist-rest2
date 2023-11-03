use std::collections::HashMap;
use rustodoist::TodoistUser;
use rustodoist::projects::{get_projects, add_new_project};

use serde_json;
use rustodoist::color::Color;

fn main() {
    let user = TodoistUser::new(String::from("3235ffe67584ea231adca143d09d1a48ef74f2fa"));
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
