use crate::{lexer::Token, Op, Parser, TokenKind, S};
use colored::*;
use miette::{miette, Error, Result};
use std::{collections::HashMap, io::Write};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Function {
        name: String,
        params: Vec<String>,
        body: Box<S>,
    },
}

pub struct Interpreter<'a> {
    pub parser: Parser<'a>,
    pub current_token: Option<Token>,
    pub variables: HashMap<String, Value>,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Function { name, .. } => write!(f, "<function {}>", name),
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: Parser<'a>) -> Self {
        Self {
            parser,
            current_token: None,
            variables: HashMap::new(),
        }
    }
    pub fn run(&mut self) -> Result<()> {
        self.expr()
    }

    pub fn replace_db(&mut self, db: HashMap<String, Value>) {
        self.variables = db;
    }

    pub fn expr(&mut self) -> Result<()> {
        while let Some(statement) = self.parser.parse_statement() {
            match statement {
                Ok(ast) => {
                    if let Err(e) = self.evaluate(&ast) {
                        println!("Error: {}", e);
                    }
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &S) -> Result<Value> {
        match expr {
            S::Atom(token) => self.evaluate_atom(token),
            S::Cons(token, args) => self.evaluate_cons(token, args),
            S::BinaryExpr { op, lhs, rhs } => self.evaluate_binary_expr(op, lhs, rhs),
            S::IfExpr {
                cond,
                then_branch,
                else_branch,
            } => self.evaluate_if_expr(cond, then_branch, else_branch),
            S::Block(statements) => self.evaluate_block(statements),
            S::FunDef { name, args, body } => self.evaluate_function_def(name, args, body),
            S::FunCall { name, args } => self.evaluate_function_call(name, args),
        }
    }

    fn evaluate_atom(&self, token: &Token) -> Result<Value> {
        match &token.kind {
            TokenKind::Number(n) => Ok(Value::Number(n.parse().unwrap())),
            TokenKind::String(s) => Ok(Value::String(s.clone())),
            TokenKind::True => Ok(Value::Boolean(true)),
            TokenKind::False => Ok(Value::Boolean(false)),
            TokenKind::Nil => Ok(Value::Nil),
            TokenKind::Identifier(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| miette!("Undefined variable: {}", name)),
            _ => Err(miette!("Invalid atomic expression: {:?}", token)),
        }
    }

    fn evaluate_cons(&mut self, token: &Token, args: &[S]) -> Result<Value> {
        match &token.kind {
            TokenKind::Let => {
                if let [S::Atom(name_token), value_expr] = args {
                    if let TokenKind::Identifier(name) = &name_token.kind {
                        let value = self.evaluate(value_expr)?;
                        self.variables.insert(name.clone(), value.clone());
                        Ok(value)
                    } else {
                        Err(miette!("Expected identifier in let binding"))
                    }
                } else {
                    Err(miette!("Invalid let expression"))
                }
            }
            TokenKind::Return => {
                if let [value_expr] = args {
                    self.evaluate(value_expr)
                } else {
                    Err(miette!("Invalid return expression"))
                }
            }
            TokenKind::While => {
                if let [condition, body] = args {
                    while let Value::Boolean(true) = self.evaluate(condition)? {
                        self.evaluate(body)?;
                    }
                    Ok(Value::Nil)
                } else {
                    Err(miette!("Invalid while expression"))
                }
            }
            _ => self.evaluate_binary_operation(token, args),
        }
    }

    fn evaluate_binary_operation(&mut self, token: &Token, args: &[S]) -> Result<Value> {
        if args.len() != 2 {
            return Err(miette!("Binary operation requires exactly two operands"));
        }

        let left = self.evaluate(&args[0])?;
        let right = self.evaluate(&args[1])?;

        match &token.kind {
            TokenKind::Plus => self.add(left, right),
            TokenKind::Minus => self.subtract(left, right),
            TokenKind::Star => self.multiply(left, right),
            TokenKind::Slash => self.divide(left, right),
            TokenKind::EqualEqual => self.equals(left, right),
            TokenKind::BangEqual => self.not_equals(left, right),
            TokenKind::Less => self.less_than(left, right),
            TokenKind::LessEqual => self.less_equal(left, right),
            TokenKind::Greater => self.greater_than(left, right),
            TokenKind::GreaterEqual => self.greater_equal(left, right),
            _ => Err(miette!("Unknown binary operator: {:?}", token)),
        }
    }

    fn evaluate_binary_expr(&mut self, op: &Op, lhs: &S, rhs: &S) -> Result<Value> {
        let left = self.evaluate(lhs)?;
        let right = self.evaluate(rhs)?;

        match op {
            Op::Plus => self.add(left, right),
            Op::Minus => self.subtract(left, right),
            Op::Star => self.multiply(left, right),
            Op::Slash => self.divide(left, right),
        }
    }

    fn evaluate_if_expr(
        &mut self,
        cond: &S,
        then_branch: &S,
        else_branch: &Option<Box<S>>,
    ) -> Result<Value> {
        let condition = self.evaluate(cond)?;

        match condition {
            Value::Boolean(true) => self.evaluate(then_branch),
            Value::Boolean(false) => {
                if let Some(else_expr) = else_branch {
                    self.evaluate(else_expr)
                } else {
                    Ok(Value::Nil)
                }
            }
            _ => Err(miette!("If condition must be a boolean")),
        }
    }

    fn evaluate_block(&mut self, statements: &[S]) -> Result<Value> {
        let mut result = Value::Nil;
        for stmt in statements {
            result = self.evaluate(stmt)?;
        }
        Ok(result)
    }

    fn evaluate_function_def(&mut self, name: &S, args: &[S], body: &S) -> Result<Value> {
        if let S::Atom(Token {
            kind: TokenKind::Identifier(name_str),
            ..
        }) = name
        {
            let mut params = Vec::new();
            for arg in args {
                if let S::Atom(Token {
                    kind: TokenKind::Identifier(param),
                    ..
                }) = arg
                {
                    params.push(param.clone());
                } else {
                    return Err(miette!("Function parameters must be identifiers"));
                }
            }

            let func = Value::Function {
                name: name_str.clone(),
                params,
                body: Box::new(body.clone()),
            };

            self.variables.insert(name_str.clone(), func.clone());
            Ok(func)
        } else {
            Err(miette!("Function name must be an identifier"))
        }
    }

    fn evaluate_function_call(&mut self, name: &S, args: &[S]) -> Result<Value> {
        if let S::Atom(Token {
            kind: TokenKind::Identifier(name_str),
            ..
        }) = name
        {
            let func = match self.variables.get(name_str) {
                Some(Value::Function {
                    params,
                    body,
                    name: fname,
                }) => Value::Function {
                    name: fname.clone(),
                    params: params.clone(),
                    body: body.clone(),
                },
                _ => {
                    if name_str == "print" {
                        let mut output = String::new();
                        for arg in args {
                            let value = self.evaluate(arg)?;
                            output.push_str(&format!("{}", value));
                        }
                        println!("{}", output);
                        return Ok(Value::Nil);
                    }
                    
                    return Err(miette!("'{}' is not a function", name_str));
                }
            };

            // Now we can safely work with the cloned function definition
            if let Value::Function { params, body, .. } = func {
                if args.len() != params.len() {
                    return Err(miette!(
                        "Wrong number of arguments: expected {}, got {}",
                        params.len(),
                        args.len()
                    ));
                }

                // Evaluate all arguments first
                let mut evaluated_args = HashMap::new();
                for (param, arg) in params.iter().zip(args) {
                    let arg_value = self.evaluate(arg)?;
                    evaluated_args.insert(param.clone(), arg_value);
                }

                // Save old scope and set new one
                let old_scope = std::mem::replace(&mut self.variables, evaluated_args);
                let result = self.evaluate(&body);
                // Restore old scope
                self.variables = old_scope;

                result
            } else {
                unreachable!("We already checked that this is a function")
            }
        } else {
            Err(miette!("Function name must be an identifier"))
        }
    }

    // Binary operation implementations
    fn add(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(miette!("Invalid operands for addition")),
        }
    }

    fn subtract(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(miette!("Invalid operands for subtraction")),
        }
    }

    fn multiply(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(miette!("Invalid operands for multiplication")),
        }
    }

    fn divide(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(miette!("Division by zero"))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(miette!("Invalid operands for division")),
        }
    }

    fn equals(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            (Value::Nil, Value::Nil) => Ok(Value::Boolean(true)),
            _ => Ok(Value::Boolean(false)),
        }
    }

    fn not_equals(&self, left: Value, right: Value) -> Result<Value> {
        self.equals(left, right).map(|v| match v {
            Value::Boolean(b) => Value::Boolean(!b),
            _ => unreachable!(),
        })
    }

    fn less_than(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
            _ => Err(miette!("Invalid operands for less than comparison")),
        }
    }

    fn less_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            _ => Err(miette!(
                "Invalid operands for less than or equal comparison"
            )),
        }
    }

    fn greater_than(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
            _ => Err(miette!("Invalid operands for greater than comparison")),
        }
    }

    fn greater_equal(&self, left: Value, right: Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            _ => Err(miette!(
                "Invalid operands for greater than or equal comparison"
            )),
        }
    }
}
