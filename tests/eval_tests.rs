use aumlang::{
    eval::evaluate::eval,
    lexer::Lexer,
    object::obj::{BOOLEAN_OBJ, DOUBLE_OBJ, Object, ObjectTrait},
    parser::parser_logic::Parser,
};

fn test_eval(input: &str) -> Object {
    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());
    match eval(&program) {
        Some(e) => e,
        None => panic!("Expected Box<dyn Object> got None"),
    }
}

fn test_double_object(obj: Object, expected: f64) -> bool {
    assert_eq!(obj.object_type(), DOUBLE_OBJ);

    let double_obj = match obj {
        Object::Double(double_object) => double_object,
        o => panic!("Expected DoubleObject, got {}", o),
    };

    assert_eq!(double_obj.value, expected);
    true
}
fn test_boolean_object(obj: Object, expected: bool) -> bool {
    assert_eq!(obj.object_type(), BOOLEAN_OBJ);

    let bool_obj = match obj {
        Object::Boolean(boolean_object) => boolean_object,
        o => panic!("Expected BooleanObject, got {}", o),
    };

    assert_eq!(bool_obj.value, expected);
    true
}

#[test]
fn test_eval_double_expression() {
    let tests = [("5", 5.0), ("10", 10.0)];

    for (s, v) in tests {
        let evaluated = test_eval(s);
        assert!(test_double_object(evaluated, v));
    }
}

#[test]
fn test_eval_boolean_expression() {
    let tests = [("true", true), ("false", false)];

    for (s, v) in tests {
        let evaluated = test_eval(s);
        assert!(test_boolean_object(evaluated, v));
    }
}
