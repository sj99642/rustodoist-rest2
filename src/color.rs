use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The available colours for Todoist projects, labels and filters.
///
/// Corresponds to the colours in <https://developer.todoist.com/guides/#colors>
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Color {
    BerryRed,
    Red,
    Orange,
    Yellow,
    OliveGreen,
    LimeGreen,
    Green,
    MintGreen,
    Teal,
    SkyBlue,
    LightBlue,
    Blue,
    Grape,
    Violet,
    Lavender,
    Magenta,
    Salmon,
    Charcoal,
    Grey,
    Taupe,
}


impl Color {
    /// Return a colour type from its numerical ID
    pub fn from_id(id: u8) -> Option<Color> {
        match id {
            30 => Some(Color::BerryRed),
            31 => Some(Color::Red),
            32 => Some(Color::Orange),
            33 => Some(Color::Yellow),
            34 => Some(Color::OliveGreen),
            35 => Some(Color::LimeGreen),
            36 => Some(Color::Green),
            37 => Some(Color::MintGreen),
            38 => Some(Color::Teal),
            39 => Some(Color::SkyBlue),
            40 => Some(Color::LightBlue),
            41 => Some(Color::Blue),
            42 => Some(Color::Grape),
            43 => Some(Color::Violet),
            44 => Some(Color::Lavender),
            45 => Some(Color::Magenta),
            46 => Some(Color::Salmon),
            47 => Some(Color::Charcoal),
            48 => Some(Color::Grey),
            49 => Some(Color::Taupe),
            _ => None,
        }
    }

    /// Returns the string which represents this colour in the Todoist API
    pub fn to_str(&self) -> &str {
        match self {
            Color::BerryRed => "berry_red",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Yellow => "yellow",
            Color::OliveGreen => "olive_green",
            Color::LimeGreen => "lime_green",
            Color::Green => "green",
            Color::MintGreen => "mint_green",
            Color::Teal => "teal",
            Color::SkyBlue => "sky_blue",
            Color::LightBlue => "light_blue",
            Color::Blue => "blue",
            Color::Grape => "grape",
            Color::Violet => "violet",
            Color::Lavender => "lavender",
            Color::Magenta => "magenta",
            Color::Salmon => "salmon",
            Color::Charcoal => "charcoal",
            Color::Grey => "grey",
            Color::Taupe => "taupe",
        }
    }

    /// Returns the Color corresponding to the string
    pub fn from_str(s: &str) -> Option<Color> {
        match s {
            "berry_red" => Some(Color::BerryRed),
            "red" => Some(Color::Red),
            "orange" => Some(Color::Orange),
            "yellow" => Some(Color::Yellow),
            "olive_green" => Some(Color::OliveGreen),
            "lime_green" => Some(Color::LimeGreen),
            "green" => Some(Color::Green),
            "mint_green" => Some(Color::MintGreen),
            "teal" => Some(Color::Teal),
            "sky_blue" => Some(Color::SkyBlue),
            "light_blue" => Some(Color::LightBlue),
            "blue" => Some(Color::Blue),
            "grape" => Some(Color::Grape),
            "violet" => Some(Color::Violet),
            "lavender" => Some(Color::Lavender),
            "magenta" => Some(Color::Magenta),
            "salmon" => Some(Color::Salmon),
            "charcoal" => Some(Color::Charcoal),
            "grey" => Some(Color::Grey),
            "taupe" => Some(Color::Taupe),
            _ => None,
        }
    }
}



impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.to_str())
    }
}


impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        let color = Color::from_str(&s);
        match color {
            Some(color) => Ok(color),
            None => Err(serde::de::Error::custom("Invalid color")),
        }
    }
}


