use crate::Lexer;


pub struct Parser<'a> {
    whole_input: String,
    lexer: Lexer<'a>,    
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            whole_input: String::new(),
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&self, input: &str) -> Result<(), String> {
        todo!("Implement parser")
    }
}
