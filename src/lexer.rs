
use std::fmt;

use miette::{miette, Error, LabeledSpan, Severity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,


    EOF,


    // multi-character tokens
    String(String),
    Number(String),
    Identifier(String),

    // Comparisons 
    Equal,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Print


    
}

pub struct Lexer<'a> {
    whole_input: &'a str,
    rest_input: &'a str,
    current_line: usize,
    current_column: usize,
    index: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::LeftParen => write!(f, "LeftParen ("),
            Token::RightParen => write!(f, "RightParen )"),
            Token::LeftBrace => write!(f, "LeftBrace {{ d"),
            Token::RightBrace => write!(f, "RightBrace }} "),
            Token::Comma => write!(f, "Comma ,"),
            Token::Dot => write!(f, "Dot ."),
            Token::Minus => write!(f, "Minus  -"),
            Token::Plus => write!(f, "Plus +"),
            Token::Semicolon => write!(f, "Semicolon ;"),
            Token::Star => write!(f, "Star * "),
            Token::Slash => write!(f, "Slash /"),
            Token::EOF => write!(f, "EOF \n"),
            Token::String(s) => write!(f, "String {s}"),
            Token::Number(s) => write!(f, "Number {s}"),
            Token::Identifier(s) => write!(f, "Identifier {s}"),
            Token::Equal => write!(f, "Equal ="),
            Token::BangEqual => write!(f, "BangEqual != "),
            Token::EqualEqual => write!(f, "EqualEqual == "),
            Token::Greater => write!(f, "Greater >"),
            Token::GreaterEqual => write!(f, "GreaterEqual >= "),
            Token::Less => write!(f, "Less <"),
            Token::LessEqual => write!(f, "LessEqual <= "),
            Token::And => write!(f, "And and "),
            Token::Class => write!(f, "Class class "),
            Token::Else => write!(f, "Else else "),
            Token::False => write!(f, "False false "),
            Token::Fun => write!(f, "Fun fun "),
            Token::For => write!(f, "For for "),
            Token::If => write!(f, "If if "),
            Token::Nil => write!(f, "Nil nil "),
            Token::Or => write!(f, "Or or "),
            Token::Return => write!(f, "Return return "),
            Token::Super => write!(f, "Super super "),
            Token::This => write!(f, "This this "),
            Token::True => write!(f, "True true "),
            Token::Var => write!(f, "Var var "),
            Token::While => write!(f, "While while "),
            Token::Print => write!(f, "Print print "),

        };
        s
    }   

}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            whole_input: input,
            rest_input: input,
            current_line: 1,
            current_column: 1,
            index: 1,
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;
    

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.rest_input.chars().next()?;
        self.rest_input = &self.rest_input[1..];
        
        self.index += 1;

        if c== ' '{
            self.current_column += 1;
            return self.next();
        }

        if self.current_column == 1 && c == '/' && self.rest_input.chars().next() == Some('/') {
            while let Some(c) = self.rest_input.chars().next() {
                self.rest_input = &self.rest_input[1..];
                self.index += 1;
                if c == '\n' {
                    break;
                }
            }
            self.current_line += 1;
            self.current_column = 1;
            return self.next();
        }
        
       

        let token = match c {
            ' ' =>{
                self.current_column += 1;
                return self.next();
                        },
                        '/' =>{
                            if self.rest_input.starts_with("/") {
                                while let Some(c) = self.rest_input.chars().next() {
                                    self.rest_input = &self.rest_input[1..];
                                    self.index += 1;
                                    if c == '\n' {
                                        break;
                                    }
                                }
                                self.current_line += 1;
                                self.current_column = 1;
                                return self.next();
                            } else {
                                Ok(Token::Slash)
                            }
                        }
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            '-' => Ok(Token::Minus),
            '+' => Ok(Token::Plus),
            ';' => Ok(Token::Semicolon),
            '*' => Ok(Token::Star),
            '\n' | '\r' => {
                if c == '\r' && self.rest_input.chars().next() == Some('\n') {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    
                    }
                    self.current_line += 1;
                    self.current_column = 1;
                    Ok(Token::EOF)
                
            },
            '=' => {
                while c == ' ' {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    self.next();
                }

                if self.rest_input.starts_with("=") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    Ok(Token::EqualEqual)
                } else {
                    Ok(Token::Equal)
                }
            },
            '!' => {
                
                while self.rest_input.starts_with(" ") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                }


                if self.rest_input.starts_with("=") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    Ok(Token::BangEqual)
                } else {
                    Err(miette! {
                        labels = vec![LabeledSpan::at(self.index-2..self.index-10, "Expected '=' after '!'")],
                        severity = Severity::Error,
                        help = "Please use '!=' for not equal",
                        "Unexpected character: at line: {:?} column: {:?} character: {:?}", self.current_line, self.current_column, c
                    }
                    .with_source_code(self.whole_input.to_string()))
                }
            },
            '<' => {
                while  self.rest_input.starts_with(" ") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                }

                if self.rest_input.starts_with("=") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    Ok(Token::LessEqual)
                } else {
                    Ok(Token::Less)
                }
            },
            '>' => {
                while self.rest_input.starts_with(" ") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                }


                if self.rest_input.starts_with("=") {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    Ok(Token::GreaterEqual)
                } else {
                    Ok(Token::Greater)
                }
            },

            '"' => {
                let mut string:String = String::new();
                while let Some(c) = self.rest_input.chars().next() {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    self.current_column += 1;

                    if c == '"' {
                        return Some(Ok(Token::String(string)))
                    }
                    if c == '\n' {
                        self.current_line += 1;
                        self.current_column = 1;
                    }
                    string.push(c);
            }
                Err(miette! {
                    labels = vec![LabeledSpan::at(self.index-2..self.index, "Unterminated string")],
                    severity = Severity::Error,
                    help = "this string is not terminated",
                    "Unterminated string: at line: {} column: {} character: {}", self.current_line, self.current_column, c
                }.with_source_code(self.whole_input.to_string())
            )
            },

            '0'..='9' => {
                let mut number = String::from(c);

                while let Some(c) = self.rest_input.chars().next() {

                    if c.is_numeric() { 
                        self.rest_input = &self.rest_input[1..];
                        self.index += 1;
                        self.current_column += 1;
                        number.push(c);
                    }else if !number.contains(".") && c == '.' {
                        self.rest_input = &self.rest_input[1..];
                        self.index += 1;
                        self.current_column += 1;
                        number.push(c);
                    
                    } else {
                        break;
                    }
                  
                }

                if number.ends_with('.') {
                    self.rest_input= &self.whole_input[self.index-2..];
                }
                
              
                Ok(Token::Number(number))
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::from(c);
                while let Some(c) = self.rest_input.chars().next() {
                    if c.is_alphanumeric() || c == '_' {
                        self.rest_input = &self.rest_input[1..];
                        self.index += 1;
                        self.current_column += 1;
                        identifier.push(c);
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "and" => return Some(Ok(Token::And)),
                    "class" => return Some(Ok(Token::Class)),
                    "else" => return Some(Ok(Token::Else)),
                    "false" => return Some(Ok(Token::False)),
                    "fun" => return Some(Ok(Token::Fun)),
                    "for" => return Some(Ok(Token::For)),
                    "if" => return Some(Ok(Token::If)),
                    "nil" => return Some(Ok(Token::Nil)),
                    "or" => return Some(Ok(Token::Or)),
                    "return" => return Some(Ok(Token::Return)),
                    "super" => return Some(Ok(Token::Super)),
                    "this" => return Some(Ok(Token::This)),
                    "true" => return Some(Ok(Token::True)),
                    "var" => return Some(Ok(Token::Var)),
                    "while" => return Some(Ok(Token::While)),
                    "print" => return Some(Ok(Token::Print)),
                    _ => {}
                }

                Ok(Token::Identifier(identifier))
            },

            _ => Err(miette! {
                labels = vec![
                    LabeledSpan::at(self.index-1..self.index, "Unexpected character" )
                    ],
                    severity  = Severity::Error,
                    help = "Please use valid characters",
                    "Unexpected character: at line: {} column: {} character: {}", self.current_line, self.current_column, c
                }
            .with_source_code(self.whole_input.to_string())),
        };
        self.current_column += 1;
        Some(token)
    }
}
