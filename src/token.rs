use core::fmt;
use std::hash::{Hash, Hasher};

use crate::parser::parser_logic::OperatorPrecedence;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Keyword {
    LET,
    PRINT,
    FUNCTION,
    IF,
    ELSE,
    FOR,
    WHILE,
    AND,
    OR,
    RETURN,
    NULL,
    TRUE,
    FALSE,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    DoubleQuote,
    Xor,
    BitwiseAnd,
    BitwiseOr,
    Modulo,
    Exclamation,
    GT,
    LT,
    EQ,  // ==
    NEQ, // !=
    GTE, // >=
    LTE, // <=
}

#[derive(Debug, Clone)]
pub enum Value {
    // Int(i32),
    Double(f64),
    StringLiteral(String),
    // Char(char),
    Null, // see if we want a Option type
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Double(l0), Value::Double(r0)) => l0.to_bits() == r0.to_bits(),
            (Value::StringLiteral(l0), Value::StringLiteral(r0)) => l0 == r0,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
impl Eq for Value {}
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            Value::Double(f) => f.to_bits().hash(state),
            Value::StringLiteral(s) => s.hash(state),
            Value::Null => 0.hash(state),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),

    Operator(Operator),
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    // LBracket, // [
    // RBracket, // ]
    Semicolon,
    Comma,
    EOF,

    Value(Value),

    Unknown,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenKind {
    Keyword(Keyword),
    Operator(Operator),

    // NOTE: to remove some hacks while developing the aumlang
    // Identifiers (no data needed here)
    // Literals (no data here; actual value stays in Token)
    Identifier,
    Value,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,

    EOF,
    Unknown,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "Keyword::{:?}", k),
            Token::Identifier(name) => write!(f, "Identifier(Name) = {}", name),

            Token::Operator(op) => write!(f, "Operator::{:?}", op),

            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, "SEMICOLON"),
            Token::Comma => write!(f, "Comma"),
            Token::EOF => write!(f, "EOF"),

            Token::Value(value) => write!(f, "{}", value),

            Token::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Token::Int(v) => write!(f, "Int({})", v),
            Value::Double(v) => write!(f, "Value::Double({})", v),
            Value::StringLiteral(s) => write!(f, "Value::String(\"{}\")", s),
            Value::Null => write!(f, "Value::Null"),
            // Value::Char(c) => write!(f, "Char('{}')", c),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Equal => "=",
            Operator::DoubleQuote => "\"",
            Operator::Xor => "^",
            // Operator::Caret => "**", // or "^" depending on your design
            Operator::BitwiseAnd => "&",
            Operator::BitwiseOr => "|",
            Operator::Modulo => "%",
            Operator::Exclamation => "!",
            Operator::GT => ">",
            Operator::LT => "<",
            Operator::GTE => ">=",
            Operator::LTE => "<=",
            Operator::EQ => "==",
            Operator::NEQ => "!=",
        };

        write!(f, "{}", op_str)
    }
}

impl Keyword {
    pub fn as_str(&self) -> &str {
        match self {
            Keyword::LET => "let",
            Keyword::PRINT => "print",
            Keyword::FUNCTION => "fn",
            Keyword::IF => "if",
            Keyword::ELSE => "else",
            Keyword::FOR => "for",
            Keyword::WHILE => "while",
            Keyword::AND => "and",
            Keyword::OR => "or",
            Keyword::RETURN => "return",
            Keyword::NULL => "null",
            Keyword::TRUE => "true",
            Keyword::FALSE => "false",
        }
    }
}
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match &self {
            Token::Keyword(keyword) => TokenKind::Keyword(keyword.clone()),
            Token::Identifier(_) => TokenKind::Identifier,
            Token::Operator(operator) => TokenKind::Operator(operator.clone()),
            Token::LParen => TokenKind::LParen,
            Token::RParen => TokenKind::RParen,
            Token::LBrace => TokenKind::LBrace,
            Token::RBrace => TokenKind::RBrace,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Comma => TokenKind::Comma,
            Token::EOF => TokenKind::EOF,
            Token::Value(_) => TokenKind::Value,
            Token::Unknown => TokenKind::Unknown,
        }
    }
}

impl Operator {
    pub fn precedence(&self) -> OperatorPrecedence {
        match self {
            Operator::EQ | Operator::NEQ => OperatorPrecedence::Equals,
            Operator::LT | Operator::GT => OperatorPrecedence::LessGreater,
            Operator::Plus | Operator::Minus => OperatorPrecedence::Sum,
            Operator::Star | Operator::Slash => OperatorPrecedence::Product,
            _ => OperatorPrecedence::Lowest,
        }
    }
}
