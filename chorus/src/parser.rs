pub use self::parsing_helper::{Languages, Parser};

mod parsing_helper {
    pub struct Parser {
        compile_target: Languages,
        file_content: String,
        parsed: bool,
    }

    pub enum Languages {
        Typescript,
    }

    impl Parser {
        pub fn new(target: Languages, content: String) -> Self {
            Parser {
                compile_target: target,
                file_content: content,
                parsed: false,
            }
        }

        pub fn parse(mut self) {
            use chorus_lexer::{Rule, TokenKind, Tokens};
            use regex::Regex;
            let lexer = chorus_lexer::Lexer::build(
                vec!(
                    Rule {
                        kind: TokenKind(Tokens::Slash),
                        re: Regex::new(r"^/").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Comment),
                        re: Regex::new(r"^//.*").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Eq),
                        re: Regex::new(r"^={2}").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Let),
                        re: Regex::new(r"^let").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Function),
                        re: Regex::new(r"^fn").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::TakesValue),
                        re: Regex::new(r"(^\w+)").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Lt),
                        re: Regex::new(r"^<{1}").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Gt),
                        re: Regex::new(r"^>{1}").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Semi),
                        re: Regex::new(r"^;").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Assign),
                        re: Regex::new(r"^={1}").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Whitespace),
                        re: Regex::new(r"^\s").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Newline),
                        re: Regex::new(r"\b[\n|\r\n]\b").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::OpenBracket),
                        re: Regex::new(r"^\(").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::ClosedBracket),
                        re: Regex::new(r"^\)").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::OpenCurly),
                        re: Regex::new(r"^\{").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::ClosedCurly),
                        re: Regex::new(r"^\}").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::OpenSquare),
                        re: Regex::new(r"^\[").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::ClosedSquare),
                        re: Regex::new(r"^\]").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::DoubleQuote),
                        re: Regex::new("^\"").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::SingleQuote),
                        re: Regex::new(r"^'").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Pipe),
                        re: Regex::new(r"^\|>").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Function),
                        re: Regex::new(r"^fn").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::If),
                        re: Regex::new(r"^if").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::While),
                        re: Regex::new(r"^while").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::For),
                        re: Regex::new(r"^for").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Or),
                        re: Regex::new(r"^\|\|").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::BitwiseOr),
                        re: Regex::new(r"^\|").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::And),
                        re: Regex::new(r"^&&").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::BitwiseAnd),
                        re: Regex::new(r"^&").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Plus),
                        re: Regex::new(r"^\+").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Minus),
                        re: Regex::new(r"^-").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Percent),
                        re: Regex::new(r"^%").unwrap()
                    },
                    Rule {
                        kind: TokenKind(Tokens::Star),
                        re: Regex::new(r"^\*").unwrap()
                    }
                )
            );
            let tokenized = lexer.tokenize(&self.file_content);
            println!("\n{:?}", tokenized);
            // Do the funny compile thing here
            self.parsed = true;
        }
    }
}
