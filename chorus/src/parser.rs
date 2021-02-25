pub use self::parsing_helper::{Languages, Parser};

use chorus_lexer;

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
            let file_pre_parse = self.file_content.trim();
            let tokenized: Vec<chorus_lexer::Token> = chorus_lexer::tokenize(file_pre_parse).collect();
            println!("\n{:?}", tokenized);
            // Do the funny compile thing here
            self.parsed = true;
        }
    }
}
