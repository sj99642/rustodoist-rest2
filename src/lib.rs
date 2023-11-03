pub mod projects;
pub mod err;
pub mod color;

mod general;

pub struct TodoistUser {
    token: String,
}

impl TodoistUser {
    pub fn new(api_token: String) -> TodoistUser {
        TodoistUser {
            token: api_token
        }
    }
}

#[cfg(test)]
mod tests {


}
