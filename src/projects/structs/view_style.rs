use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a project's view styles, namely list vs board
#[derive(Debug)]
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

    /// Convert an API string into a ViewStyle
    pub fn from_str(s: &str) -> ViewStyle {
        match s {
            "board" => ViewStyle::Board,
            _ => ViewStyle::List,
        }
    }
}

impl Serialize for ViewStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.to_str())
    }
}


impl<'de> Deserialize<'de> for ViewStyle {
    fn deserialize<D>(deserializer: D) -> Result<ViewStyle, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(ViewStyle::from_str(&s))
    }
}
