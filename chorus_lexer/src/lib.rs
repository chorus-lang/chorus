//! A lot of the parser/lexer code is a modified version of the rustc parser

mod cursor;

use cursor::Cursor;

use self::TokenKind::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenKind {
    // TODO: implement literals
    // literals
    /// 0-9
    Int(usize),
    /// FLoating point integer
    Float(f64),
    /// String literal
    Str(String),
    /// Boolean literal
    Bool(bool),

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
    /// `=`
    Assign,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `==`
    Eq,
    /// `<=`
    Lte,
    /// `>=`
    Gte,

    /// `;`
    Semi,

    /// Any whitespace character
    Whitespace,
    /// Unknown token
    Unknown,
}

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        let token = Cursor::new(input).advance_token();
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
                }
                '*' => {
                    self.bump().unwrap();
                    BlockComment { terminated: true }
                }
                _ => Slash,
            },
            // Gte/Lte/Eq
            '=' => match self.first() {
                '=' => {
                    self.bump().unwrap();
                    Eq
                }
                _ => Assign,
            },

            // single char tokens
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '%' => Percent,
            ';' => Semi,
            '<' => match self.first() {
                '=' => {
                    self.bump().unwrap();
                    Lte
                },
                _ => Lt
            }
            '>' => match self.first() {
                '=' => {
                    self.bump().unwrap();
                    Gte
                },
                _ => Gt
            }
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

    const SINGLE_CHAR_TEST_STR: &'static str = "+-*/%;<>";
    const DOUBLE_CHAR_TEST_STR: &'static str = "<=>===";

    #[test]
    fn create_token() {
        let new_token = Token::new(TokenKind::Plus, 1);

        assert_eq!(new_token.kind, TokenKind::Plus);
        assert_eq!(new_token.len, 1);
    }

    #[test]
    fn single_char_tokens() {
        let tokens = tokenize(SINGLE_CHAR_TEST_STR).collect::<Vec<Token>>();
        let expected =
            construct_test![Plus, Minus, Star, Slash, Percent, Semi, Lt, Gt];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn double_char_tokens() {
        let tokens = tokenize(DOUBLE_CHAR_TEST_STR).collect::<Vec<Token>>();
        let expected = construct_test![Lte, Gte, Eq];

        assert_eq!(tokens, expected);
    }
}
