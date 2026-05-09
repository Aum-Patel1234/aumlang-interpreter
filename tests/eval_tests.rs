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
    let tests = [
        ("5", 5.0),
        ("10", 10.0),
        ("-5", -5.0),
        ("-10", -10.0),
        ("5 + 5 + 5 + 5 - 10", 10.0),
        ("2 * 2 * 2 * 2 * 2", 32.0),
        ("-50 + 100 + -50", 0.0),
        ("5 * 2 + 10", 20.0),
        ("5 + 2 * 10", 25.0),
        ("20 + 2 * -10", 0.0),
        ("50 / 2 * 2 + 10", 60.0),
        ("2 * (5 + 10)", 30.0),
        ("3 * 3 * 3 + 10", 37.0),
        ("3 * (3 * 3) + 10", 37.0),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50.0),
    ];

    for (s, v) in tests {
        let evaluated = test_eval(s);
        assert!(test_double_object(evaluated, v));
    }
}

#[test]
fn test_eval_boolean_expression() {
    let tests = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (s, v) in tests {
        let evaluated = test_eval(s);
        assert!(test_boolean_object(evaluated, v));
    }
}

#[test]
fn test_exclamation_operator() {
    let tests = [
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (s, v) in tests {
        let evaluated = test_eval(s);
        test_boolean_object(evaluated, v);
    }
}
