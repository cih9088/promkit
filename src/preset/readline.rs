use crate::{
    crossterm::{
        event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
        style::{Attribute, Attributes, Color, ContentStyle},
    },
    error::Result,
    render::{Renderable, State},
    style::Style,
    text,
    text_editor::{self, History, Mode, Suggest},
    validate::Validator,
    Prompt,
};

mod confirm;
pub use confirm::Confirm;
mod password;
pub use password::Password;

/// The `Readline` struct provides functionality for reading a single line of input from the user.
/// It supports various configurations such as input masking, history, suggestions, and custom styles.
pub struct Readline {
    /// Renderer for the title displayed above the input field.
    title_renderer: text::Renderer,
    /// Renderer for the text editor where user input is entered.
    text_editor_renderer: text_editor::Renderer,
    /// Renderer for displaying error messages based on input validation.
    error_message_renderer: text::Renderer,
    /// Optional validator for input validation with custom error messages.
    validator: Option<Validator<str>>,
}

impl Default for Readline {
    fn default() -> Self {
        Self {
            title_renderer: text::Renderer {
                text: Default::default(),
                style: Style::new()
                    .attrs(Attributes::from(Attribute::Bold))
                    .build(),
            },
            text_editor_renderer: text_editor::Renderer {
                texteditor: Default::default(),
                history: Default::default(),
                suggest: Default::default(),
                ps: String::from("❯❯ "),
                mask: Default::default(),
                ps_style: Style::new().fgc(Color::DarkGreen).build(),
                active_char_style: Style::new().bgc(Color::DarkCyan).build(),
                inactive_char_style: Style::new().build(),
                edit_mode: Default::default(),
                lines: Default::default(),
            },
            error_message_renderer: text::Renderer {
                text: Default::default(),
                style: Style::new()
                    .fgc(Color::DarkRed)
                    .attrs(Attributes::from(Attribute::Bold))
                    .build(),
            },
            validator: Default::default(),
        }
    }
}

impl Readline {
    /// Sets the title text displayed above the input field.
    pub fn title<T: AsRef<str>>(mut self, text: T) -> Self {
        self.title_renderer.text = text.as_ref().to_string();
        self
    }

    /// Sets the style for the title text.
    pub fn title_style(mut self, style: ContentStyle) -> Self {
        self.title_renderer.style = style;
        self
    }

    /// Enables suggestion functionality with the provided `Suggest` instance.
    pub fn enable_suggest(mut self, suggest: Suggest) -> Self {
        self.text_editor_renderer.suggest = suggest;
        self
    }

    /// Enables history functionality allowing navigation through previous inputs.
    pub fn enable_history(mut self) -> Self {
        self.text_editor_renderer.history = Some(History::default());
        self
    }

    /// Sets the prefix string displayed before the input text.
    pub fn prefix_string<T: AsRef<str>>(mut self, ps: T) -> Self {
        self.text_editor_renderer.ps = ps.as_ref().to_string();
        self
    }

    /// Sets the character used for masking input text, typically used for password fields.
    pub fn mask(mut self, mask: char) -> Self {
        self.text_editor_renderer.mask = Some(mask);
        self
    }

    /// Sets the style for the prefix string.
    pub fn prefix_string_style(mut self, style: ContentStyle) -> Self {
        self.text_editor_renderer.ps_style = style;
        self
    }

    /// Sets the style for the currently active character in the input field.
    pub fn active_char_style(mut self, style: ContentStyle) -> Self {
        self.text_editor_renderer.active_char_style = style;
        self
    }

    /// Sets the style for characters that are not currently active in the input field.
    pub fn inactive_char_style(mut self, style: ContentStyle) -> Self {
        self.text_editor_renderer.inactive_char_style = style;
        self
    }

    /// Sets the edit mode for the text editor, either insert or overwrite.
    pub fn edit_mode(mut self, mode: Mode) -> Self {
        self.text_editor_renderer.edit_mode = mode;
        self
    }

    /// Sets the number of lines available for rendering the text editor.
    pub fn text_editor_lines(mut self, lines: usize) -> Self {
        self.text_editor_renderer.lines = Some(lines);
        self
    }

    /// Configures a validator for the input with a function to validate the input and another to configure the error message.
    pub fn validator<V, F>(mut self, validator: V, error_message_configure: F) -> Self
    where
        V: Fn(&str) -> bool + 'static,
        F: Fn(&str) -> String + 'static,
    {
        self.validator = Some(Validator::new(validator, error_message_configure));
        self
    }

    /// Initiates the prompt process, displaying the configured UI elements and handling user input.
    pub fn prompt(self) -> Result<Prompt<String>> {
        let validator = self.validator;

        Prompt::try_new(
            vec![
                Box::new(State::<text::Renderer>::new(self.title_renderer)),
                Box::new(State::<text_editor::Renderer>::new(
                    self.text_editor_renderer,
                )),
                Box::new(State::<text::Renderer>::new(self.error_message_renderer)),
            ],
            move |event: &Event,
                  renderables: &Vec<Box<dyn Renderable + 'static>>|
                  -> Result<bool> {
                let text: String = renderables[1]
                    .as_any()
                    .downcast_ref::<State<text_editor::Renderer>>()
                    .unwrap()
                    .after
                    .borrow()
                    .texteditor
                    .text_without_cursor();

                let error_message_state = renderables[2]
                    .as_any()
                    .downcast_ref::<State<text::Renderer>>()
                    .unwrap();

                let ret = match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    }) => match &validator {
                        Some(validator) => {
                            let ret = validator.validate(&text);
                            if !validator.validate(&text) {
                                error_message_state.after.borrow_mut().text =
                                    validator.error_message(&text);
                            }
                            ret
                        }
                        None => true,
                    },
                    _ => true,
                };
                if ret {
                    *error_message_state.after.borrow_mut() = error_message_state.init.clone();
                }
                Ok(ret)
            },
            |renderables: &Vec<Box<dyn Renderable + 'static>>| -> Result<String> {
                Ok(renderables[1]
                    .as_any()
                    .downcast_ref::<State<text_editor::Renderer>>()
                    .unwrap()
                    .after
                    .borrow()
                    .texteditor
                    .text_without_cursor())
            },
        )
    }
}
