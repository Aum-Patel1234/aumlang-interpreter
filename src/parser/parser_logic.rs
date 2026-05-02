use std::collections::HashMap;

use crate::{
    lexer::Lexer,
    parser::{
        ast::{
            Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program,
            ReturnStatement, Statement,
        },
        pratt_parser::{InfixParseFn, PrefixParseFn},
    },
    token::{Keyword, Operator, Token, TokenKind},
    utils::print_error,
};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum OperatorPrecedence {
    Lowest = 0,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Parser<'a> {
    l: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,

    errors: Vec<String>,

    // NOTE: Storing the memory of the function in map
    prefix_parse_fns: HashMap<TokenKind, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenKind, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::Unknown,
            peek_token: Token::Unknown,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.next_token();
        p.next_token();

        p.register_prefix(TokenKind::Identifier, Parser::parse_identifier);
        p.register_prefix(TokenKind::Value, Parser::parse_integer_literal);

        p
    }
    pub fn register_prefix(&mut self, token: TokenKind, prefix_parse_fn: PrefixParseFn<'a>) {
        self.prefix_parse_fns.insert(token, prefix_parse_fn);
    }
    pub fn register_infix(&mut self, token: TokenKind, infix_parse_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token, infix_parse_fn);
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }
    pub fn peek_error(&mut self, tok: Token) {
        let msg = format!(
            "Expected next token {}, but got {} instead",
            tok, self.peek_token
        );
        self.errors.push(msg);
    }
    pub fn check_parse_errors(&self) -> bool {
        let errors = self.get_errors();

        if errors.is_empty() {
            return true;
        }

        print_error(&format!("\nParser has {} error(s):", errors.len()));
        for (i, e) in errors.iter().enumerate() {
            print_error(&format!("  {}. {}", i + 1, e));
        }
        false
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
    pub fn expect_peek(&mut self, tok: Token) -> bool {
        if self.peek_token == tok {
            self.next_token();
            return true;
        }
        self.peek_error(tok);
        false
    }
    pub fn expect_peek_ident(&mut self) -> bool {
        if matches!(self.peek_token, Token::Identifier(_)) {
            self.next_token();
            true
        } else {
            self.errors.push(format!(
                "Expected identifier, got {} instead",
                self.peek_token
            ));
            false
        }
    }
    fn parse_identifier(&mut self) -> Option<Expression> {
        let identifier = Identifier::new(self.curr_token.clone());
        match identifier {
            Ok(ident) => Some(Expression::Identifier(ident)),
            Err(e) => {
                self.errors.push(e);
                None
            }
        }
    }
    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let integer_literal = IntegerLiteral::new(self.curr_token.clone());
        match integer_literal {
            Ok(il) => Some(Expression::IntegerLiteral(il)),
            Err(e) => {
                self.errors.push(e);
                None
            }
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.curr_token != Token::EOF {
            // println!("{}", self.curr_token);
            let statement: Option<Statement> = match &self.curr_token {
                Token::Keyword(kw) => match kw {
                    Keyword::LET => self.parse_let_statement(),
                    Keyword::RETURN => self.parse_return_statement(),
                    Keyword::IF => self.parse_if_statement(),
                    _ => None,
                },
                _ => self.parse_expression_statement(),
            };

            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }

            self.next_token();
        }

        program
    }

    fn skip_to_semicolon(&mut self) {
        while self.curr_token != Token::Semicolon && self.curr_token != Token::EOF {
            self.next_token();
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        // Eg: let <identifier> = <expression>;
        if !self.expect_peek_ident() {
            self.skip_to_semicolon();
            return None;
        }

        let identifier = match Identifier::new(self.curr_token.clone()) {
            Ok(id) => id,
            Err(_) => {
                self.skip_to_semicolon();
                return None;
            }
        };

        if !self.expect_peek(Token::Operator(Operator::Equal)) {
            self.skip_to_semicolon();
            return None;
        }
        self.next_token();

        // TODO: Skipping for now
        let expression = self.parse_expression(OperatorPrecedence::Lowest);

        self.skip_to_semicolon();

        expression.map(|expr| Statement::Let(LetStatement::new(Keyword::LET, identifier, expr)))
    }
    fn parse_return_statement(&mut self) -> Option<Statement> {
        // Eg: return <expression>;
        self.next_token(); // skip return

        // TODO: parse expredssion
        let expression = self.parse_expression(OperatorPrecedence::Lowest);
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        expression.map(|expr| Statement::Return(ReturnStatement::new(Keyword::RETURN, expr)))
    }
    fn parse_if_statement(&self) -> Option<Statement> {
        None
    }
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.curr_token.clone();
        let expression = self.parse_expression(OperatorPrecedence::Lowest);
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        match expression {
            Some(expr) => {
                let stmt = ExpressionStatement::new(token, expr);
                Some(Statement::Expression(stmt))
            }
            None => None,
        }
    }

    fn parse_expression(&mut self, _precedence: OperatorPrecedence) -> Option<Expression> {
        // TODO:
        // let key = match &self.curr_token {
        //     Token::Identifier(_) => Token::Identifier(String::new()),
        //     _ => self.curr_token.clone(),
        // };
        let key = self.curr_token.kind();

        let prefix = self.prefix_parse_fns.get(&key)?;

        let expr = prefix(self)?;
        // println!("\nExpression : {:?}, prececdence: {:?}", expr, precedence);
        // self.next_token();
        Some(expr)
    }
}
