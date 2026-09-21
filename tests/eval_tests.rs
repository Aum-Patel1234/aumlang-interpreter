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
fn test_string_object_hello_world() {
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
#[inline]
fn test_string_object(object: Object, expected: &str) -> bool {
    match object {
        Object::StringObj(string_object) => string_object.value == expected,
        _ => false,
    }
}
#[test]
fn test_builtin_string_case_functions() {
    enum Expected {
        String(&'static str),
        Error(&'static str),
    }

    let tests = [
        // lower()
        ("lower(\"\")", Expected::String("")),
        ("lower(\"HELLO\")", Expected::String("hello")),
        ("lower(\"Hello World\")", Expected::String("hello world")),
        ("lower(\"Rust123!\")", Expected::String("rust123!")),
        ("lower(1)", Expected::Error("Expected string got DOUBLE")),
        (
            "lower(\"one\", \"two\")",
            Expected::Error("Expected one string argument in builtin lower() func got 2 args"),
        ),
        // upper()
        ("upper(\"\")", Expected::String("")),
        ("upper(\"hello\")", Expected::String("HELLO")),
        ("upper(\"Hello World\")", Expected::String("HELLO WORLD")),
        ("upper(\"Rust123!\")", Expected::String("RUST123!")),
        ("upper(1)", Expected::Error("Expected string got DOUBLE")),
        (
            "upper(\"one\", \"two\")",
            Expected::Error("Expected one string argument in builtin upper() func got 2 args"),
        ),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);

        match expected {
            Expected::String(expected_value) => {
                assert!(
                    test_string_object(evaluated, expected_value),
                    "expected {}",
                    expected_value
                );
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

#[test]
fn test_identifier_and_assignment() {
    let double_tests = [
        // Basic identifier evaluation
        ("let a = 5; a;", 5.0),
        // Basic assignment
        ("let a = 5; a = 9;", 9.0),
        ("let a = 5; a = 5 + 4;", 9.0),
        ("let a = 5; a = 9; a;", 9.0),
        // Multiple reassignment
        ("let a = 1; a = 2; a = 3; a;", 3.0),
        // Assignment using another identifier
        ("let a = 5; let b = 10; a = b + 2; a;", 12.0),
        // Assignment using a more complex expression
        ("let a = 0; a = 2 * 3 + 4;", 10.0),
        ("let a = 0; a = (2 + 3) * 4;", 20.0),
        ("let a = 10; a = a + 5;", 15.0),
        ("let a = 10; a = a * 2;", 20.0),
        // Chained assignment
        ("let a = 0; let b = 0; a = (b = 9);", 9.0),
        ("let a = 0; let b = 0; a = (b = 9); a;", 9.0),
        ("let a = 0; let b = 0; a = (b = 9); b;", 9.0),
        // Chained assignment with an expression
        ("let a = 0; let b = 0; a = (b = 4 + 5);", 9.0),
        ("let a = 0; let b = 0; a = (b = 2 * 3); a + b;", 12.0),
    ];

    for (input, expected) in double_tests {
        test_double_object(test_eval(input), expected);
    }

    let string_tests = [
        ("let a = \"hello\"; a;", "hello"),
        ("let a = \"hello\"; a = \"world\";", "world"),
        ("let a = \"hello\"; a = \"world\"; a;", "world"),
    ];

    for (input, expected) in string_tests {
        assert!(
            test_string_object(test_eval(input), expected),
            "Expected StringObj with value {:?} for input: {}",
            expected,
            input
        );
    }

    let boolean_tests = [
        ("let a = true; a;", true),
        ("let a = true; a = false;", false),
        ("let a = false; a = true; a;", true),
    ];

    for (input, expected) in boolean_tests {
        test_boolean_object(test_eval(input), expected);
    }
}
#[test]
fn test_assignment_to_undefined_identifier() {
    let tests = [
        ("a = 9;", "identifier not found: a"),
        ("let a = 0; a = (b = 9);", "identifier not found: b"),
    ];

    for (input, expected_error) in tests {
        match test_eval(input) {
            Object::Error(error) => {
                assert_eq!(
                    error.msg, expected_error,
                    "Unexpected error for input: {}",
                    input
                );
            }
            object => panic!("Expected error for input {}, got {}", input, object),
        }
    }
}
#[test]
fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";

    let evaluated = test_eval(input);

    let array = match evaluated {
        Object::Array(array) => array,
        object => {
            panic!("object is not Array. got={}", object);
        }
    };

    assert_eq!(
        array.arr.borrow().len(),
        3,
        "array has wrong number of elements. got={}",
        array.arr.borrow().len()
    );

    assert!(test_double_object(
        array.arr.borrow().get(0).expect("element 0 should exist"),
        1.0
    ));
    assert!(test_double_object(
        array.arr.borrow().get(1).expect("element 1 should exist"),
        4.0
    ));
    assert!(test_double_object(
        array.arr.borrow().get(2).expect("element 2 should exist"),
        6.0
    ));
    assert!(array.arr.borrow().get(100).is_none());
}
#[test]
fn test_array_append() {
    let input = r#"
        let a = [1, 2, 3];
        append(a, 4, 5);
        a
    "#;

    let evaluated = test_eval(input);

    let array = match evaluated {
        Object::Array(array) => array,
        object => {
            panic!("object is not Array. got={}", object);
        }
    };

    let arr = array.arr.borrow();

    assert_eq!(
        arr.len(),
        5,
        "array should have 5 elements after append. got={}",
        arr.len()
    );

    assert!(test_double_object(
        arr.get(0).expect("element 0 should exist"),
        1.0
    ));

    assert!(test_double_object(
        arr.get(1).expect("element 1 should exist"),
        2.0
    ));

    assert!(test_double_object(
        arr.get(2).expect("element 2 should exist"),
        3.0
    ));

    assert!(test_double_object(
        arr.get(3).expect("element 3 should exist"),
        4.0
    ));

    assert!(test_double_object(
        arr.get(4).expect("element 4 should exist"),
        5.0
    ));

    assert!(arr.get(100).is_none());
}
#[test]
fn test_len() {
    let tests = [
        (r#"len([1, 2, 3])"#, 3.0),
        (r#"len([])"#, 0.0),
        (r#"len(append([1, 2], 3))"#, 3.0),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);

        assert!(
            test_double_object(evaluated, expected),
            "expected {} for input `{}`",
            expected,
            input
        );
    }
}
#[test]
fn test_array_index_expressions() {
    enum Expected {
        Double(f64),
        Error(&'static str),
    }

    let tests = [
        ("[1, 2, 3][0]", Expected::Double(1.0)),
        ("[1, 2, 3][1]", Expected::Double(2.0)),
        ("[1, 2, 3][2]", Expected::Double(3.0)),
        ("let i = 0; [1][i];", Expected::Double(1.0)),
        ("[1, 2, 3][1 + 1];", Expected::Double(3.0)),
        (
            "let myArray = [1, 2, 3]; myArray[2];",
            Expected::Double(3.0),
        ),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            Expected::Double(6.0),
        ),
        (
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
            Expected::Double(2.0),
        ),
        (
            "[1, 2, 3][3]",
            Expected::Error("Index out of bound: 3 max capacity = 3"),
        ),
        (
            "[1, 2, 3][-1]",
            Expected::Error("Index cannot be negative: -1"),
        ),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);

        match expected {
            Expected::Double(expected_value) => {
                assert!(
                    test_double_object(evaluated.clone(), expected_value),
                    "expected {} for input `{}`, got {}",
                    expected_value,
                    input,
                    evaluated
                );
            }

            Expected::Error(expected_error) => match evaluated {
                Object::Error(error_object) => {
                    assert_eq!(
                        error_object.msg, expected_error,
                        "unexpected error for input `{}`",
                        input
                    );
                }
                object => {
                    panic!("expected ERROR for input `{}`, got {}", input, object);
                }
            },
        }
    }
}
