use std::iter::Peekable;

use crate::{
    lexer::{Token, TokenKind},
    Lexer,
};
use miette::{miette, Error, LabeledSpan, Severity};

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum S {
    Atom(Token),
    Cons(Token, Vec<S>),
    BinaryExpr {
        op: Op,
        lhs: Box<S>,
        rhs: Box<S>,
    },
    IfExpr {
        cond: Box<S>,
        then_branch: Box<S>,
        else_branch: Option<Box<S>>,
    },
    Block(Vec<S>),
    FunDef {
        name: Box<S>,
        args: Vec<S>,
        body: Box<S>,
    },
    FunCall {
        name: Box<S>,
        args: Vec<S>,
    },
}

impl S {
    pub fn to_string(&self) -> String {
        match self {
            S::Atom(token) => token.kind.to_string(),
            S::Cons(token, args) => {
                let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
                format!("{:?} {:?}", token, args)
            }
            S::BinaryExpr { op, lhs, rhs } => format!("{:?} {:?} {:?}", op, lhs, rhs),
            S::IfExpr {
                cond,
                then_branch,
                else_branch,
            } => format!("if {:?} {:?} {:?}", cond, then_branch, else_branch),
            S::Block(block) => format!("{:?}", block),
            S::FunDef { name, args, body } => format!("def {:?} {:?} {:?}", name, args, body),
            S::FunCall { name, args } => format!("call {:?} {:?}", name, args),
        }
    }
}


pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    _whole_input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            _whole_input: input,
        }
    }

    pub fn parse_statement(&mut self) -> Option<Result<S, Error>> {
        if let Some(Ok(token)) = self.eat_token() {
            match token {
                Token {
                    kind: TokenKind::EOF,
                    ..
                } => return None,
                Token {
                    kind: TokenKind::NewLine,
                    ..
                } => {
                    return self.parse_statement();
                }
                Token {
                    kind: TokenKind::Let,
                    row,
                    column,
                    index,
                } => {
                    let name_match =
                        self.match_tokens(TokenKind::Identifier("variable_name".to_string()));

                    if let Err(e) = name_match {
                        return Some(Err(miette!{
                            severity = Severity::Error,
                            labels = vec![LabeledSpan::at(e.index-2..e.index, format!("Expected Identifier, got {:?}", e.kind))],
                            help = format!("use"),
                            "Expected Identifier, got {:?}", e.kind,
                        }.with_source_code(self._whole_input.to_string())));
                    }

                    let name = self.eat_token().unwrap().unwrap();
                    let equal = self.match_tokens(TokenKind::Equal);

                    if let Err(equal) = equal {
                        return Some(Err(miette!{
                            severity = Severity::Error,
                            labels = vec![LabeledSpan::at(equal.index-2..equal.index, format!("Expected Equal, got {:?}", equal.kind))],
                            help = format!("use"),
                            "Expected Equal, got {:?}", equal.kind,
                        }.with_source_code(self._whole_input.to_string())));
                    }

                    self.eat_token();
                    match self.peek_token() {
                        Some(Ok(Token {
                            kind: TokenKind::LeftBrace,
                            ..
                        })) => {
                            let block = self.parse_block().unwrap();
                            return Some(Ok(S::Cons(
                                Token {
                                    kind: TokenKind::Let,
                                    row,
                                    column,
                                    index,
                                },
                                vec![S::Atom(name), S::Block(block)],
                            )));
                        }

                        _token => {
                            let expr = self.parse_expression(0).unwrap().unwrap();
                            return Some(Ok(S::Cons(
                                Token {
                                    kind: TokenKind::Let,
                                    row,
                                    column,
                                    index,
                                },
                                vec![S::Atom(name), expr],
                            )));
                        }
                    }
                }
                Token {
                    kind: TokenKind::LeftBrace,
                    row: _,
                    column: _,
                    index: _,
                } => {
                    let block = self.parse_block().unwrap();
                    return Some(Ok(S::Block(block)));
                }

                Token {
                    kind: TokenKind::Fun,
                    row: _,
                    column: _,
                    index: _,
                } => {
                    return self.parse_function_definition();
                }

                Token {
                    kind: TokenKind::If,
                    row: _,
                    column: _,
                    index: _,
                } => {
                    return self.parse_if_expression();
                }

                Token {
                    kind: TokenKind::Return,
                    row,
                    column,
                    index,
                } => {
                    let expr = self.parse_expression(0).unwrap().unwrap();
                    return Some(Ok(S::Cons(
                        Token {
                            kind: TokenKind::Return,
                            row,
                            column,
                            index,
                        },
                        vec![expr],
                    )));
                }

                Token {
                    kind: TokenKind::Identifier(t),
                    row,
                    column,
                    index,
                } => {
                    let name = Token {
                        kind: TokenKind::Identifier(t),
                        row,
                        column,
                        index,
                    };

                    let same = self.match_tokens(TokenKind::LeftParen);

                    if same.is_ok() {
                        let args = self.parse_arguments().unwrap();
                        return Some(Ok(S::FunCall {
                            name: Box::new(S::Atom(name)),
                            args,
                        }));
                    }

                    return Some(Ok(S::Atom(name)));
                }

                Token {
                    kind: TokenKind::While,
                    row,
                    column,
                    index,
                } => {
                    return self.parse_while_expression(row, column, index);
                }

                token => {
                    return Some(Err(miette!(
                        labels = vec![LabeledSpan::at(
                            token.index - 2..token.index,
                            format!("Unexpected token: {:?}", token.kind)
                        )],
                        severity = Severity::Error,
                        help = format!("use {:?}", token.kind),
                        "Unexpected token: {:?}",
                        token.kind
                    )
                    .with_source_code(self._whole_input.to_string())));
                }
            }
        };
        None
    }

    fn parse_expression(&mut self, min_bp: u8) -> Option<Result<S, String>> {
        let mut left = match self.peek_token() {
            Some(Ok(Token {
                kind: TokenKind::EOF,
                ..
            })) => {
                return None;
            }
            Some(Ok(Token {
                kind: TokenKind::NewLine,
                ..
            })) => {
                self.eat_token();
                return self.parse_expression(0);
            }
            Some(Ok(token)) => match token {
                Token {
                    kind: TokenKind::Plus,
                    ..
                }
                | Token {
                    kind: TokenKind::Minus,
                    ..
                } => {
                    let token = self.eat_token().unwrap().unwrap();
                    let (_, r_bp) = get_prefix_binding_power(&token);
                    let right = self.parse_equality(r_bp).unwrap().unwrap();
                    S::Cons(token, vec![right])
                }

                Token {
                    kind: TokenKind::LeftParen,
                    ..
                } => {
                    let _token = self.eat_token().unwrap().unwrap();
                    let expr = self.parse_equality(0)?;
                    let ifmatch = self.match_tokens(TokenKind::RightParen);

                    if ifmatch.is_err() {
                        return Some(Err("Expected RightParen".to_string()));
                    }

                    self.eat_token();

                    expr.unwrap()
                }
                _token => {
                    let token = self.eat_token().unwrap().unwrap();
                    if let Some(Ok(Token {
                        kind: TokenKind::LeftParen,
                        ..
                    })) = self.peek_token()
                    {
                        let args = self.parse_arguments().unwrap();

                        return Some(Ok(S::FunCall {
                            name: Box::new(S::Atom(token)),
                            args,
                        }));
                    }
                    S::Atom(token)
                }
            },
            Some(Err(err)) => return Some(Err(err.to_string())),
            None => {
                return Some(Ok(S::Atom(Token {
                    kind: TokenKind::EOF,
                    row: 0,
                    column: 0,
                    index: 0,
                })))
            }
        };

        loop {
            let operator = match self.peek_token() {
                Some(Ok(Token {
                    kind: TokenKind::EOF,
                    ..
                })) => break,
                Some(Ok(token)) => token.clone(),
                Some(Err(err)) => return Some(Err(err.to_string())),
                None => break,
            };

            if let Some((l_bp, ())) = get_postfix_binding_power(&operator) {
                if l_bp < min_bp {
                    break;
                }
                self.eat_token();
                left = S::Cons(operator, vec![left]);
                continue;
            }

            if let Some((l_bp, r_bp)) = get_infix_binding_power(&operator) {
                if l_bp < min_bp {
                    break;
                }
                self.eat_token();

                let right = self.parse_expression(r_bp).unwrap().unwrap();
                left = S::Cons(operator, vec![left, right]);
                continue;
            }

            break;
        }

        Some(Ok(left))
    }

    fn parse_equality(&mut self, min_bp: u8) -> Option<Result<S, String>> {
        self.parse_expression(min_bp)
    }

    fn parse_block(&mut self) -> Result<Vec<S>, String> {
        self.eat_token();
        let mut block = Vec::new();
        match self.peek_token() {
            Some(Ok(Token {
                kind: TokenKind::RightBrace,
                ..
            })) => {
                self.eat_token();
                return Ok(block);
            }
            Some(Ok(Token {
                kind: TokenKind::EOF,
                ..
            })) => {
                return Ok(block);
            }

            _ => {
                while let Some(Ok(token)) = self.peek_token() {
                    match token {
                        Token {
                            kind: TokenKind::NewLine,
                            ..
                        } => {
                            self.eat_token();
                        }
                        Token {
                            kind: TokenKind::EOF,
                            ..
                        } => {
                            return Ok(block);
                        }

                        Token {
                            kind: TokenKind::RightBrace,
                            ..
                        } => {
                            self.eat_token();
                            return Ok(block);
                        }
                        _ => {
                            if let Some(expr) = self.parse_statement() {
                                match expr {
                                    Ok(expr) => block.push(expr),
                                    Err(err) => return Err(miette!(err).to_string()),
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(block)
    }

    fn parse_arguments(&mut self) -> Result<Vec<S>, String> {
        self.eat_token();

        let mut args = Vec::new();

        if let Some(Ok(Token {
            kind: TokenKind::RightParen,
            ..
        })) = self.peek_token()
        {
            self.eat_token();
            return Ok(args);
        }

        loop {
            let arg = self.parse_expression(0).unwrap().unwrap();
            args.push(arg);

            match self.eat_token() {
                Some(Ok(Token {
                    kind: TokenKind::Comma,
                    ..
                })) => continue,
                Some(Ok(Token {
                    kind: TokenKind::RightParen,
                    ..
                })) => {
                    break;
                }
                Some(token) => return Err(format!("Expected ',' or ')', Got: {:?}", token).into()),
                None => {
                    break;
                }
            }
        }

        Ok(args)
    }

    fn parse_if_expression(&mut self) -> Option<Result<S, Error>> {
        let cond = self.parse_expression(0).unwrap().unwrap();
        let then_branch = self.parse_block().unwrap();

        while let Some(Ok(Token {
            kind: TokenKind::NewLine,
            ..
        })) = self.peek_token()
        {
            self.eat_token();
        }

        let mut else_branch = Vec::new();
        if let Some(Ok(Token {
            kind: TokenKind::Else,
            ..
        })) = self.peek_token()
        {
            self.eat_token();
            else_branch = self.parse_block().unwrap();
        }

        return Some(Ok(S::IfExpr {
            cond: Box::new(cond),
            then_branch: Box::new(S::Block(then_branch)),
            else_branch: if else_branch.is_empty() {
                None
            } else {
                Some(Box::new(S::Block(else_branch)))
            },
        }));
    }

    fn parse_while_expression(
        &mut self,
        row: usize,
        column: usize,
        index: usize,
    ) -> Option<Result<S, Error>> {
        let cond = self.parse_expression(0).unwrap().unwrap();
        let block = self.parse_block().unwrap();
        return Some(Ok(S::Cons(
            Token {
                kind: TokenKind::While,
                row,
                column,
                index,
            },
            vec![cond, S::Block(block)],
        )));
    }

    fn parse_function_definition(&mut self) -> Option<Result<S, Error>> {
        let name_match = self.match_tokens(TokenKind::Identifier("function_name".to_string()));

        if let Err(e) = name_match {
            return Some(Err(miette!{
            severity = Severity::Error,
            labels = vec![LabeledSpan::at(e.index-2..e.index, format!("Expected Identifier, got {:?}", e.kind))],
            help = format!("use"),
            "Expected Identifier, got {:?}", e.kind,
        }.with_source_code(self._whole_input.to_string())));
        }

        let name = self.eat_token().unwrap().unwrap();

        let args = self.parse_arguments().unwrap();
        let body = self.parse_block().unwrap();
        return Some(Ok(S::FunDef {
            name: Box::new(S::Atom(name)),
            args,
            body: Box::new(S::Block(body)),
        }));
    }

    fn eat_token(&mut self) -> Option<Result<Token, Error>> {
        self.lexer.next().map(|res| res.map_err(|err| err))
    }

    fn peek_token(&mut self) -> Option<&Result<Token, Error>> {
        self.lexer.peek()
    }

    fn match_tokens(&mut self, expected: TokenKind) -> Result<Token, Token> {
        match self.peek_token() {
            Some(Ok(token))
                if (token.kind == expected || match_token_kind(&token.kind, &expected)) =>
            {
                Ok(Token {
                    kind: token.kind.clone(),
                    row: token.row,
                    column: token.column,
                    index: token.index,
                })
            }

            Some(Ok(token)) => Err(Token {
                kind: token.kind.clone(),
                row: token.row,
                column: token.column,
                index: token.index,
            }),

            Some(Err(_err)) => Err(Token {
                kind: TokenKind::EOF,
                row: 0,
                column: 0,
                index: 0,
            }),

            None => Err(Token {
                kind: TokenKind::EOF,
                row: 0,
                column: 0,
                index: 0,
            }),
        }
    }
}

fn get_prefix_binding_power(token: &Token) -> ((), u8) {
    match token {
        Token {
            kind: TokenKind::Plus,
            ..
        }
        | Token {
            kind: TokenKind::Minus,
            ..
        } => ((), 1),
        Token {
            kind: TokenKind::LeftParen,
            ..
        } => ((), 8),
        _ => panic!("Unknown operator: {:?}", token),
    }
}

fn get_infix_binding_power(token: &Token) -> Option<(u8, u8)> {
    match token {
        Token {
            kind: TokenKind::Plus,
            ..
        }
        | Token {
            kind: TokenKind::Minus,
            ..
        } => Some((1, 2)),
        Token {
            kind: TokenKind::Star,
            ..
        }
        | Token {
            kind: TokenKind::Slash,
            ..
        } => Some((3, 4)),
        Token {
            kind: TokenKind::EqualEqual,
            ..
        }
        | Token {
            kind: TokenKind::BangEqual,
            ..
        }
        | Token {
            kind: TokenKind::Less,
            ..
        }
        | Token {
            kind: TokenKind::LessEqual,
            ..
        }
        | Token {
            kind: TokenKind::Greater,
            ..
        }
        | Token {
            kind: TokenKind::GreaterEqual,
            ..
        } => Some((5, 5)),
        Token {
            kind: TokenKind::Or,
            ..
        }
        | Token {
            kind: TokenKind::And,
            ..
        } => Some((6, 7)),
        _ => None,
    }
}

fn get_postfix_binding_power(token: &Token) -> Option<(u8, ())> {
    match token {
        Token {
            kind: TokenKind::Bang,
            ..
        }
        | Token {
            kind: TokenKind::LeftBracket,
            ..
        } => Some((6, ())),
        Token {
            kind: TokenKind::LeftParen,
            ..
        } => Some((8, ())),
        _ => None,
    }
}

fn match_token_kind(token: &TokenKind, expected: &TokenKind) -> bool {
    match (token, expected) {
        (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
        _ => false,
    }
}
