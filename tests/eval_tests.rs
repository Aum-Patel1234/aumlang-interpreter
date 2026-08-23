use core::panic;
use std::{cell::RefCell, rc::Rc};

use aumlang::{
    environment::Environment,
    eval::evaluate::eval,
    lexer::Lexer,
    object::obj::{BOOLEAN_OBJ, DOUBLE_OBJ, Object, ObjectTrait},
    parser::{Node, parser_logic::Parser},
};

fn test_eval(input: &str) -> Object {
    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());
    let env = Rc::new(RefCell::new(Environment::default()));
    match eval(&program, env) {
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
#[test]
fn test_string_object() {
    let input = "\"Hello World!\"";
    let eveal = test_eval(input);
    match eveal {
        Object::StringObj(string_object) => assert_eq!(string_object.value, "Hello World!"),
        _ => panic!("Object is not a StringObj"),
    }
}
#[test]
fn test_string_concatenation() {
    let input = "\"Hello World!\" + \" new str\" + 9";
    let eveal = test_eval(input);
    match eveal {
        Object::StringObj(string_object) => {
            assert_eq!(string_object.value, "Hello World! new str9")
        }
        _ => panic!("Object is not a StringObj"),
    }
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
        ("\"a\" == \"a\"", true),
        ("\"ab\" != \"a\"", true),
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

#[test]
fn test_if_else_expression() {
    let tests = [
        ("if (true) { 10 }", Some(10.0)),
        ("if (false) { 10 }", None),
        ("if (1) { 10 }", Some(10.0)),
        ("if (1 < 2) { 10 }", Some(10.0)),
        ("if (1 > 2) { 10 }", None),
        ("if (1 > 2) { 10 } else { 20 }", Some(20.0)),
        ("if (1 < 2) { 10 } else { 20 }", Some(10.0)),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        match expected {
            Some(v) => assert!(test_double_object(evaluated, v)),
            None => assert!(test_null_obj(evaluated)),
        }
    }
}

fn test_null_obj(obj: Object) -> bool {
    matches!(obj, Object::Null(_))
}

#[test]
fn test_return_statements() {
    let tests = [
        ("return 10;", 10.0),
        ("return 10; 9;", 10.0),
        ("return 2 * 5; 9;", 10.0),
        ("9; return 2 * 5; 9;", 10.0),
        (
            r#"
            if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }
            "#,
            10.0,
        ),
    ];
    for (s, v) in tests {
        let evaluated = test_eval(s);
        assert!(test_double_object(evaluated, v))
    }
}

#[test]
fn test_error_handling() {
    let tests = [
        ("5 + true;", "type mismatch: DOUBLE + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: DOUBLE + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("\"Hello\" - \"World\"", "unknown operator: STRING - STRING"),
        (
            r#"
            if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }
            "#,
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("foobar", "identifier not found: foobar"),
    ];

    for (s, err) in tests {
        println!("{}", s);
        let evaluated = test_eval(s);
        let e = match evaluated {
            Object::Error(error) => error,
            o => panic!("No error Object returned, got {}", o),
        };
        if e.msg != err {
            panic!("Wrong error message, expected - {}, got - {}", err, e.msg)
        }
    }
}

#[test]
fn test_let_statements() {
    let tests = [
        ("let a = 5; a; 5;", 5),
        ("let a = 5*5;a;", 25),
        ("let a = 5; let b = a;b;", 5),
        ("let a = 5; let b = a; let c = a+b+5; c;", 15),
    ];
    for (s, v) in tests {
        test_double_object(test_eval(s), v as f64);
    }
}

#[test]
fn test_function_object() {
    let input = "fn(x) {x+2;}";
    let evaluated = test_eval(input);
    let fo = match evaluated {
        Object::Function(function_object) => function_object,
        o => panic!("Expected function_object, got  {}", o),
    };
    assert_eq!(fo.args.len(), 1);
    assert_eq!(fo.args[0].value, "x");
    assert_eq!(fo.body.string(), "(x + Value::Double(2))");
}

#[test]
fn test_function_application() {
    let tests = [
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];
    for (s, v) in tests {
        test_double_object(test_eval(s), v as f64);
    }
}

#[test]
fn test_closures() {
    let input = r#"
    let newAdder = fn(x) {
        fn(y) { x + y };
    };
    let addTwo = newAdder(2);
    addTwo(2);
    "#;
    test_double_object(test_eval(input), 4.0);
}

#[test]
fn test_builtin_functions() {
    enum Expected {
        Double(f64),
        Error(&'static str),
    }
    let tests = [
        ("len(\"\")", Expected::Double(0f64)),
        ("len(\"four\")", Expected::Double(4f64)),
        ("len(\"hello world\")", Expected::Double(11f64)),
        ("len(1)", Expected::Error("Expected string got DOUBLE")),
        (
            "len(\"one\", \"two\")",
            Expected::Error("Expected one string argument in builtin len() func got 2 args"),
        ),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        match expected {
            Expected::Double(v) => {
                assert!(test_double_object(evaluated, v), "expected {}", v);
            }
            Expected::Error(expected_error) => match evaluated {
                Object::Error(error_object) => {
                    assert_eq!(error_object.msg, expected_error);
                }

                object => {
                    panic!("Expected error object, got {}", object);
                }
            },
        }
    }
}
