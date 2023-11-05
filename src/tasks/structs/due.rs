use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Due {
    pub string: String,
    pub date: String,
    pub is_recurring: bool,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}
