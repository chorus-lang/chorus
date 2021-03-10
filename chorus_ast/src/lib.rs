use chorus_lexer::*;

#[derive(Clone, Debug)]
pub struct Ast {
    language: Languages,
    done: (bool, bool),
    errored: bool,
    tokens:Vec<Token>,
    parsed: Vec<Node>
}

#[derive(Clone, Debug)]
pub struct Node {
    value: Option<String>,
    t: Option<String>,
    kind: TokenKind
}

impl Ast {
    pub fn build(language: Languages, tokens: Vec<Token>) -> Ast {
        Ast {
            language: language,
            done: (false, false),
            errored: false,
            tokens: tokens,
            parsed: Vec::new()
        }
    }

    pub fn parse(&mut self) -> &mut Self {    // parse the tokens to nodes and meaningful grammar
        for token in self.tokens.clone() {
            let tokens = &self.tokens;
            match token.kind {
                TokenKind(Tokens::Let) => {
                    let let_slice = &tokens[1..11];
                    println!("{:?}", let_slice);
                    self.tokens = tokens[let_slice.len()..].to_vec();
                },
                TokenKind(Tokens::Whitespace) => {
                    println!("Whitespace")
                }
                _ => {}
            }
        }
        self.done.0 = true;
        self
    }

    pub fn compile(self) -> String {
        match self.language {
            Languages::Typescript => {
                return "you are stupid stop trying".to_string();
            },
            // _ => return "no language".to_string()
        }
    }
}


