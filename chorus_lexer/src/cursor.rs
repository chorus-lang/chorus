#![allow(dead_code)]
use std::str::Chars;

pub(crate) struct Cursor<'a> {
    start_len: usize,
    chars: Chars<'a>,
    prev: char,
    pub ln: i32,
    pub col: i32
}

pub(crate) const EOF: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            start_len: input.len(),
            chars: input.chars(),
            prev: EOF,
            ln: 1,
            col: 1
        }
    }

    pub(crate) fn prev(&self) -> char {
        self.prev
    }

    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF)
    }

    pub(crate) fn first(&self) -> char {
        self.nth_char(0)
    }

    pub(crate) fn second(&self) -> char {
        self.nth_char(1)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn len_consumed(&self) -> usize {
        self.start_len - self.chars.as_str().len()
    }

    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        self.prev = c;
        if c == '\n' {
            self.ln  += 1;
            self.col  = 1;
        } else {
            self.col += 1
        }

        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &'static str = r#"let test_str =  "this is a test str";"#;

    #[test]
    fn create_cursor() {
        Cursor::new(TEST_STR);
    }

    #[test]
    fn cursor_prev_eof() {
        let cursor = Cursor::new(TEST_STR);

        assert_eq!(cursor.prev(), EOF);
    }

    #[test]
    fn cursor_prev_char() {
        let mut cursor = Cursor::new(TEST_STR);

        cursor.bump();

        assert_eq!(cursor.prev(), 'l');
    }

    #[test]
    fn cursor_first_char() {
        let cursor = Cursor::new(TEST_STR);

        assert_eq!(cursor.first(), 'l');
    }

    #[test]
    fn cursor_second_char() {
        let cursor = Cursor::new(TEST_STR);

        assert_eq!(cursor.second(), 'e');
    }
}
