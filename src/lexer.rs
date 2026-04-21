use core::str;

use crate::{
    consts::{look_keyword, FUNCTION_KEYWORD, LET_KEYWORD, NULL_KEYWORD, PRINT_KEYWORD},
    token::{Keyword, Operator, Token, Value},
};

const COMMENT_PREFIX: char = '#';
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new_lexer(input_str: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input_str,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        self.input.as_bytes()[self.read_position] as char
    }
    fn skip_extras(&mut self) {
        loop {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }

            if self.ch == COMMENT_PREFIX {
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_extras();

        // println!("Current char: {:?}", self.ch);
        let token = match self.ch {
            // ' ' | '\t' => {
            //     i += 1;
            // }
            '+' => Token::Operator(Operator::Plus),
            '-' => Token::Operator(Operator::Minus),
            '/' => Token::Operator(Operator::Slash),
            '*' => Token::Operator(Operator::Star),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::EQ)
                } else {
                    Token::Operator(Operator::Equal)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::NEQ)
                } else {
                    Token::Operator(Operator::Exclamation)
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::LTE)
                } else {
                    Token::Operator(Operator::LT)
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Operator(Operator::GTE)
                } else {
                    Token::Operator(Operator::GT)
                }
            }
            '%' => Token::Operator(Operator::Modulo),
            '&' => Token::Operator(Operator::BitwiseAnd),
            '|' => Token::Operator(Operator::BitwiseOr),
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,

            ',' => Token::Comma,
            ';' => Token::Semicolon,

            '"' => Token::Operator(Operator::DoubleQuote),

            '\0' => Token::EOF,
            _ => {
                if Lexer::is_letter(self.ch) {
                    return self.read_identifier();
                } else if Lexer::is_number(self.ch) {
                    return self.read_literal();
                }

                Token::Unknown
            }
        };
        self.read_char();
        token
    }

    pub fn read_literal(&mut self) -> Token {
        let pos = self.position;
        while Lexer::is_number(self.ch) {
            self.read_char();
        }

        let token_string = &self.input[pos..self.position];
        // TODO: Handle string literals
        match token_string.parse::<f64>() {
            Ok(v) => Token::Value(Value::Double(v)),
            Err(_) => Token::Unknown,
        }
    }

    pub fn read_identifier(&mut self) -> Token {
        let pos = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }

        let token_string = &self.input[pos..self.position];
        match look_keyword(token_string) {
            Some(keyword) => Token::Keyword(keyword),
            None => Token::Identifier(token_string.to_string()),
        }
    }

    fn is_letter(ch: char) -> bool {
        // ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
        ch.is_alphabetic() || ch == '_'
    }
    fn is_number(ch: char) -> bool {
        ch.is_ascii_digit()
    }
}

pub fn get_tokens(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();

    while i < n {
        let ch = chars[i];
        match ch {
            ' ' | '\t' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::Operator(Operator::Plus));
                i += 1;
            }
            '-' => {
                tokens.push(Token::Operator(Operator::Minus));
                i += 1;
            }
            '/' => {
                tokens.push(Token::Operator(Operator::Slash));
                i += 1;
            }
            '*' => {
                tokens.push(Token::Operator(Operator::Star));
                i += 1;
            }
            '=' => {
                tokens.push(Token::Operator(Operator::Equal));
                i += 1;
            }
            '\"' => {
                tokens.push(Token::Operator(Operator::DoubleQuote));
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '{' => {
                tokens.push(Token::LBrace);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RBrace);
                i += 1;
            }

            '0'..='9' => {
                let mut dot_seen = false;
                let mut num = String::new();

                // HACK: here i did dot_seen cause to avoid 24.2.4
                while i < n && (chars[i].is_ascii_digit() || (!dot_seen && chars[i] == '.')) {
                    if chars[i] == '.' {
                        dot_seen = true;
                    }
                    num.push(chars[i]);
                    i += 1;
                }

                // NOTE: i am only converting to Double nomatter what number
                // main thing is logic not so much of specifics
                tokens.push(Token::Value(Value::Double(num.parse().unwrap())));
            }

            // TODO: support string value type with start of  ""
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut variable = String::new();

                while i < n && (chars[i].is_alphabetic() || chars[i] == '_') {
                    variable.push(chars[i]);
                    i += 1;
                }

                match variable.as_str() {
                    PRINT_KEYWORD => tokens.push(Token::Keyword(Keyword::PRINT)),
                    LET_KEYWORD => tokens.push(Token::Keyword(Keyword::LET)),
                    FUNCTION_KEYWORD => tokens.push(Token::Keyword(Keyword::FUNCTION)),
                    // TODO:
                    // "if" => tokens.push(Token::Keyword(Keyword::IF)),
                    // "else" => tokens.push(Token::Keyword(Keyword::ELSE)),
                    // "while" => tokens.push(Token::Keyword(Keyword::WHILE)),
                    // "and" => tokens.push(Token::Keyword(Keyword::AND)),
                    // "for" => tokens.push(Token::Keyword(Keyword::FOR)),
                    // "or" => tokens.push(Token::Keyword(Keyword::OR)),
                    // "return" => tokens.push(Token::Keyword(Keyword::RETURN)),
                    NULL_KEYWORD => tokens.push(Token::Value(Value::Null)),
                    _ => tokens.push(Token::Identifier(variable)),
                }
            }

            _ => {
                println!("Unknown character: {}", ch);
                i += 1;
            }
        }
    }

    tokens
}

pub fn print_tokens(tokens: &[Token]) {
    print!("[");
    tokens.iter().for_each(|t| print!("{}, ", t));
    println!("]");
}
