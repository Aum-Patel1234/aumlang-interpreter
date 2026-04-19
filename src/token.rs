use core::fmt;

#[derive(Debug)]
pub enum Keyword {
    Print,
    IF,
    ELSE,
    FOR,
    WHILE,
    AND,
    OR,
    RETURN,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    DoubleQuote,
    // Xor,
    // Caret, // for power make an inbuilt function
    // BitwiseAnd,
    // BitwiseOr,
    // Modulo,
    // Exclamation,
    // Equality, // ==
    // NotEqual, // !=
    // GreaterThanEqualTo // >=
    // LessThanEqualTo // <=
}

#[derive(Debug)]
pub enum Value {
    // Int(i32),
    Double(f64),
    StringLiteral(String),
    // Char(char),
    NULL, // see if we want a Option type
}

#[derive(Debug)]
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
    Value(Value),
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

            Token::Value(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Token::Int(v) => write!(f, "Int({})", v),
            Value::Double(v) => write!(f, "Value::Double({})", v),
            Value::StringLiteral(s) => write!(f, "Value::String(\"{}\")", s),
            Value::NULL => write!(f, "NULL"),
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
            // Operator::Xor => "^",
            // Operator::Caret => "**", // or "^" depending on your design
            // Operator::BitwiseAnd => "&",
            // Operator::BitwiseOr => "|",
            // Operator::Modulo => "%",
            // Operator::Exclamation => "!",
            // Operator::Equality => "==",
            // Operator::NotEqual => "!=",
            // Operator::GreaterThanEqualTo => ">=",
            // Operator::LessThanEqualTo => "<=",
        };

        write!(f, "{}", op_str)
    }
}
