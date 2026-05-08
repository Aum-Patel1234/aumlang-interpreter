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
    BlockStatement(BlockStatement),
}
impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token_literal(),
            Statement::Return(s) => s.token_literal(),
            Statement::Expression(expression_statement) => expression_statement.token_literal(),
            Statement::BlockStatement(block_statement) => block_statement.token_literal(),
        }
    }

    fn string(&self) -> String {
        match &self {
            Statement::Return(return_statement) => return_statement.string(),
            Statement::Let(let_statement) => let_statement.string(),
            Statement::Expression(expression_statement) => expression_statement.string(),
            Statement::BlockStatement(block_statement) => block_statement.string(),
        }
    }
}

// NOTE: i have ignored expressionNode() in the book
#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    DoubleLiteral(DoubleLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal(),
            Expression::DoubleLiteral(double_literal) => double_literal.token_literal(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.token_literal(),
            Expression::InfixExpression(infix_expression) => infix_expression.token_literal(),
            Expression::Boolean(boolean) => boolean.token_literal(),
            Expression::IfExpression(if_expression) => if_expression.token_literal(),
            Expression::FunctionLiteral(function_literal) => function_literal.token_literal(),
            Expression::CallExpression(call_expression) => call_expression.token_literal(),
        }
    }

    fn string(&self) -> String {
        match &self {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::DoubleLiteral(double_literal) => double_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
            Expression::Boolean(boolean) => boolean.string(),
            Expression::IfExpression(if_expression) => if_expression.string(),
            Expression::FunctionLiteral(function_literal) => function_literal.string(),
            Expression::CallExpression(call_expression) => call_expression.string(),
        }
    }
}

// Identifier
#[derive(Debug, Clone)]
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

// DoubleLiteral
#[derive(Debug)]
pub struct DoubleLiteral {
    pub token: Token,
    pub val: f64,
}
impl DoubleLiteral {
    pub fn new(token: Token) -> Result<DoubleLiteral, String> {
        if let Token::Value(Value::Double(v)) = token {
            Ok(DoubleLiteral { token, val: v })
        } else {
            Err(format!("Expected numeric literal, got {:?}", token))
        }
    }
}
impl Node for DoubleLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        self.token.to_string()
    }
}

// Boolean
#[derive(Debug)]
pub struct Boolean {
    pub value: Value,
}
impl Boolean {
    pub fn new(token: Token) -> Result<Boolean, String> {
        match token {
            Token::Keyword(kw) => match kw {
                Keyword::TRUE => Ok(Boolean { value: Value::True }),
                Keyword::FALSE => Ok(Boolean {
                    value: Value::False,
                }),
                k => Err(format!("Expected Keyword::True/False, found {}", k)),
            },
            e => Err(format!("Expected boolean token, found {}", e)),
        }
    }
}
impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.value.to_string()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

// If Expression
#[derive(Debug)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl IfExpression {
    pub fn new(
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }
    }
}
impl Node for IfExpression {
    fn token_literal(&self) -> String {
        Token::Keyword(Keyword::IF).to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&Token::Keyword(Keyword::IF).to_string());
        out.push_str(&self.condition.string());
        out.push(' ');
        out.push_str(&self.consequence.string());
        if let Some(b) = &self.alternative {
            out.push_str("else");
            out.push_str(&b.string());
        }
        out
    }
}

#[derive(Debug)]
pub struct FunctionLiteral {
    // token: Token,
    // QUESTION: Weather it being Vec<Expression> would be good ?
    // it would enable to parse a=9, fn(x,y){x} as fn args
    // For now idk
    // Ans: now i know its beign done in the function call part not here.
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
}
impl FunctionLiteral {
    pub fn new(args: Vec<Identifier>, body: BlockStatement) -> Self {
        FunctionLiteral {
            // token: Token::Keyword(Keyword::FUNCTION),
            args,
            body,
        }
    }
}
impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        Token::Keyword(Keyword::FUNCTION).to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ( ");
        out.push_str(
            &self
                .args
                .iter()
                .map(|i| i.string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push_str(" ){\n");
        out.push_str(&self.body.string());
        out.push('}');
        out
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub args: Vec<Expression>,
}
impl CallExpression {
    pub fn new(function: Expression, args: Vec<Expression>) -> Self {
        CallExpression {
            function: Box::new(function),
            args,
        }
    }
}
impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.function.token_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push('(');
        out.push_str(
            &self
                .args
                .iter()
                .map(|i| i.string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push(')');
        out
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
            Expression::DoubleLiteral(double_literal) => out.push_str(&double_literal.string()),
            Expression::PrefixExpression(prefix_expression) => {
                out.push_str(&prefix_expression.string())
            }
            Expression::InfixExpression(infix_expression) => {
                out.push_str(&infix_expression.string())
            }
            Expression::Boolean(boolean) => out.push_str(&boolean.string()),
            Expression::IfExpression(if_expression) => out.push_str(&if_expression.string()),
            Expression::FunctionLiteral(function_literal) => {
                out.push_str(&function_literal.string())
            }
            Expression::CallExpression(call_expression) => out.push_str(&call_expression.string()),
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
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        match &self.expression {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::DoubleLiteral(double_literal) => double_literal.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
            Expression::Boolean(boolean) => boolean.string(),
            Expression::IfExpression(if_expression) => if_expression.string(),
            Expression::FunctionLiteral(function_literal) => function_literal.string(),
            Expression::CallExpression(call_expression) => call_expression.string(),
        }
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        Token::LBrace.to_string()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }

        out
    }
}
