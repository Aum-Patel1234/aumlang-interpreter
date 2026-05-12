use crate::{
    object::obj::{
        BooleanObject, DoubleObject, ErrorObject, NullObject, Object, ObjectTrait, ReturnObject,
    },
    parser::ast::{
        BlockStatement, Expression, IfExpression, InfixExpression, PrefixExpression, Program,
        Statement,
    },
    token::{Operator, Value},
};

fn is_error_object(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}

pub fn eval(node: &Program) -> Option<Object> {
    let mut result = None;
    for stmt in &node.statements {
        result = eval_statement(stmt);
        match result {
            Some(Object::RetrunValue(return_object)) => {
                return Some(*return_object.value);
            }
            Some(Object::Error(_)) => {
                return result;
            }
            _ => {}
        }
    }
    result
}

fn eval_block_statement(block_stmt: &BlockStatement) -> Option<Object> {
    let mut result = None;
    for stmt in &block_stmt.statements {
        result = eval_statement(stmt);
        match result {
            Some(Object::RetrunValue(_)) => return result,
            Some(Object::Error(_)) => return result,
            _ => {}
        }
    }
    result
}

fn eval_statement(stmt: &Statement) -> Option<Object> {
    match stmt {
        // Statement::Let(let_statement) => None,
        Statement::Return(return_statement) => {
            let obj = eval_expression(&return_statement.return_val)?;
            if is_error_object(&obj) {
                return Some(obj);
            }
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
    if is_error_object(&val) {
        return Some(val);
    }

    match op {
        Operator::Minus => eval_minus_prefix_expression(&val),
        Operator::Exclamation => eval_exclamation_operator(&val),
        o => Some(Object::Error(ErrorObject::new(format!(
            "unknown operator: {}{}",
            o,
            val.object_type()
        )))),
    }
}

fn eval_minus_prefix_expression(val: &Object) -> Option<Object> {
    match val {
        Object::Double(double_object) => Some(Object::Double(DoubleObject {
            value: -double_object.value,
        })),
        o => Some(Object::Error(ErrorObject::new(format!(
            "unknown operator: -{}",
            o.object_type(),
        )))),
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
    if is_error_object(&left) {
        return Some(left);
    }
    let right = eval_expression(right_expr)?;
    if is_error_object(&right) {
        return Some(right);
    }

    match (&left, &right) {
        (Object::Double(lo), Object::Double(ro)) => {
            let (l, r) = (lo.value, ro.value);
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

                _ => {
                    return Some(Object::Error(ErrorObject::new(format!(
                        "unknown operator: {} {} {}",
                        lo.object_type(),
                        op,
                        ro.object_type()
                    ))));
                }
            };

            Some(obj)
        }

        // NOTE: here i could have assigned 1.0->true and 0.0->false and treated like double
        // but i choose not to. then it would have behaved like node/python interpreter
        (Object::Boolean(l), Object::Boolean(r)) => match op {
            Operator::EQ => Some(Object::Boolean(BooleanObject::get(l == r))),
            Operator::NEQ => Some(Object::Boolean(BooleanObject::get(l != r))),
            _ => Some(Object::Error(ErrorObject::new(format!(
                "unknown operator: {} {} {}",
                l.object_type(),
                op,
                r.object_type()
            )))),
        },

        // _ => None,
        (l, r) => {
            let err_msg = if l.object_type() != r.object_type() {
                format!(
                    "type mismatch: {} {} {}",
                    l.object_type(),
                    op,
                    r.object_type()
                )
            } else {
                format!(
                    "unknown operator: {} {} {}",
                    l.object_type(),
                    op,
                    r.object_type()
                )
            };

            Some(Object::Error(ErrorObject::new(err_msg)))
        }
    }
}

fn eval_if_expression(if_expr: &IfExpression) -> Option<Object> {
    let IfExpression {
        condition,
        consequence,
        alternative,
    } = if_expr;

    let obj = eval_expression(condition)?;
    if is_error_object(&obj) {
        return Some(obj);
    }
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
