use rustodoist::TodoistUser;
use rustodoist::projects;
use rustodoist::color::Color;
use rustodoist::projects::ViewStyle;

use std::fs;

fn main() {
    let token = fs::read_to_string("token.txt").expect("Unable to read file").trim().to_string();
    let user = TodoistUser::new(token);
    let new_project = projects::add_new_project(
        &user,
         "New project",
        None,
        Some(Color::Lavender),
        Some(false),
        Some(ViewStyle::List),
    );
    println!("New project: {:?}", new_project);

    if let Ok(proj) = new_project {
        let result = projects::delete_project_by_id(&user, proj.id.as_str());
        println!("Deletion result: {:?}", result);
    }
}
