//! TODO: docs

use crate::SearchDirection;
use core::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
/// TODO: docs
pub struct Row {
    /// TODO: docs
    string: String,

    /// TODO: docs
    len: usize,
}

/// TODO: docs
impl From<&str> for Row {
    /// TODO: docs
    fn from(slice: &str) -> Self {
        return Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        };
    }
}

impl Row {
    /// TODO: Handle tabs and spaces properly.
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string();
        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }
        return result;
    }

    /// TODO: docs
    #[must_use]
    pub fn len(&self) -> usize {
        return self.len;
    }

    /// TODO: docs
    #[must_use]
    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    /// TODO: docs
    pub fn insert(&mut self, position: usize, character: char) {
        if position >= self.len() {
            self.string.push(character);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == position {
                length += 1;
                result.push(character);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    /// TODO: docs
    pub fn delete(&mut self, position: usize) {
        if position >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != position {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    /// TODO: docs
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    /// TODO: docs
    #[must_use]
    pub fn split(&mut self, position: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < position {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }
        self.string = row;
        self.len = length;
        return Self {
            string: splitted_row,
            len: splitted_length,
        };
    }

    /// TODO: docs
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        return self.string.as_bytes();
    }

    /// TODO: docs
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len {
            return None;
        }
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forwarf {
            self.len
        } else {
            at
        };
        #[allow(clippy::integer_arithmetic)]
        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
        }
        return None;
    }
}
