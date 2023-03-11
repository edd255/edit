//! TODO: docs

#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::print_stdout, clippy::needless_return)]

mod document;
mod editor;
mod row;
mod terminal;
use document::Document;
use editor::Editor;
use editor::Position;
use row::Row;
use terminal::Terminal;

/// TODO
fn main() {
    Editor::default().run();
}
