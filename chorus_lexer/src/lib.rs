//! A lot of the parser/lexer code is a modified version of the rustc parser

mod cursor;

use colored::*;
use regex::Regex;

// use cursor::Cursor;

use self::Tokens::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Token {
    kind: TokenKind,
    len: usize
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub kind: TokenKind,
    pub re: Regex,
}

pub struct Lexer {
    rules: Vec<Rule>,
}

// impl Token {
//     fn new(kind: TokenKind, len: usize) -> Self {
//         Self { kind, len }
//     }
// }

pub const VARIABLE_NAME_VALIDATION: [&str; 8] =
    ["/", "+", "-", "%", "=", "<", ">", "="];
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TokenKind(pub Tokens);

#[derive(Clone, Debug, PartialEq, PartialOrd)] #[repr(u16)]
pub enum Tokens {
    // / `//`
    Comment,
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
    /// ()
    OpenBracket,
    ClosedBracket,
    /// keywords
    Let,
    Function,
    /// Identifiers
    Identifier(std::string::String),
    TakesValue,
    Type,
    /// Types
    String,
    Float,
    Int,
    Bool,
    /// Typed values
    Num(i128),
    Floating(f64),
    Boolean(bool),
    String_(std::string::String),
    Dyn(std::string::String),
    /// A newline
    Newline,
    /// Any whitespace character
    Whitespace,
    /// Unknown token
    Unknown,
}

// pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
//     std::iter::from_fn(move || {
//         if input.is_empty() {
//             return None;
//         }
//         let token = Cursor::new(input).advance_token();
//         input = &input[token.len..];

//         Some(token)
//     })
// }

impl Lexer {
    pub fn build(rules: Vec<Rule>) -> Self {
        Lexer { rules: rules }
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut val = Vec::new();
        let mut rem = input;        // Remaining input
        while !rem.is_empty() {
            let token = self.next_token(rem);
            let len = token.len.clone();
            val.push(token);
            rem = &rem[len..];
        }
        val
    }

    // pub fn test_token(&self, input: &str, expected: &str) {
    //     let mut len = 0;
    //     let mut actual = String::new();

    //     for t in self.tokenize(input) {
    //         let end = len + t.len;
    //         let text = &input[len..end];
    //         actual += &format!("{:?} {}\n", text, t.kind.0);
    //         len = end;
    //     }
    //     let expected = expected.trim();
    //     let actual = actual.trim();

    //     assert_eq!(
    //         expected, actual,
    //         "\nExpected:\n\n\
    //         {}\n\n\
    //         Actual:\n\n\
    //         {}\n\n",
    //         expected, actual,
    //     );
    // }

    pub fn next_token(&self, input: &str) -> Token {
        self.token_valid(input).unwrap_or_else(|| {
            self.token_invalid(input)
        })
    }

    fn token_valid(&self, input: &str) -> Option<Token> {
        let longest = self.rules.iter()
            .rev()
            .filter_map(|rule| {
                let mch = rule.re.find(input)?;
                Some((mch.end(), rule))
            })
            .max_by_key(|&(len, _)| len)?;

        let (len, rule) = longest;
        let mut kind_cl = rule.kind.clone();
        assert!(len > 0, "Bad token\nkind: {:?}\nregex: {:?}\ninput {:?}", rule.kind, rule.re, input);
        if kind_cl == TokenKind(Tokens::TakesValue) {
            let iden = &input[..len];
            kind_cl = TokenKind(Tokens::Identifier(iden.to_string()));
        }
        Some(Token {kind: kind_cl, len: len})
    }

    fn token_invalid(&self, input: &str) -> Token {
        let mut len = 0;
        for c in input.chars() {
            len += c.len_utf8();
            if self.token_valid(&input[len..]).is_some() {
                break;
            }
        }
        Token { kind: TokenKind(Unknown), len: len }
    }
}

// impl Cursor<'_> {
//     fn advance_token(&mut self) -> Token {
//         let first_char = self.bump().unwrap();
//         let token_kind = match first_char {
//             // Comments
//             // Eq
//             '=' => match self.first() {
//                 '=' => {
//                     self.bump().unwrap();
//                     Eq
//                 }
//                 _ => Assign,
//             },

//             // single char tokens
//             '+' => Plus,
//             '-' => Minus,
//             '*' => Star,
//             '%' => Percent,
//             ';' => Semi,
//             '<' => match self.first() {
//                 '=' => {
//                     self.bump().unwrap();
//                     Lte
//                 }
//                 _ => Lt,
//             },
//             '>' => match self.first() {
//                 '=' => {
//                     self.bump().unwrap();
//                     Gte
//                 }
//                 _ => Gt,
//             },

//             _ => Unknown,
//         };

//         Token::new(token_kind, self.len_consumed())
//     }
// }

fn resolve_t(mut let_type: std::string::String, ln: i32, col: i32) -> Tokens {
    if let_type
        .replace("<", "")
        .replace(">", "")
        .starts_with("Dyn")
    {
        let_type.drain(1..4);
        Tokens::Dyn(
            let_type
                .replace("(", "")
                .replace(")", "")
                .replace("<", "")
                .replace(">", ""),
        )
    } else {
        match let_type.replace("<", "").replace(">", "").as_str() {
            "String" => Tokens::String,
            "Bool" => Tokens::Bool,
            "Int" => Tokens::Int,
            "Float" => Tokens::Float,
            _ => {
                throw_error(
                    "Syntax Error:".red().bold(),
                    format!("Invalid let type {}", let_type),
                    ln,
                    col,
                );
                Tokens::Dyn(std::string::String::from(""))
            }
        }
    }
}

fn throw_error(t: ColoredString, message: std::string::String, ln: i32, col: i32) {
    println!("{} {} @ line {}, col {}", t, message, ln, col);
    std::process::exit(0)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// An easier way to construct a vector of tokens
//     /// to be used in unit tests
//     macro_rules! construct_test {
//         // construct_test![(Tokens, 1), (Tokens, 2)]
//         ( $(($typ:tt, $len:expr)),* $(,)?) => {
//             vec![$(Token { kind: $typ, len: $len },)*]
//         };

//         // construct_test![Tokens, Tokens, Tokens]
//         ($($typ:tt),* $(,)?) => {
//             construct_test![$(($typ, 1),)*]
//         };
//     }

//     const SINGLE_CHAR_TEST_STR: &'static str = "+-*/%;<>";
//     const DOUBLE_CHAR_TEST_STR: &'static str = "<=>===";

//     #[test]
//     fn create_token() {
//         let new_token = Token::new(Tokens::Plus, 1);

//         assert_eq!(new_token.kind, Tokens::Plus);
//         assert_eq!(new_token.len, 1);
//     }

//     #[test]
//     fn single_char_tokens() {
//         let tokens = tokenize(SINGLE_CHAR_TEST_STR).collect::<Vec<Token>>();
//         let expected =
//             construct_test![Plus, Minus, Star, Slash, Percent, Semi, Lt, Gt];

//         assert_eq!(tokens, expected);
//     }

//     #[test]
//     fn double_char_tokens() {
//         let tokens = tokenize(DOUBLE_CHAR_TEST_STR).collect::<Vec<Token>>();
//         let expected = construct_test![Lte, Gte, Eq];

//         assert_eq!(tokens, expected);
//     }
// }
