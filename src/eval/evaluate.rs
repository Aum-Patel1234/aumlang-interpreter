use crate::{
    object::obj::{BooleanObject, DoubleObject, Object},
    parser::ast::{Expression, Program, Statement},
    token::Value,
};

pub fn eval(node: &Program) -> Option<Object> {
    node.statements
        .iter()
        .filter_map(eval_statement)
        .next_back()
}

fn eval_statement(stmt: &Statement) -> Option<Object> {
    match stmt {
        // Statement::Let(let_statement) => None,
        // Statement::Return(return_statement) => None,
        Statement::Expression(expression_statement) => {
            eval_expression(&expression_statement.expression)
        }
        // Statement::BlockStatement(block_statement) => None,
        _ => None,
    }
}

fn eval_expression(expr: &Expression) -> Option<Object> {
    match expr {
        // Expression::Identifier(identifier) => None,
        Expression::DoubleLiteral(double_literal) => Some(Object::Double(DoubleObject {
            value: double_literal.val,
        })),
        // Expression::PrefixExpression(prefix_expression) => None,
        // Expression::InfixExpression(infix_expression) => None,
        Expression::Boolean(boolean) => Some(Object::Boolean(BooleanObject::get(
            boolean.value == Value::True,
        ))),
        // Expression::IfExpression(if_expression) => None,
        // Expression::FunctionLiteral(function_literal) => None,
        // Expression::CallExpression(call_expression) => None,
        _ => None,
    }
}
