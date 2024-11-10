
use std::fmt;

use miette::{miette, Error, LabeledSpan, Severity};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub row: usize,
    pub column: usize,
    pub index: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        self.kind.to_string()
    }
    
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,
    EOF,
    NewLine,
    String(String),
    Number(String),
    Identifier(String),
    // Comparisons 
    Equal,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Keywords
    Fun,
    Let,
    And,
    Class,
    If,
    Else,
    False,
    For,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    While,
    Block(Vec<String>),
}
impl TokenKind {
    pub fn to_string(&self) -> String {
        match self {
            TokenKind::LeftParen => "LeftParen".to_string(),
            TokenKind::RightParen => "RightParen".to_string(),
            TokenKind::LeftBrace => "LeftBrace".to_string(),
            TokenKind::RightBrace => "RightBrace".to_string(),
            TokenKind::LeftBracket => "LeftBracket".to_string(),
            TokenKind::RightBracket => "RightBracket".to_string(),
            TokenKind::Comma => "Comma".to_string(),
            TokenKind::Dot => "Dot".to_string(),
            TokenKind::Minus => "Minus".to_string(),
            TokenKind::Plus => "Plus".to_string(),
            TokenKind::Semicolon => "Semicolon".to_string(),
            TokenKind::Star => "Star".to_string(),
            TokenKind::Slash => "Slash".to_string(),
            TokenKind::EOF => "EOF".to_string(),
            TokenKind::NewLine => "NewLine".to_string(),
            TokenKind::String(s) => format!("String({})", s),
            TokenKind::Number(n) => format!("Number({})", n),
            TokenKind::Identifier(s) => s.to_string(),
            TokenKind::Equal => "Equal".to_string(),
            TokenKind::Bang => "Bang".to_string(),
            TokenKind::BangEqual => "BangEqual".to_string(),
            TokenKind::EqualEqual => "EqualEqual".to_string(),
            TokenKind::Greater => "Greater".to_string(),
            TokenKind::GreaterEqual => "GreaterEqual".to_string(),
            TokenKind::Less => "Less".to_string(),
            TokenKind::LessEqual => "LessEqual".to_string(),
            TokenKind::Fun => "Fun".to_string(),
            TokenKind::Let => "Let".to_string(),
            TokenKind::And => "And".to_string(),
            TokenKind::Class => "Class".to_string(),
            TokenKind::If => "If".to_string(),
            TokenKind::Else => "Else".to_string(),
            TokenKind::False => "False".to_string(),
            TokenKind::For => "For".to_string(),
            TokenKind::Nil => "Nil".to_string(),
            TokenKind::Or => "Or".to_string(),
            TokenKind::Return => "Return".to_string(),
            TokenKind::Super => "Super".to_string(),
            TokenKind::This => "This".to_string(),
            TokenKind::True => "True".to_string(),
            TokenKind::While => "While".to_string(),
            TokenKind::Block(s) => format!("Block({:?})", s)
        }
    }
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
        write!(f, "Token: {:?}, row: {} col:{}", self.kind, self.row, self.column)
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
                    Ok(Token { kind: TokenKind::Slash, row: self.current_line, column: self.current_column, index: self.index,})
                    }
                }
            '(' => Ok(Token { kind: TokenKind::LeftParen, row: self.current_line, column: self.current_column, index: self.index, }),
            ')' => Ok(Token { kind: TokenKind::RightParen, row: self.current_line, column: self.current_column, index: self.index, }),
            '{' => Ok(Token { kind: TokenKind::LeftBrace, row: self.current_line, column: self.current_column, index: self.index, }),
            '}' => Ok(Token { kind: TokenKind::RightBrace, row: self.current_line, column: self.current_column, index: self.index,  }),
            '[' => Ok(Token { kind: TokenKind::LeftBracket, row: self.current_line, column: self.current_column, index: self.index,  }),
            ']' => Ok(Token { kind: TokenKind::RightBracket, row: self.current_line, column: self.current_column, index: self.index,  }),
            ',' => Ok(Token { kind: TokenKind::Comma, row: self.current_line, column: self.current_column, index: self.index, }),
            '.' => Ok(Token { kind: TokenKind::Dot, row: self.current_line, column: self.current_column, index: self.index, }),
            '-' => Ok(Token { kind: TokenKind::Minus, row: self.current_line, column: self.current_column, index: self.index, }),
            '+' => Ok(Token { kind: TokenKind::Plus, row: self.current_line, column: self.current_column, index: self.index, }),
            ';' => Ok(Token { kind: TokenKind::Semicolon, row: self.current_line, column: self.current_column, index: self.index,  }),
            '*' => Ok(Token { kind: TokenKind::Star, row: self.current_line, column: self.current_column, index: self.index, }),
            '\n' | '\r' => {
                if c == '\r' && self.rest_input.chars().next() == Some('\n') {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    }
                    self.current_line += 1;
                    self.current_column = 1;
                    Ok(Token { kind: TokenKind::NewLine, row: self.current_line, column: self.current_column, index: self.index,  })
                
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
                    Ok(Token { kind: TokenKind::EqualEqual, row: self.current_line, column: self.current_column, index: self.index,  })
                } else {
                    Ok(Token { kind: TokenKind::Equal, row: self.current_line, column: self.current_column, index: self.index, })
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
                    Ok(Token { kind: TokenKind::BangEqual, row: self.current_line, column: self.current_column, index: self.index, })
                } else {
                    Ok(Token { kind: TokenKind::Bang, row: self.current_line, column: self.current_column, index: self.index,})
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
                    Ok(Token { kind: TokenKind::LessEqual, row: self.current_line, column: self.current_column, index: self.index, })
                } else {
                    Ok(Token { kind: TokenKind::Less, row: self.current_line, column: self.current_column, index: self.index,})
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
                    Ok(Token { kind: TokenKind::GreaterEqual, row: self.current_line, column: self.current_column, index: self.index,})
                } else {
                    Ok(Token { kind: TokenKind::Greater, row: self.current_line, column: self.current_column, index: self.index, })
                }
            },

            '"' => {
                let mut string:String = String::new();
                while let Some(c) = self.rest_input.chars().next() {
                    self.rest_input = &self.rest_input[1..];
                    self.index += 1;
                    self.current_column += 1;

                    if c == '"' {
                        return Some(Ok(Token { kind: TokenKind::String(string), row: self.current_line, column: self.current_column, index: self.index, }));
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
                
              
                Ok(Token { kind: TokenKind::Number(number), row: self.current_line, column: self.current_column, index: self.index, })
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
                    "and" => return Some(Ok(Token{kind: TokenKind::And, row: self.current_line, column: self.current_column, index: self.index, })),
                    "class" => return Some(Ok(Token{kind: TokenKind::Class, row: self.current_line, column: self.current_column, index: self.index, })),
                    "else" => return Some(Ok(Token { kind: TokenKind::Else, row: self.current_line, column: self.current_column, index: self.index, })),
                    "false" => return Some(Ok(Token{kind: TokenKind::False, row: self.current_line, column: self.current_column, index: self.index, })),
                    "func" => return Some(Ok(Token{kind: TokenKind::Fun, row: self.current_line, column: self.current_column, index: self.index, })),
                    "for" => return Some(Ok(Token{kind: TokenKind::For, row: self.current_line, column: self.current_column, index: self.index, })),
                    "if" => return Some(Ok(Token{kind: TokenKind::If, row: self.current_line, column: self.current_column, index: self.index, })),
                    "nil" => return Some(Ok(Token{kind: TokenKind::Nil, row: self.current_line, column: self.current_column, index: self.index, })),
                    "or" => return Some(Ok(Token{kind: TokenKind::Or, row: self.current_line, column: self.current_column, index: self.index, })),
                    "return" => return Some(Ok(Token{kind: TokenKind::Return, row: self.current_line, column: self.current_column, index: self.index, })),
                    "super" => return Some(Ok(Token{kind: TokenKind::Super, row: self.current_line, column: self.current_column, index: self.index, })),
                    "this" => return Some(Ok(Token{kind: TokenKind::This, row: self.current_line, column: self.current_column, index: self.index, })),
                    "true" => return Some(Ok(Token{kind: TokenKind::True, row: self.current_line, column: self.current_column, index: self.index, })),
                    "let" => return Some(Ok(Token{kind: TokenKind::Let, row: self.current_line, column: self.current_column, index: self.index, })),
                    "while" => return Some(Ok(Token{kind: TokenKind::While, row: self.current_line, column: self.current_column, index: self.index, })),
                    _ => {}
                }

                Ok(Token{kind: TokenKind::Identifier(identifier), row: self.current_line, column: self.current_column, index: self.index, })
            },

            _ => Err(miette! {
                labels = vec![
                    LabeledSpan::at(self.index-2..self.index-1, "Unexpected character" )
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
