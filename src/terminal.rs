//! TODO: docs

use crate::Position;
use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// TODO: docs
pub struct Size {
    /// TODO: docs
    pub width: u16,

    /// TODO: docs
    pub height: u16,
}

/// TODO: docs
pub struct Terminal {
    /// TODO: docs
    size: Size,

    /// TODO: docs
    _stdout: RawTerminal<std::io::Stdout>,
}

/// TODO: docs
impl Terminal {
    /// # Errors
    /// TODO
    pub fn standard() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        return Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        });
    }

    /// TODO: docs
    #[must_use]
    pub fn size(&self) -> &Size {
        return &self.size;
    }

    /// TODO: docs
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    /// TODO: docs
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// # Errors
    /// TODO
    pub fn flush() -> Result<(), std::io::Error> {
        return io::stdout().flush();
    }

    /// # Errors
    /// TODO
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    /// TODO: docs
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    /// TODO: docs
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    /// TODO: docs
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    /// TODO: docs
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    /// TODO: docs
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    /// TODO: docs
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    /// TODO: docs
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
