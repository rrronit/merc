use crate::{lexer::Token, Parser};


pub struct Interpreter<'a> {
    pub parser: Parser<'a>,
    pub current_token: Option<Token>,
}

impl<'a> Interpreter<'a> {
    
    pub fn new(parser: Parser) -> Self {
        todo!()
    }

    
}