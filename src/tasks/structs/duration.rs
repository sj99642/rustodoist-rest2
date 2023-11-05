use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Duration {
    pub amount: u32,
    pub unit: String,
}
