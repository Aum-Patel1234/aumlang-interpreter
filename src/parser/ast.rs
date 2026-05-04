use std::fmt::Display;

use crate::token::{Keyword, Operator, Token, Value};

// Traits
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}
impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token_literal(),
            Statement::Return(s) => s.token_literal(),
            Statement::Expression(expression_statement) => expression_statement.token_literal(),
        }
    }

    fn string(&self) -> String {
        match &self {
            Statement::Let(let_statement) => let_statement.string(),
            Statement::Return(return_statement) => return_statement.string(),
            Statement::Expression(expression_statement) => expression_statement.string(),
        }
    }
}

// NOTE: i have ignored expressionNode() in the book
#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.token_literal(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.token_literal(),
            Expression::InfixExpression(infix_expression) => infix_expression.token_literal(),
        }
    }

    fn string(&self) -> String {
        match &self {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
        }
    }
}

// Identifier
#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

// impls for Identifier
impl Identifier {
    pub fn new(token: Token) -> Result<Identifier, String> {
        match token {
            Token::Identifier(val) => Ok(Identifier { value: val }),
            _ => Err(format!(
                "Error while Identifier::new() --> Expected Token::Identifier, got {:?}",
                token
            )),
        }
    }
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.value.to_string()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

// IntegerLiteral
#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub val: f64,
}
impl IntegerLiteral {
    pub fn new(token: Token) -> Result<IntegerLiteral, String> {
        if let Token::Value(Value::Double(v)) = token {
            Ok(IntegerLiteral { token, val: v })
        } else {
            Err(format!("Expected numeric literal, got {:?}", token))
        }
    }
}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}

// PrefixExpression
#[derive(Debug)]
pub struct PrefixExpression {
    // pub token: Token,
    pub op: Operator,
    pub right: Box<Expression>, // to break cycle for compiler
}
impl PrefixExpression {
    pub fn new(op: Operator, right: Expression) -> Self {
        PrefixExpression {
            op,
            right: Box::new(right),
        }
    }
}
impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.op.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::from("(");
        out.push_str(&self.op.to_string());
        out.push_str(&self.right.string());
        out.push(')');
        out
    }
}

// Infix Expression
#[derive(Debug)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub op: Operator,
    pub right: Box<Expression>,
}
impl InfixExpression {
    pub fn new(left: Expression, op: Operator, right: Expression) -> InfixExpression {
        InfixExpression {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}
impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.op.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::from("(");
        out.push_str(&self.left.string());
        out.push_str(&format!(" {} ", self.op));
        out.push_str(&self.right.string());
        out.push(')');
        out
    }
}

// Program
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program { statements }
    }
}
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return "".to_string();
        }
        // TODO:
        self.statements[0].token_literal()
    }

    fn string(&self) -> String {
        let mut string = String::new();

        for stmt in &self.statements {
            let mut out = stmt.string();
            out.push('\n');
            string.push_str(&out);
        }

        string
    }
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<_>>()
            .join(";\n");

        write!(f, "{}", output)
    }
}

// LetStatement
#[derive(Debug)]
pub struct LetStatement {
    pub keyword: Keyword,
    pub name: Identifier,
    pub value: Expression,
}
// impls for LetStatement
impl LetStatement {
    pub fn new(keyword: Keyword, name: Identifier, value: Expression) -> LetStatement {
        LetStatement {
            keyword,
            name,
            value,
        }
    }
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.keyword.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push(' ');
        out.push_str(self.name.string().as_str());
        out.push_str(" = ");

        out.push_str(self.value.string().as_str());
        out.push(';');

        out
    }
}

// ReturnStatement
#[derive(Debug)]
pub struct ReturnStatement {
    pub keyword: Keyword,
    pub return_val: Expression,
}
impl ReturnStatement {
    pub fn new(keyword: Keyword, return_val: Expression) -> ReturnStatement {
        ReturnStatement {
            keyword,
            return_val,
        }
    }
}
impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.keyword.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&(self.token_literal() + " "));
        match &self.return_val {
            Expression::Identifier(identifier) => out.push_str(identifier.string().as_str()),
            Expression::IntegerLiteral(integer_literal) => out.push_str(&integer_literal.string()),
            Expression::PrefixExpression(prefix_expression) => {
                out.push_str(&prefix_expression.string())
            }
            Expression::InfixExpression(infix_expression) => {
                out.push_str(&infix_expression.string())
            }
        }
        out.push(';');

        out
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}
impl ExpressionStatement {
    pub fn new(token: Token, expression: Expression) -> ExpressionStatement {
        ExpressionStatement { token, expression }
    }
    pub fn read(&self) {
        println!("{}", self.expression.token_literal());
    }
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        match &self.expression {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
        }
    }
}
