#![allow(dead_code)]
use crate::lexer::Token;
use std::fmt;
use std::iter::Iterator;

type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
    Number(String),
    ProcudureCall(Box<Expression>, Vec<Box<Expression>>),
}

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    error: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax error: {}", self.error)
    }
}

macro_rules! syntax_error {
    ($($arg:tt)*) => (
        return Err(SyntaxError { error: format!($($arg)*) });
    )
}

pub struct Parser<TokenIter: Iterator<Item = Token> + Clone> {
    current: Option<Token>,
    lexer: TokenIter,
}

impl<TokenIter: Iterator<Item = Token> + Clone> Parser<TokenIter> {
    pub fn new(mut lexer: TokenIter) -> Parser<TokenIter> {
        Self {
            current: lexer.next(),
            lexer: lexer,
        }
    }

    pub fn parse(&mut self) -> Result<Option<Box<Expression>>> {
        match self.current.clone() {
            Some(token) => match token {
                Token::Number(a) => self.generate(Box::new(Expression::Number(a))),
                Token::Identifier(a) => self.generate(Box::new(Expression::Identifier(a))),
                Token::LeftParen => self.procedure_call(),
                Token::RightParen => syntax_error!("Unmatched Parentheses!"),
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn procedure_call(&mut self) -> Result<Option<Box<Expression>>> {
        self.advance();
        match self.parse()? {
            None => Ok(None),
            Some(operator) => {
                let mut params: Vec<Box<Expression>> = vec![];
                loop {
                    match &self.current {
                        Some(Token::RightParen) => {
                            return self.generate(Box::new(Expression::ProcudureCall(operator, params)));
                        }
                        None => syntax_error!("Unmatched Parentheses!"),
                        _ => params.push(
                            match self.parse()? {
                                None => syntax_error!("Unmatched Parentheses!"),
                                Some(subexpr) => subexpr
                            }),
                    }
                }
            }
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn generate(&mut self, ast: Box<Expression>) -> Result<Option<Box<Expression>>> {
        self.advance();
        Ok(Some(ast))
    }
}

#[test]
fn empty() -> Result<()> {
    let tokens = Vec::new();
    let mut parser = Parser::new(tokens.into_iter());
    let ast = parser.parse()?;
    assert_eq!(ast, None);
    Ok(())
}

#[test]
fn number() -> Result<()>{
    let tokens = vec![Token::Number("1".to_string())];
    let mut parser = Parser::new(tokens.into_iter());
    let ast = parser.parse()?;
    assert_eq!(ast, Some(Box::new(Expression::Number("1".to_string()))));
    Ok(())
}

#[test]
fn identifier() -> Result<()>{
    let tokens = vec![Token::Identifier("test".to_string())];
    let mut parser = Parser::new(tokens.into_iter());
    let ast = parser.parse()?;
    assert_eq!(ast, Some(Box::new(Expression::Identifier("test".to_string()))));
    Ok(())
}

#[test]
fn procedure_call() -> Result<()> {
    let tokens = vec![
        Token::LeftParen,
        Token::Identifier("+".to_string()),
        Token::Number("1".to_string()),
        Token::Number("2".to_string()),
        Token::Number("3".to_string()),
        Token::RightParen,
    ];
    let mut parser = Parser::new(tokens.into_iter());
    let ast = parser.parse()?;
    assert_eq!(
        ast,
        Some(Box::new( Expression::ProcudureCall(
            Box::new(Expression::Identifier("+".to_string())),
            vec![
                Box::new(Expression::Number("1".to_string())),
                Box::new(Expression::Number("2".to_string())),
                Box::new(Expression::Number("3".to_string())),
            ]
        )))
    );
    Ok(())
}

#[test]
fn unmatched_parantheses() {
    let tokens = vec![
        Token::LeftParen,
        Token::Identifier("+".to_string()),
        Token::Number("1".to_string()),
        Token::Number("2".to_string()),
        Token::Number("3".to_string()),
    ];
    let mut parser = Parser::new(tokens.into_iter());
    assert_eq!(
        parser.parse(),
        Err(SyntaxError {
            error: "Unmatched Parentheses!".to_string()
        })
    );
}

#[test]
fn nested_procedure_call() -> Result<()>{
    let tokens = vec![
        Token::LeftParen,
        Token::Identifier("+".to_string()),
        Token::Number("1".to_string()),
        Token::LeftParen,
        Token::Identifier("-".to_string()),
        Token::Number("2".to_string()),
        Token::Number("3".to_string()),
        Token::RightParen,
        Token::RightParen,
    ];
    let mut parser = Parser::new(tokens.into_iter());
    let ast = parser.parse()?;
    assert_eq!(
        ast,
        Some(Box::new(Expression::ProcudureCall(
            Box::new(Expression::Identifier("+".to_string())),
            vec![
                Box::new(Expression::Number("1".to_string())),
                Box::new(Expression::ProcudureCall(
                    Box::new(Expression::Identifier("-".to_string())),
                    vec![
                        Box::new(Expression::Number("2".to_string())),
                        Box::new(Expression::Number("3".to_string()))
                    ]
                )),
            ]
        )))
    );
    Ok(())
}
