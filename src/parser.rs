pub use self::parsing_helper::{Languages, Parser};

mod parsing_helper {
    pub struct Parser {
        compile_target: Languages,
        file_content: String,
        parsed: bool
    }

    pub enum Languages {
        Typescript
    }

    impl Parser {
        pub fn new(target: Languages, content: String) -> Self {
            Parser {
                compile_target: target,
                file_content: content,
                parsed: false
            }
        }

        pub fn parse(mut self) {
            let parsed_file = String::new();
            let file_pre_parse = self.file_content.trim();
            for c in file_pre_parse.split_whitespace() {
                let file_vec: Vec<&str> = self.file_content.split_whitespace().collect();
                match c {
                    "let" => {
                        let index = file_vec.iter().position(|&v| v == "let").unwrap();
                        let decl_token = &file_vec[index..index+4];
                        println!("{:?}", decl_token);
                    }
                    _ => ()
                }
            };
            self.parsed = true;
        }
    }
}