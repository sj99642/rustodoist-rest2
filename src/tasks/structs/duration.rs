use serde::Deserialize;

/// Represents the duration of a task. The possible `unit` values are "minute" or "day".
/// The "amount" field is the number of minutes or days, depending on the unit.
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Duration {
    pub amount: u32,
    pub unit: String,
}
