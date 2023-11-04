/// Represents a project's view styles, namely list vs board
pub enum ViewStyle {
    List,
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
}
