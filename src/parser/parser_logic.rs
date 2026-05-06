use std::collections::HashMap;

use crate::{
    lexer::Lexer,
    parser::{
        ast::{
            BlockStatement, Boolean, Expression, ExpressionStatement, FunctionLiteral, Identifier,
            IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program,
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
    infix_parse_fns: HashMap<TokenKind, InfixParseFn<'a>>,
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

        // prefix_parse_fns
        p.register_prefix(TokenKind::Identifier, Parser::parse_identifier);
        p.register_prefix(TokenKind::Value, Parser::parse_integer_literal);
        p.register_prefix(
            TokenKind::Operator(Operator::Exclamation),
            Parser::parse_prefix_expression,
        );
        p.register_prefix(
            TokenKind::Operator(Operator::Minus),
            Parser::parse_prefix_expression,
        );
        p.register_prefix(TokenKind::Keyword(Keyword::TRUE), Parser::parse_boolean);
        p.register_prefix(TokenKind::Keyword(Keyword::FALSE), Parser::parse_boolean);
        p.register_prefix(TokenKind::LParen, Parser::parse_grouped_expression);
        p.register_prefix(TokenKind::Keyword(Keyword::IF), Parser::parse_if_expression);
        p.register_prefix(
            TokenKind::Keyword(Keyword::FUNCTION),
            Parser::parse_function_literal,
        );

        // infix_parse_fns
        p.register_infix(
            TokenKind::Operator(Operator::Plus),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::Minus),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::GT),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::LT),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::Slash),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::Star),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::EQ),
            Parser::parse_infix_expression,
        );
        p.register_infix(
            TokenKind::Operator(Operator::NEQ),
            Parser::parse_infix_expression,
        );

        p
    }
    pub fn register_prefix(&mut self, token: TokenKind, prefix_parse_fn: PrefixParseFn<'a>) {
        self.prefix_parse_fns.insert(token, prefix_parse_fn);
    }
    pub fn register_infix(&mut self, token: TokenKind, infix_parse_fn: InfixParseFn<'a>) {
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
    pub fn no_prefix_parse_error(&mut self, kind: TokenKind) {
        self.errors
            .push(format!("No prefix parse fn found for token - {:?}.", kind));
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
    pub fn peek_precedence(&self) -> OperatorPrecedence {
        match &self.peek_token {
            Token::Operator(op) => op.precedence(),
            _ => OperatorPrecedence::Lowest,
        }
    }
    pub fn curr_precedence(&self) -> OperatorPrecedence {
        match &self.curr_token {
            Token::Operator(op) => op.precedence(),
            _ => OperatorPrecedence::Lowest,
        }
    }

    // parse funcs
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
    fn parse_boolean(&mut self) -> Option<Expression> {
        match Boolean::new(self.curr_token.clone()) {
            Ok(b) => Some(Expression::Boolean(b)),
            Err(e) => {
                self.errors.push(e);
                None
            }
        }
    }
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token(); // skip (

        let expr = self.parse_expression(OperatorPrecedence::Lowest);
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        expr
    }
    fn parse_if_expression(&mut self) -> Option<Expression> {
        // if <condition> {consequence} else {alternative}
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        self.next_token(); // skip if token

        let condition = self.parse_expression(OperatorPrecedence::Lowest)?;
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement()?;
        if self.curr_token == Token::EOF || self.peek_token != Token::Keyword(Keyword::ELSE) {
            return Some(Expression::IfExpression(IfExpression::new(
                condition,
                consequence,
                None,
            )));
        }

        let alternative = if self.peek_token == Token::Keyword(Keyword::ELSE) {
            self.next_token(); // skip }

            // skip else
            if !self.expect_peek(Token::LBrace) {
                return None;
            }

            self.parse_block_statement()
        } else {
            None
        };

        Some(Expression::IfExpression(IfExpression::new(
            condition,
            consequence,
            alternative,
        )))
    }
    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        self.next_token(); // skip {
        // println!("Block - {}", self.curr_token);

        let mut statements: Vec<Statement> = Vec::new();
        while self.curr_token != Token::EOF && self.curr_token != Token::RBrace {
            let statement = self.parse_statement();

            if let Some(stmt) = statement {
                statements.push(stmt);
            }

            self.next_token();
        }
        // if self.curr_token != Token::EOF && self.curr_token == Token::RBrace {
        //     self.next_token(); // skip {
        // }

        Some(BlockStatement { statements })
    }
    fn parse_function_args(&mut self) -> Option<Vec<Identifier>> {
        let mut args: Vec<Identifier> = Vec::new();
        while self.curr_token != Token::EOF && self.curr_token != Token::RParen {
            let ident = match Identifier::new(self.curr_token.clone()) {
                Ok(i) => Some(i),
                Err(e) => {
                    self.errors.push(e);
                    None
                }
            };
            if let Some(arg) = ident {
                args.push(arg);
            }

            if self.peek_token == Token::Comma {
                self.next_token(); // move to comma
                self.next_token(); // move to next identifier
            } else if self.peek_token == Token::RParen {
                self.next_token(); // move to RParen
                break;
            } else {
                self.errors.push(format!(
                    "Expected Comma or RParen, found {} in fn args",
                    self.peek_token
                ));
                return None;
            }
        }

        Some(args)
    }
    fn parse_function_literal(&mut self) -> Option<Expression> {
        // Eg: fn <parameters> <block statement>
        if !self.expect_peek(Token::LParen) {
            // skip fn
            return None;
        }
        self.next_token(); // skip LParen

        let args = self.parse_function_args()?;
        if !self.expect_peek(Token::LBrace) {
            return None;
        }
        let body = self.parse_block_statement()?;

        Some(Expression::FunctionLiteral(FunctionLiteral::new(
            args, body,
        )))
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let op = match &self.curr_token {
            Token::Operator(op) => op.clone(),
            tok => {
                self.errors.push(format!(
                    "Expected Operator for prefix_expression, but found {}",
                    tok
                ));
                return None;
            }
        };

        self.next_token(); // move to right expression

        let expr = self.parse_expression(OperatorPrecedence::Prefix)?;
        Some(Expression::PrefixExpression(PrefixExpression::new(
            op, expr,
        )))
    }
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let op = if let Token::Operator(op) = &self.curr_token {
            op.clone()
        } else {
            self.errors
                .push(format!("Expected operator, got {}", self.curr_token));
            return None;
        };

        let precedence = op.precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        // NOTE:  for left associative currenty the above is right associative
        // right associative : "(a + (b + c))"
        // left associative  : "((a + b) + c)"
        // if matches!(op, Operator::Plus) {
        //     right = self.parse_expression(precedence-1)?;
        // }
        //
        // NOTE: or another way is to make an (left_precedence, right_precedence)
        // Can be usefull for -- or ++ operators

        Some(Expression::InfixExpression(InfixExpression::new(
            left, op, right,
        )))
    }

    // parse program
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.curr_token != Token::EOF {
            // println!("{}", self.curr_token);
            let statement = self.parse_statement();

            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match &self.curr_token {
            Token::Keyword(kw) => match kw {
                Keyword::LET => self.parse_let_statement(),
                Keyword::RETURN => self.parse_return_statement(),
                // Keyword::IF => self.parse_if_statement(),
                _ => self.parse_expression_statement(),
            },
            _ => self.parse_expression_statement(),
        }
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

        let expression = self.parse_expression(OperatorPrecedence::Lowest);
        self.skip_to_semicolon();

        expression.map(|expr| Statement::Let(LetStatement::new(Keyword::LET, identifier, expr)))
    }
    fn parse_return_statement(&mut self) -> Option<Statement> {
        // Eg: return <expression>;
        self.next_token(); // skip return

        let expression = self.parse_expression(OperatorPrecedence::Lowest);
        if self.peek_token != Token::Semicolon {
            self.errors.push(format!(
                "Expected ';' after return expression, got {}",
                self.peek_token
            ));
            self.skip_to_semicolon();
            return None;
        }

        self.next_token(); // consume ;

        expression.map(|expr| Statement::Return(ReturnStatement::new(Keyword::RETURN, expr)))
    }
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.curr_token.clone();
        let expression = self.parse_expression(OperatorPrecedence::Lowest);
        if self.peek_token == Token::Semicolon
            || self.peek_token == Token::EOF
            || self.peek_token == Token::RBrace
        {
            self.next_token();
        } else {
            self.errors.push(format!(
                "Expected ';' after expression, got {}",
                self.peek_token
            ));
        }

        match expression {
            Some(expr) => {
                let stmt = ExpressionStatement::new(token, expr);
                Some(Statement::Expression(stmt))
            }
            None => None,
        }
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> Option<Expression> {
        let key = self.curr_token.kind();

        let prefix = match self.prefix_parse_fns.get(&key) {
            Some(p) => *p,
            None => {
                self.no_prefix_parse_error(key);
                return None;
            }
        };

        let mut expr = prefix(self)?;
        // println!("{:?}", expr);

        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            let infix = match self.infix_parse_fns.get(&self.peek_token.kind()) {
                Some(i) => *i,
                None => {
                    return Some(expr);
                }
            };

            self.next_token();

            expr = infix(self, expr)?;
        }

        Some(expr)
    }
}
