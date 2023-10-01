use crate::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    grapheme::{Grapheme, Graphemes},
    pane::Pane,
    text::TextBuffer,
};

use super::Editor;

pub struct TextEditor {
    textbuffer: TextBuffer,

    label: Graphemes,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            textbuffer: TextBuffer::new(),
            label: Graphemes::from("❯❯ "),
        }
    }
}

impl Editor for TextEditor {
    fn gen_pane(&self, size: (u16, u16)) -> Pane {
        let mut buf = vec![];
        buf.append(&mut self.label.clone());
        buf.append(&mut self.textbuffer.buf.clone());

        let mut layout = vec![];
        let mut row = Graphemes::default();
        for ch in buf.iter() {
            let width_with_next_char = row.iter().fold(0, |mut layout, g| {
                layout += g.width;
                layout
            }) + ch.width;
            if !row.is_empty() && (size.0 as usize) < width_with_next_char {
                layout.push(row);
                row = Graphemes::default();
            }
            if (size.0 as usize) >= ch.width {
                row.push(ch.clone());
            }
        }
        layout.push(row);
        Pane {
            layout,
            offset: self.textbuffer.position / size.0 as usize,
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            // Move cursor.
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.prev(),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.next(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.to_head(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.to_tail(),

            // Erase char.
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.erase(),

            // Input char.
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => self.textbuffer.insert(Grapheme::from(*ch)),

            _ => [TextBuffer::new(), TextBuffer::new()],
        };
    }

    fn reset(&mut self) {
        self.textbuffer = TextBuffer::new();
    }

    fn to_string(&self) -> String {
        self.textbuffer.to_string()
    }
}
