pub enum ViewStyle {
    List,
    Board,
}

impl ViewStyle {
    pub fn to_str(&self) -> &str {
        match self {
            ViewStyle::List => "list",
            ViewStyle::Board => "board"
        }
    }
}
