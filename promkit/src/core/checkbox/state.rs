use crate::{crossterm::style::ContentStyle, grapheme::StyledGraphemes, pane::Pane, PaneFactory};

use super::Checkbox;

/// Represents the state of a `Checkbox` component.
///
/// This state includes not only the checkbox itself but also various attributes
/// that determine how the checkbox and its items are displayed. These attributes
/// include symbols for indicating active and inactive items, styles for selected
/// and unselected lines, and the number of lines available for rendering.
#[derive(Clone)]
pub struct State {
    /// The `Checkbox` component to be rendered.
    pub checkbox: Checkbox,

    /// Symbol for the selected line.
    pub cursor: String,

    /// Symbol used to indicate an active (selected) checkbox item.
    pub active_mark: char,
    /// Symbol used to indicate an inactive (unselected) checkbox item.
    pub inactive_mark: char,

    /// Style for the selected line.
    pub active_item_style: ContentStyle,
    /// Style for unselected lines.
    pub inactive_item_style: ContentStyle,

    /// Number of lines available for rendering.
    pub lines: Option<usize>,
}

impl PaneFactory for State {
    fn create_pane(&self, width: u16, height: u16) -> Pane {
        let f = |idx: usize, item: &String| -> String {
            if self.checkbox.picked_indexes().contains(&idx) {
                format!("{} {}", self.active_mark, item)
            } else {
                format!("{} {}", self.inactive_mark, item)
            }
        };

        let height = match self.lines {
            Some(lines) => lines.min(height as usize),
            None => height as usize,
        };

        let matrix = self
            .checkbox
            .items()
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                *i >= self.checkbox.position() && *i < self.checkbox.position() + height
            })
            .map(|(i, item)| {
                if i == self.checkbox.position() {
                    StyledGraphemes::from_str(
                        format!("{}{}", self.cursor, f(i, item)),
                        self.active_item_style,
                    )
                } else {
                    StyledGraphemes::from_str(
                        format!(
                            "{}{}",
                            " ".repeat(StyledGraphemes::from(self.cursor.clone()).widths()),
                            f(i, item)
                        ),
                        self.inactive_item_style,
                    )
                }
            })
            .map(|row| row.truncate_to_width(width as usize))
            .collect::<Vec<StyledGraphemes>>();

        Pane::new(matrix, 0)
    }
}
