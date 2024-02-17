use crate::grapheme::Graphemes;

/// Represents a pane within a terminal interface, managing a layout of graphemes.
///
/// A `Pane` is essentially a container for text, where the text is represented as a vector of `Graphemes`.
/// It supports operations such as checking if the pane is empty and extracting a subset of graphemes
/// based on the viewport height and other parameters like offset and fixed height.
pub struct Pane {
    /// The layout of graphemes within the pane.
    layout: Vec<Graphemes>,
    /// The offset from the top of the pane, used when extracting graphemes to display.
    offset: usize,
    /// An optional fixed height for the pane. If set, this limits the number of graphemes extracted.
    fixed_height: Option<usize>,
}

impl Pane {
    /// Constructs a new `Pane` with the specified layout, offset, and optional fixed height.
    ///
    /// # Arguments
    ///
    /// * `layout` - A vector of `Graphemes` representing the content of the pane.
    /// * `offset` - The initial offset from the top of the pane.
    /// * `fixed_height` - An optional fixed height for the pane.
    pub fn new(layout: Vec<Graphemes>, offset: usize, fixed_height: Option<usize>) -> Self {
        Pane {
            layout,
            offset,
            fixed_height,
        }
    }

    /// Checks if the pane is empty. A pane is considered empty if it contains a single grapheme with no width.
    ///
    /// # Returns
    ///
    /// Returns `true` if the pane is empty, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.layout.len() == 1 && self.layout[0].widths() == 0
    }

    /// Extracts a subset of graphemes to fit within a specified viewport height.
    ///
    /// This method takes into account the pane's offset and fixed height (if any) to determine
    /// which graphemes to include in the returned vector.
    ///
    /// # Arguments
    ///
    /// * `viewport_height` - The height of the viewport in which the graphemes are to be displayed.
    ///
    /// # Returns
    ///
    /// Returns a vector of `Graphemes` that fit within the specified viewport height.
    pub fn extract(&self, viewport_height: usize) -> Vec<Graphemes> {
        let lines = self.layout.len().min(
            self.fixed_height
                .unwrap_or(viewport_height)
                .min(viewport_height),
        );

        let mut start = self.offset;
        let end = self.offset + lines;
        if end > self.layout.len() {
            start = self.layout.len().saturating_sub(lines);
        }

        return self
            .layout
            .iter()
            .enumerate()
            .filter(|(i, _)| start <= *i && *i < end)
            .map(|(_, row)| row.clone())
            .collect::<Vec<_>>();
    }
}

#[cfg(test)]
mod test {
    mod is_empty {
        use crate::grapheme::matrixify;

        use super::super::*;

        #[test]
        fn test() {
            assert_eq!(
                true,
                Pane {
                    layout: matrixify(10, &Graphemes::new("")),
                    offset: 0,
                    fixed_height: None,
                }
                .is_empty()
            );
        }
    }
    mod extract {
        use super::super::*;

        #[test]
        fn test_with_less_extraction_size_than_layout() {
            let expect = vec![
                Graphemes::new("aa"),
                Graphemes::new("bb"),
                Graphemes::new("cc"),
            ];
            assert_eq!(
                expect,
                Pane {
                    layout: vec![
                        Graphemes::new("aa"),
                        Graphemes::new("bb"),
                        Graphemes::new("cc"),
                        Graphemes::new("dd"),
                        Graphemes::new("ee"),
                    ],
                    offset: 0,
                    fixed_height: None,
                }
                .extract(3)
            );
        }

        #[test]
        fn test_with_much_extraction_size_than_layout() {
            let expect = vec![
                Graphemes::new("aa"),
                Graphemes::new("bb"),
                Graphemes::new("cc"),
                Graphemes::new("dd"),
                Graphemes::new("ee"),
            ];
            assert_eq!(
                expect,
                Pane {
                    layout: vec![
                        Graphemes::new("aa"),
                        Graphemes::new("bb"),
                        Graphemes::new("cc"),
                        Graphemes::new("dd"),
                        Graphemes::new("ee"),
                    ],
                    offset: 0,
                    fixed_height: None,
                }
                .extract(10)
            );
        }

        #[test]
        fn test_with_within_extraction_size_and_offset_non_zero() {
            let expect = vec![Graphemes::new("cc"), Graphemes::new("dd")];
            assert_eq!(
                expect,
                Pane {
                    layout: vec![
                        Graphemes::new("aa"),
                        Graphemes::new("bb"),
                        Graphemes::new("cc"),
                        Graphemes::new("dd"),
                        Graphemes::new("ee"),
                    ],
                    offset: 2, // indicate `cc`
                    fixed_height: None,
                }
                .extract(2)
            );
        }

        #[test]
        fn test_with_beyond_extraction_size_and_offset_non_zero() {
            let expect = vec![
                Graphemes::new("cc"),
                Graphemes::new("dd"),
                Graphemes::new("ee"),
            ];
            assert_eq!(
                expect,
                Pane {
                    layout: vec![
                        Graphemes::new("aa"),
                        Graphemes::new("bb"),
                        Graphemes::new("cc"),
                        Graphemes::new("dd"),
                        Graphemes::new("ee"),
                    ],
                    offset: 3, // indicate `dd`
                    fixed_height: None,
                }
                .extract(3)
            );
        }

        #[test]
        fn test_with_small_fixed_height_and_beyond_extraction_size_and_offset_non_zero() {
            let expect = vec![
                Graphemes::new("bb"),
                Graphemes::new("cc"),
                Graphemes::new("dd"),
                Graphemes::new("ee"),
            ];
            assert_eq!(
                expect,
                Pane {
                    layout: vec![
                        Graphemes::new("aa"),
                        Graphemes::new("bb"),
                        Graphemes::new("cc"),
                        Graphemes::new("dd"),
                        Graphemes::new("ee"),
                    ],
                    offset: 3, // indicate `dd`
                    fixed_height: Some(5),
                }
                .extract(4)
            );
        }

        #[test]
        fn test_with_large_fixed_height_and_beyond_extraction_size_and_offset_non_zero() {
            let expect = vec![
                Graphemes::new("bb"),
                Graphemes::new("cc"),
                Graphemes::new("dd"),
                Graphemes::new("ee"),
            ];
            assert_eq!(
                expect,
                Pane {
                    layout: vec![
                        Graphemes::new("aa"),
                        Graphemes::new("bb"),
                        Graphemes::new("cc"),
                        Graphemes::new("dd"),
                        Graphemes::new("ee"),
                    ],
                    offset: 3, // indicate `dd`
                    fixed_height: Some(4),
                }
                .extract(5)
            );
        }
    }
}
