use crate::{
    lexer::Lexer,
    parser::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement},
    token::{Keyword, Operator, Token},
    utils::print_error,
};

pub struct Parser<'a> {
    l: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::Unknown,
            peek_token: Token::Unknown,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();

        p
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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.curr_token != Token::EOF {
            println!("{}", self.curr_token);
            let statement: Option<Statement> = match &self.curr_token {
                Token::Keyword(kw) => match kw {
                    Keyword::LET => self.parse_let_statement(),
                    Keyword::RETURN => self.parse_return_statement(),
                    Keyword::IF => self.parse_if_statement(),
                    _ => None,
                },
                _ => None,
            };

            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }

            self.next_token();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        // Eg: let <identifier> = <expression>;
        if !self.expect_peek_ident() {
            return None;
        }

        let identifier = match Identifier::new(self.curr_token.clone()) {
            Ok(id) => id,
            Err(_) => return None,
        };

        if !self.expect_peek(Token::Operator(Operator::Equal)) {
            return None;
        }
        self.next_token(); // skip equal

        // TODO: Skipping for now
        let expression = self.parse_expression();
        while self.curr_token != Token::Semicolon && self.curr_token != Token::EOF {
            self.next_token();
        }

        Some(Statement::Let(LetStatement::new(
            Keyword::LET,
            identifier,
            expression,
        )))
    }
    fn parse_return_statement(&mut self) -> Option<Statement> {
        // Eg: return <expression>;
        self.next_token(); // skip return

        // TODO: parse expredssion
        let expression = self.parse_expression();
        while self.curr_token != Token::Semicolon && self.curr_token == Token::EOF {
            self.next_token();
        }

        Some(Statement::Return(ReturnStatement::new(
            Keyword::RETURN,
            expression,
        )))
    }
    fn parse_if_statement(&self) -> Option<Statement> {
        None
    }

    fn parse_expression(&self) -> Expression {
        Expression::Identifier(Identifier {
            value: String::from("Not impelmented"),
        })
    }
}
