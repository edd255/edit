//! TODO: docs

use crate::Position;
use crate::Row;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
#[allow(clippy::partial_pub_fields)]
/// TODO: docs
pub struct Document {
    /// TODO: docs
    rows: Vec<Row>,

    /// TODO: docs
    pub file_name: Option<String>,

    /// TODO: docs
    dirty: bool,
}

/// TODO: docs
impl Document {
    /// # Errors
    /// TODO
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        return Ok(Self {
            rows,
            file_name: Some(filename.to_owned()),
            dirty: false,
        });
    }

    /// TODO: docs
    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        return self.rows.get(index);
    }

    /// TODO: docs
    #[must_use]
    pub fn is_empty(&self) -> bool {
        return self.rows.is_empty();
    }

    /// TODO: docs
    #[must_use]
    pub fn len(&self) -> usize {
        return self.rows.len();
    }

    /// # Panics
    /// TODO
    pub fn insert(&mut self, position: &Position, character: char) {
        if position.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        if character == '\n' {
            self.insert_newline(position);
            return;
        }
        if position.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, character);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = self.rows.get_mut(position.y).unwrap();
            let row = &mut self.rows[position.y];
            row.insert(position.x, character);
        }
    }

    /// # Panics
    /// TODO
    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, position: &Position) {
        let len = self.rows.len();
        if position.y >= self.rows.len() {
            return;
        }
        self.dirty = true;
        if position.x == self.rows[position.y].len() && position.y + 1 < len {
            let next_row = self.rows.remove(position.y + 1);
            let row = &mut self.rows[position.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[position.y];
            row.delete(position.x);
        }
    }

    /// TODO: docs
    fn insert_newline(&mut self, position: &Position) {
        if position.y > self.rows.len() {
            return;
        }
        if position.y == self.rows.len() {
            self.rows.push(Row::default());
        }
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[position.y].split(position.x);

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(position.y + 1, new_row);
    }

    /// # Errors
    /// TODO
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n");
            }
            self.dirty = false;
        }
        return Ok(());
    }

    /// TODO: docs
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        return self.dirty;
    }
}
