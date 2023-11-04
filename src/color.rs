/// The available colours for Todoist projects, labels and filters.
///
/// Corresponds to the colours in <https://developer.todoist.com/guides/#colors>
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
}
