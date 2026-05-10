use crate::{
    object::obj::{BooleanObject, DoubleObject, NullObject, Object, ReturnObject},
    parser::ast::{
        BlockStatement, Expression, IfExpression, InfixExpression, PrefixExpression, Program,
        Statement,
    },
    token::{Operator, Value},
};

pub fn eval(node: &Program) -> Option<Object> {
    let mut result = None;
    for stmt in &node.statements {
        result = eval_statement(stmt);
        if let Some(Object::RetrunValue(val)) = result {
            return Some(*val.value);
        }
    }
    result
}

fn eval_statement(stmt: &Statement) -> Option<Object> {
    match stmt {
        // Statement::Let(let_statement) => None,
        Statement::Return(return_statement) => {
            let obj = eval_expression(&return_statement.return_val)?;
            Some(Object::RetrunValue(ReturnObject::new(obj)))
        }
        Statement::Expression(expression_statement) => {
            eval_expression(&expression_statement.expression)
        }
        Statement::BlockStatement(block_statement) => eval_block_statement(block_statement),
        _ => None,
    }
}

fn eval_expression(expr: &Expression) -> Option<Object> {
    match expr {
        // Expression::Identifier(identifier) => None,
        Expression::DoubleLiteral(double_literal) => Some(Object::Double(DoubleObject {
            value: double_literal.val,
        })),
        Expression::PrefixExpression(prefix_expression) => {
            eval_prefix_expression(prefix_expression)
        }
        Expression::InfixExpression(infix_expression) => eval_infix_expression(infix_expression),
        Expression::Boolean(boolean) => Some(Object::Boolean(BooleanObject::get(
            boolean.value == Value::True,
        ))),
        Expression::IfExpression(if_expression) => eval_if_expression(if_expression),
        // Expression::FunctionLiteral(function_literal) => None,
        // Expression::CallExpression(call_expression) => None,
        _ => None,
    }
}

fn eval_prefix_expression(prefix_expr: &PrefixExpression) -> Option<Object> {
    let (op, right) = (&prefix_expr.op, &prefix_expr.right);
    let val = eval_expression(right)?;

    match op {
        Operator::Minus => eval_minus_prefix_expression(&val),
        Operator::Exclamation => eval_exclamation_operator(&val),
        _ => None,
    }
}

fn eval_minus_prefix_expression(val: &Object) -> Option<Object> {
    match val {
        Object::Double(double_object) => Some(Object::Double(DoubleObject {
            value: -double_object.value,
        })),
        _ => None,
    }
}

fn eval_exclamation_operator(val: &Object) -> Option<Object> {
    match val {
        Object::Double(double_object) => {
            let obj = BooleanObject::get_from_num(double_object.value);
            Some(Object::Boolean(obj.not_get()))
        }
        Object::Boolean(boolean_object) => Some(Object::Boolean(boolean_object.not_get())),
        _ => Some(Object::Boolean(BooleanObject::get(true))),
    }
}

fn eval_infix_expression(infix_expr: &InfixExpression) -> Option<Object> {
    let (left_expr, op, right_expr) = (&infix_expr.left, &infix_expr.op, &infix_expr.right);
    let left = eval_expression(left_expr)?;
    let right = eval_expression(right_expr)?;

    match (&left, &right) {
        (Object::Double(_), Object::Double(_)) => {
            let l = get_double_value_or_none(left)?;
            let r = get_double_value_or_none(right)?;

            let obj = match op {
                Operator::Plus => Object::Double(DoubleObject { value: l + r }),
                Operator::Minus => Object::Double(DoubleObject { value: l - r }),
                Operator::Star => Object::Double(DoubleObject { value: l * r }),
                Operator::Slash => Object::Double(DoubleObject { value: l / r }),

                Operator::GT => Object::Boolean(BooleanObject::get(l > r)),
                Operator::LT => Object::Boolean(BooleanObject::get(l < r)),
                Operator::EQ => Object::Boolean(BooleanObject::get(l == r)),
                Operator::NEQ => Object::Boolean(BooleanObject::get(l != r)),
                Operator::GTE => Object::Boolean(BooleanObject::get(l >= r)),
                Operator::LTE => Object::Boolean(BooleanObject::get(l <= r)),

                _ => return None,
            };

            Some(obj)
        }

        // NOTE: here i could have assigned 1.0->true and 0.0->false and treated like double
        // but i choose not to. then it would have behaved like node/python interpreter
        (Object::Boolean(_), Object::Boolean(_)) => {
            let l = get_boolean_or_none(left)?;
            let r = get_boolean_or_none(right)?;

            match op {
                Operator::EQ => Some(Object::Boolean(BooleanObject::get(l == r))),
                Operator::NEQ => Some(Object::Boolean(BooleanObject::get(l != r))),
                _ => None,
            }
        }

        _ => None,
    }
}

#[inline]
fn get_double_value_or_none(obj: Object) -> Option<f64> {
    match obj {
        Object::Double(double_object) => Some(double_object.value),
        _ => None,
    }
}

#[inline]
fn get_boolean_or_none(obj: Object) -> Option<&'static BooleanObject> {
    match obj {
        Object::Boolean(bo) => Some(bo),
        _ => None,
    }
}

fn eval_block_statement(block_stmt: &BlockStatement) -> Option<Object> {
    let mut result = None;
    for stmt in &block_stmt.statements {
        result = eval_statement(stmt);
        if let Some(Object::RetrunValue(_)) = &result {
            return result;
        }
    }
    result
}
fn eval_if_expression(if_expr: &IfExpression) -> Option<Object> {
    let IfExpression {
        condition,
        consequence,
        alternative,
    } = if_expr;

    let obj = eval_expression(condition)?;
    let flag = match obj {
        Object::Boolean(boolean_object) => boolean_object.value,
        Object::Double(double_object) => double_object.value != 0.0,
        _ => false,
    };

    if flag {
        eval_block_statement(consequence)
    } else if let Some(alt) = alternative {
        eval_block_statement(alt)
    } else {
        Some(Object::Null(NullObject::get()))
    }
}
