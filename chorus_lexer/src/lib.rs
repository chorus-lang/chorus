//! A lot of the parser/lexer code is a modified version of the rustc parser

mod cursor;

use cursor::Cursor;

use self::TokenKind::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// `//`
    Comment,
    /// `/*` `*/`
    BlockComment { terminated: bool },

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,

    /// `;`
    Semi,

    /// Any whitespace character
    Whitespace,
    /// Unknown token
    Unknown,
}

pub fn first_token(input: &str) -> Token {
    Cursor::new(input).advance_token()
}

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        let token = first_token(input);
        input = &input[token.len..];

        Some(token)
    })
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            // Comments
            '/' => match self.first() {
                '/' => {
                    self.bump().unwrap();
                    Comment
                },
                '*' => {
                    self.bump().unwrap();
                    BlockComment { terminated: true }
                },
                _ => Slash,
            },

            // single char tokens
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '%' => Percent,
            ';' => Semi,

            _ => Unknown,
        };

        Token::new(token_kind, self.len_consumed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An easier way to construct a vector of tokens
    /// to be used in unit tests
    macro_rules! construct_test {
        // construct_test![(TokenKind, 1), (TokenKind, 2)]
        ( $(($typ:tt, $len:expr)),* $(,)?) => {
            vec![$(Token { kind: $typ, len: $len },)*]
        };

        // construct_test![TokenKind, TokenKind, TokenKind]
        ($($typ:tt),* $(,)?) => {
            construct_test![$(($typ, 1),)*]
        };
    }

    const SINGLE_CHAR_TEST_STR: &'static str = "+-*/%;";

    #[test]
    fn create_token() {
        let new_token = Token::new(TokenKind::Plus, 1);

        assert_eq!(new_token.kind, TokenKind::Plus);
        assert_eq!(new_token.len, 1);
    }

    #[test]
    fn single_char_tokens() {
        let tokens = tokenize(SINGLE_CHAR_TEST_STR).collect::<Vec<Token>>();
        let expected = construct_test![Plus, Minus, Star, Slash, Percent, Semi];

        assert_eq!(tokens, expected);
    }
}
