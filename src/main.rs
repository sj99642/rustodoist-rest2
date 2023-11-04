use std::collections::HashMap;
use rustodoist::TodoistUser;
use rustodoist::projects;

use serde_json;
use rustodoist::color::Color;

use std::fs;

fn main() {
    let token = fs::read_to_string("token.txt").expect("Unable to read file").trim().to_string();
    let user = TodoistUser::new(token);
    let project = projects::get_project_by_id(&user, "2323023562");
    println!("{:?}", project);
}
