use serde::{Deserialize, Serialize};

/// Represents a project's view styles, namely list vs board
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ViewStyle {
    /// The "list" view, the default
    List,

    /// The "board" view, displaying sections as Kanban columns
    Board,
}

impl ViewStyle {
    /// Convert the ViewStyle to a string, for use in the API
    pub fn to_str(&self) -> &str {
        match self {
            ViewStyle::List => "list",
            ViewStyle::Board => "board"
        }
    }

    /// Convert an API string into a ViewStyle
    pub fn from_str(s: &str) -> ViewStyle {
        match s {
            "board" => ViewStyle::Board,
            _ => ViewStyle::List,
        }
    }
}
