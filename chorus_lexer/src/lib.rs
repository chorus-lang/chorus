//! A lot of the parser/lexer code is a modified version of the rustc parser

mod cursor;

use regex::Regex;
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
    /// And/Or
    And,
    BitwiseAnd,
    Or,
    BitwiseOr,
    /// `;`
    Semi,
    /// "" ''
    DoubleQuote,
    SingleQuote,
    /// |> (ECMA pipe operator)
    Pipe,
    /// () {} []
    OpenBracket,
    ClosedBracket,
    OpenCurly,
    ClosedCurly,
    OpenSquare,
    ClosedSquare,
    /// keywords
    Let,
    Function,
    If,
    While,
    For,
    /// Identifiers
    Identifier(std::string::String),
    TakesValue,
    Type,
    /// Whitespaces
    Newline,
    Whitespace,
    /// Unknown token
    Unknown,
}

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
