use crate::{
    crossterm::style::{Attribute, Attributes, Color, ContentStyle},
    style::Style,
};

pub struct Theme {
    pub title_style: ContentStyle,
    pub item_style: ContentStyle,
    pub folded_symbol: String,
    pub unfolded_symbol: String,
    pub cursor_style: ContentStyle,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            title_style: Style::new()
                .attrs(Attributes::from(Attribute::Bold))
                .build(),
            item_style: Style::new().build(),
            folded_symbol: String::from("▶︎ "),
            unfolded_symbol: String::from("▼ "),
            cursor_style: Style::new().fgc(Color::DarkCyan).build(),
        }
    }
}
