use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    object::obj::{
        BooleanObject, DoubleObject, ErrorObject, FunctionObject, NullObject, Object, ObjectTrait,
        ReturnObject,
    },
    parser::ast::{
        BlockStatement, Expression, IfExpression, InfixExpression, LetStatement, PrefixExpression,
        Program, Statement,
    },
    token::{Operator, Value},
};

fn is_error_object(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}

pub fn eval(node: &Program, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let mut result = None;
    for stmt in &node.statements {
        result = eval_statement(stmt, env.clone());
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

fn eval_block_statement(
    block_stmt: &BlockStatement,
    env: Rc<RefCell<Environment>>,
) -> Option<Object> {
    let mut result = None;
    for stmt in &block_stmt.statements {
        result = eval_statement(stmt, env.clone());
        match result {
            Some(Object::RetrunValue(_)) => return result,
            Some(Object::Error(_)) => return result,
            _ => {}
        }
    }
    result
}

fn eval_statement(stmt: &Statement, env: Rc<RefCell<Environment>>) -> Option<Object> {
    match stmt {
        Statement::Let(let_statement) => eval_let_statement(let_statement, env),
        Statement::Return(return_statement) => {
            let obj = eval_expression(&return_statement.return_val, env)?;
            if is_error_object(&obj) {
                return Some(obj);
            }
            Some(Object::RetrunValue(ReturnObject::new(obj)))
        }
        Statement::Expression(expression_statement) => {
            eval_expression(&expression_statement.expression, env)
        }
        Statement::BlockStatement(block_statement) => eval_block_statement(block_statement, env),
    }
}

fn eval_expression(expr: &Expression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    match expr {
        Expression::Identifier(identifier) => {
            let obj = env.borrow().get(&identifier.value);
            match obj {
                Some(_) => obj,
                None => Some(Object::Error(ErrorObject::new(format!(
                    "identifier not found: {}",
                    identifier.value
                )))),
            }
        }
        Expression::DoubleLiteral(double_literal) => Some(Object::Double(DoubleObject {
            value: double_literal.val,
        })),
        Expression::PrefixExpression(prefix_expression) => {
            eval_prefix_expression(prefix_expression, env.clone())
        }
        Expression::InfixExpression(infix_expression) => {
            eval_infix_expression(infix_expression, env.clone())
        }
        Expression::Boolean(boolean) => Some(Object::Boolean(BooleanObject::get(
            boolean.value == Value::True,
        ))),
        Expression::IfExpression(if_expression) => eval_if_expression(if_expression, env.clone()),
        Expression::FunctionLiteral(function_literal) => {
            Some(Object::Function(FunctionObject::new(
                function_literal.args.clone(),
                function_literal.body.clone(),
                env.clone(),
            )))
        }
        Expression::CallExpression(call_expression) => {
            let fn_obj = eval_expression(&call_expression.function, env.clone())?;
            if is_error_object(&fn_obj) {
                return Some(fn_obj);
            }
            let mut args = eval_expressions(&call_expression.args, env.clone())?;
            if args.len() == 1 && is_error_object(&args[0]) {
                let err_obj = args.pop()?;
                return Some(err_obj);
            }
            let fo = match fn_obj {
                Object::Function(function_object) => function_object,
                _ => return None,
            };
            apply_function(fo, args)
        }
    }
}
fn apply_function(func: FunctionObject, args: Vec<Object>) -> Option<Object> {
    // initialize env inside func
    let env = Rc::new(RefCell::new(Environment::default()));
    // IMPORTANT: as we set the outer to be the func env above it so that we can
    // continue looking upward if there is any variable with the known variable_name value
    env.borrow_mut().set_outer(func.env);
    for (idx, ident) in func.args.iter().enumerate() {
        env.borrow_mut()
            .set(ident.value.to_string(), args[idx].clone());
    }

    // eval
    match eval_block_statement(&func.body, env)? {
        Object::RetrunValue(return_object) => Some(*return_object.value),
        o => {
            // match &o {
            //     Object::Function(fo) => {
            //         println!("here");
            //         let vals: Vec<String> = fo
            //             .env
            //             .borrow()
            //             .iter()
            //             .map(|(k, v)| format!("{}: {}", k, v.inspect()))
            //             .collect();
            //
            //         println!("{:?} {}", vals, fo.inspect());
            //     }
            //     _ => {}
            // }
            Some(o)
        }
    }
}

fn eval_expressions(exprs: &Vec<Expression>, env: Rc<RefCell<Environment>>) -> Option<Vec<Object>> {
    let mut objs: Vec<Object> = Vec::new();
    for expr in exprs {
        let obj = eval_expression(expr, env.clone())?;
        if is_error_object(&obj) {
            return Some(vec![obj]);
        }
        objs.push(obj);
    }
    Some(objs)
}

fn eval_prefix_expression(
    prefix_expr: &PrefixExpression,
    env: Rc<RefCell<Environment>>,
) -> Option<Object> {
    let (op, right) = (&prefix_expr.op, &prefix_expr.right);
    let val = eval_expression(right, env)?;
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

fn eval_infix_expression(
    infix_expr: &InfixExpression,
    env: Rc<RefCell<Environment>>,
) -> Option<Object> {
    let (left_expr, op, right_expr) = (&infix_expr.left, &infix_expr.op, &infix_expr.right);
    let left = eval_expression(left_expr, env.clone())?;
    if is_error_object(&left) {
        return Some(left);
    }
    let right = eval_expression(right_expr, env.clone())?;
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

fn eval_if_expression(if_expr: &IfExpression, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let IfExpression {
        condition,
        consequence,
        alternative,
    } = if_expr;

    let obj = eval_expression(condition, env.clone())?;
    if is_error_object(&obj) {
        return Some(obj);
    }
    let flag = match obj {
        Object::Boolean(boolean_object) => boolean_object.value,
        Object::Double(double_object) => double_object.value != 0.0,
        _ => false,
    };

    if flag {
        eval_block_statement(consequence, env.clone())
    } else if let Some(alt) = alternative {
        eval_block_statement(alt, env.clone())
    } else {
        Some(Object::Null(NullObject::get()))
    }
}

fn eval_let_statement(ls: &LetStatement, env: Rc<RefCell<Environment>>) -> Option<Object> {
    let obj = eval_expression(&ls.value, env.clone())?;
    if is_error_object(&obj) {
        return Some(obj);
    }
    env.borrow_mut().set(ls.name.value.clone(), obj.clone());
    Some(obj)
}
