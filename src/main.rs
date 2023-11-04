use rustodoist::TodoistUser;
use rustodoist::projects;
use rustodoist::color::Color;

use std::fs;

fn main() {
    let token = fs::read_to_string("token.txt").expect("Unable to read file").trim().to_string();
    let user = TodoistUser::new(token);
    let project = projects::get_project_by_id(&user, "2323023562");
    println!("{:?}", project);
    let updated_project = projects::update_project_by_id(
        &user,
        "2323023562",
        None,
        Some(Color::Green),
        None,
        None
    );
    println!("{:?}", updated_project);
}
