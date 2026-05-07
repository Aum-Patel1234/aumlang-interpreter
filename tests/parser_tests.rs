use core::panic;

use aumlang::{
    lexer::Lexer,
    parser::{
        // Node,
        Node,
        ast::{Expression, Statement},
        parser_logic::Parser,
    },
    token::Value,
};

#[test]
fn test_let_parser() {
    let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
    "#;

    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    p.check_parse_errors();
    let tests = ["x", "y", "foobar"];

    assert_eq!(
        program.statements.len(),
        tests.len(),
        "Mismatch: number of parsed statements does not match number of tests"
    );
    for (i, test) in tests.iter().enumerate() {
        let stmt = program.statements.get(i);
        if let Some(s) = stmt
            && !test_let_statement(s, test)
        {
            return;
        }
    }
}
#[test]
fn test_parser_error() {
    let input = r#"
        let foobar = 838383;
        let x  5;
        let y - 10;
    "#;

    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    assert!(
        !p.get_errors().is_empty(),
        "Expected parser errors but got none"
    );

    // let tests = ["x", "y", "foobar"];
    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, got {}",
        program.statements.len()
    );
    // for (i, test) in tests.iter().enumerate() {
    //     let stmt = program.statements.get(i);
    //     if let Some(s) = stmt
    //         && !test_let_statement(s, test)
    //     {
    //         return;
    //     }
    // }
}

fn test_let_statement(s: &Statement, name: &str) -> bool {
    if s.token_literal() != "let" {
        return false;
    }

    let lstmt = match s {
        Statement::Let(stmt) => stmt,
        _ => return false,
    };
    if lstmt.name.value != name {
        return false;
    }

    if lstmt.name.token_literal() != name {
        return false;
    }

    true
}

#[test]
fn test_return_statements() {
    let input = r#"
        return 5;
        return 10;
        return 993322;
    "#;

    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    assert!(p.check_parse_errors());

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    for stmt in program.statements.iter() {
        match stmt {
            Statement::Return(return_stmt) => {
                assert_eq!(
                    return_stmt.token_literal(),
                    "return",
                    "returnStmt.token_literal not 'return'"
                );
            }
            _ => {
                panic!("stmt not ReturnStatement. got={:?}", stmt);
            }
        }
    }
}

#[test]
fn test_mixed_statements() {
    let input = r#"
        foobar;
        let x = y;
        return z;
        hello;
        let a = b;
        return c;
    "#;

    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    assert!(p.check_parse_errors(), "Parser had errors");

    assert_eq!(
        program.statements.len(),
        6,
        "Expected 6 statements, got {}",
        program.statements.len()
    );

    let expected = [
        ("expr", "foobar", ""),
        ("let", "x", "y"),
        ("return", "z", ""),
        ("expr", "hello", ""),
        ("let", "a", "b"),
        ("return", "c", ""),
    ];

    for (i, (kind, name, val)) in expected.iter().enumerate() {
        match (&program.statements[i], *kind) {
            (Statement::Expression(expr_stmt), "expr") => {
                let expr = &expr_stmt.expression;

                match expr {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.value, *name);
                    }
                    _ => panic!("Expected Identifier expression at index {}", i),
                }
            }

            (Statement::Let(let_stmt), "let") => {
                assert_eq!(
                    let_stmt.name.value, *name,
                    "Let name mismatch at index {}",
                    i
                );

                match &let_stmt.value {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.value, *val, "Let value mismatch at index {}", i);
                    }
                    _ => panic!("Expected Identifier in let value at index {}", i),
                }
            }

            (Statement::Return(ret_stmt), "return") => match &ret_stmt.return_val {
                Expression::Identifier(ident) => {
                    assert_eq!(ident.value, *name, "Return value mismatch at index {}", i);
                }
                _ => panic!("Expected Identifier in return at index {}", i),
            },

            _ => panic!("Unexpected statement at index {}", i),
        }
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    match stmt {
        Statement::Expression(expression_statement) => match &expression_statement.expression {
            Expression::IntegerLiteral(il) => assert_eq!(il.val, 5.0),
            _ => panic!("Expeceted IntegerLiteral"),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn test_parsing_prefix_expressions() {
    let tests = [("!5", "!", 5.0), ("-15", "-", 15.0)];

    for (input, prefix, ans) in tests {
        let l = Lexer::new_lexer(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        assert!(p.check_parse_errors());

        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        // println!("\n\n{:?}\n\n", stmt);

        match stmt {
            Statement::Expression(expression_statement) => match &expression_statement.expression {
                Expression::PrefixExpression(pe) => {
                    assert_eq!(&pe.op.to_string(), prefix);

                    match pe.right.as_ref() {
                        Expression::IntegerLiteral(il) => assert_eq!(il.val, ans),
                        _ => panic!("Expected IntegerLiteral as right expression."),
                    }
                }
                _ => panic!("Expeceted IntegerLiteral"),
            },
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let tests = [
        ("5 + 5;", 5.0, "+", 5.0),
        ("5 - 5;", 5.0, "-", 5.0),
        ("5 * 5;", 5.0, "*", 5.0),
        ("5 / 5;", 5.0, "/", 5.0),
        ("5 > 5;", 5.0, ">", 5.0),
        ("5 < 5;", 5.0, "<", 5.0),
        ("5 == 5;", 5.0, "==", 5.0),
        ("5 != 5;", 5.0, "!=", 5.0),
    ];

    for (input, left_val, op, right_val) in tests {
        let l = Lexer::new_lexer(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        assert!(p.check_parse_errors());

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, got {}",
            program.statements.len()
        );

        let stmt = &program.statements[0];

        // Statement must be ExpressionStatement
        let Statement::Expression(expr_stmt) = stmt else {
            panic!("Expected ExpressionStatement");
        };

        // Expression must be InfixExpression
        let Expression::InfixExpression(infix) = &expr_stmt.expression else {
            panic!("Expected InfixExpression");
        };

        // Left side
        match infix.left.as_ref() {
            Expression::IntegerLiteral(il) => {
                assert_eq!(il.val, left_val);
            }
            _ => panic!("Expected left IntegerLiteral"),
        }

        // Operator
        assert_eq!(infix.op.to_string(), op);

        // Right side
        match infix.right.as_ref() {
            Expression::IntegerLiteral(il) => {
                assert_eq!(il.val, right_val);
            }
            _ => panic!("Expected right IntegerLiteral"),
        }
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = [
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        (
            "3 + 4; -5 * 5",
            "(Value::Double(3) + Value::Double(4));\n((-Value::Double(5)) * Value::Double(5))",
        ),
        (
            "5 > 4 == 3 < 4",
            "((Value::Double(5) > Value::Double(4)) == (Value::Double(3) < Value::Double(4)))",
        ),
        (
            "5 < 4 != 3 > 4",
            "((Value::Double(5) < Value::Double(4)) != (Value::Double(3) > Value::Double(4)))",
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((Value::Double(3) + (Value::Double(4) * Value::Double(5))) == ((Value::Double(3) * Value::Double(1)) + (Value::Double(4) * Value::Double(5))))",
        ),
        (
            "1 + (2 + 3) + 4",
            "((Value::Double(1) + (Value::Double(2) + Value::Double(3))) + Value::Double(4))",
        ),
        (
            "(5 + 5) * 2",
            "((Value::Double(5) + Value::Double(5)) * Value::Double(2))",
        ),
        (
            "2 / (5 + 5)",
            "(Value::Double(2) / (Value::Double(5) + Value::Double(5)))",
        ),
        ("-(5 + 5)", "(-(Value::Double(5) + Value::Double(5)))"),
        ("!(true == true)", "(!(Value::True == Value::True))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, Value::Double(1), (Value::Double(2) * Value::Double(3)), (Value::Double(4) + Value::Double(5)), add(Value::Double(6), (Value::Double(7) * Value::Double(8))))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new_lexer(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        assert!(p.check_parse_errors());

        assert_eq!(program.to_string(), expected);
    }
}

fn test_identifier(exp: &Expression, value: &str) {
    let ident = match exp {
        Expression::Identifier(identifier) => identifier,
        expr => panic!("Expected IdentifierExpression, found {:?}", expr),
    };

    assert_eq!(ident.value, value, "identifier value mismatch");
    assert_eq!(ident.token_literal(), value, "token literal mismatch");
}
fn test_literal_expression(exp: &Expression, expected: Value) {
    match (exp, expected) {
        (Expression::IntegerLiteral(lit), Value::Double(v)) => {
            assert_eq!(lit.val, v, "literal value mismatch");
        }
        (Expression::Identifier(ident), Value::StringLiteral(s)) => {
            assert_eq!(ident.value, s, "identifier mismatch");
        }
        (Expression::Boolean(b), Value::True) => {
            assert_eq!(b.value, Value::True);
        }
        (Expression::Boolean(b), Value::False) => {
            assert_eq!(b.value, Value::False);
        }
        (expr, val) => {
            panic!("type mismatch: got {:?} expected {:?}", expr, val);
        }
    }
}

fn test_infix_expression(exp: &Expression, left: Value, operator: &str, right: Value) {
    let op_exp = match exp {
        Expression::InfixExpression(infix) => infix,
        _ => panic!("exp is not InfixExpression"),
    };

    test_literal_expression(&op_exp.left, left);
    assert_eq!(op_exp.op.to_string(), operator, "operator mismatch");
    test_literal_expression(&op_exp.right, right);
}
#[test]
fn test_parsing_infix_expressions_2() {
    let tests = [
        ("5 + 5;", Value::Double(5.0), "+", Value::Double(5.0)),
        ("5 - 5;", Value::Double(5.0), "-", Value::Double(5.0)),
        ("5 * 5;", Value::Double(5.0), "*", Value::Double(5.0)),
        ("5 / 5;", Value::Double(5.0), "/", Value::Double(5.0)),
        ("5 > 5;", Value::Double(5.0), ">", Value::Double(5.0)),
        ("5 < 5;", Value::Double(5.0), "<", Value::Double(5.0)),
        ("5 == 5;", Value::Double(5.0), "==", Value::Double(5.0)),
        ("5 != 5;", Value::Double(5.0), "!=", Value::Double(5.0)),
    ];

    for (input, left, operator, right) in tests {
        let lexer = Lexer::new_lexer(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(parser.check_parse_errors());
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        let expr = match stmt {
            Statement::Expression(es) => &es.expression,
            _ => panic!("stmt is not ExpressionStatement"),
        };

        test_infix_expression(expr, left, operator, right);
    }
}
#[test]
fn test_identifier_expression() {
    let input = "foobar;";
    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());

    assert_eq!(program.statements.len(), 1);

    let stmt = &program.statements[0];

    let expr = match stmt {
        Statement::Expression(expr) => &expr.expression,
        _ => panic!("stmt is not Expression"),
    };

    test_identifier(expr, "foobar");
}
fn test_boolean_literal(exp: &Expression, value: Value) {
    let bo = match exp {
        Expression::Boolean(b) => b,
        _ => panic!("exp is not Boolean. got {:?}", exp),
    };

    assert_eq!(bo.value, value, "boolean value mismatch");

    assert_eq!(
        bo.token_literal(),
        value.to_string(),
        "token literal mismatch"
    );
}
#[test]
fn test_boolean_expression() {
    let tests = [("true;", Value::True), ("false;", Value::False)];

    for (input, expected) in tests {
        let lexer = Lexer::new_lexer(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.check_parse_errors());
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];

        let expr = match stmt {
            Statement::Expression(es) => &es.expression,
            _ => panic!("stmt is not Expression"),
        };

        test_boolean_literal(expr, expected);
    }
}
#[test]
fn test_parsing_infix_expressions_with_booleans() {
    let tests = [
        ("true == true", Value::True, "==", Value::True),
        ("true != false", Value::True, "!=", Value::False),
        ("false == false", Value::False, "==", Value::False),
    ];

    for (input, left, operator, right) in tests {
        let lexer = Lexer::new_lexer(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.check_parse_errors());
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];

        let expr = match stmt {
            Statement::Expression(es) => &es.expression,
            _ => panic!("stmt is not Expression"),
        };

        test_infix_expression(expr, left, operator, right);
    }
}
#[test]
fn test_parsing_prefix_expressions_with_booleans() {
    let tests = [("!true;", "!", Value::True), ("!false;", "!", Value::False)];

    for (input, operator, value) in tests {
        let lexer = Lexer::new_lexer(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(parser.check_parse_errors());
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];

        let expr = match stmt {
            Statement::Expression(es) => &es.expression,
            _ => panic!("stmt is not Expression"),
        };

        let prefix = match expr {
            Expression::PrefixExpression(p) => p,
            _ => panic!("expr is not PrefixExpression"),
        };

        assert_eq!(prefix.op.to_string(), operator);

        test_literal_expression(&prefix.right, value);
    }
}

#[test]
fn test_if_expression() {
    let input = "if (x<y) {x}";

    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());

    assert_eq!(program.statements.len(), 1);
    let stmt = match &program.statements[0] {
        Statement::Expression(es) => es,
        stmt => panic!("Expected ExpressionStatement, found {}", stmt.string()),
    };
    let ie = match &stmt.expression {
        Expression::IfExpression(ie) => ie,
        expr => panic!("Expected IfExpression, found {}", expr.string()),
    };

    let condition = &ie.condition;

    test_infix_expression(
        condition,
        Value::StringLiteral("x".to_string()),
        "<",
        Value::StringLiteral("y".to_string()),
    );

    assert_eq!(ie.consequence.statements.len(), 1);
    let consequence = match &ie.consequence.statements[0] {
        Statement::Expression(es) => es,
        _ => panic!("Expected ExpressionStatement in consequence"),
    };

    test_identifier(&consequence.expression, "x");
}

#[test]
fn test_function_literal_parsing() {
    let input = "fn(x, y){x+y;}";
    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());

    assert_eq!(program.statements.iter().len(), 1);
    let es = match &program.statements[0] {
        Statement::Expression(es) => es,
        s => panic!("Expected Expression, found {}", s.string()),
    };
    let fl = match &es.expression {
        Expression::FunctionLiteral(function_literal) => function_literal,
        e => panic!("Expected FunctionLiteral, found {}", e.string()),
    };

    assert_eq!(fl.args.len(), 2);
    test_identifier(&Expression::Identifier(fl.args[0].clone()), "x");
    test_identifier(&Expression::Identifier(fl.args[1].clone()), "y");
    assert_eq!(fl.body.statements.len(), 1);

    let stmt = match &fl.body.statements[0] {
        Statement::Expression(es) => es,
        s => panic!(
            "Expected Expression in functions body, found {}",
            s.string()
        ),
    };

    test_infix_expression(
        &stmt.expression,
        Value::StringLiteral("x".to_string()),
        "+",
        Value::StringLiteral("y".to_string()),
    );
}
#[test]
fn test_function_parameter_parsing() {
    let tests = [
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected_params) in tests {
        let l = Lexer::new_lexer(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        assert!(p.check_parse_errors());

        assert_eq!(program.statements.len(), 1);

        let stmt = match &program.statements[0] {
            Statement::Expression(es) => es,
            _ => panic!("stmt is not ExpressionStatement"),
        };

        let function = match &stmt.expression {
            Expression::FunctionLiteral(fl) => fl,
            _ => panic!("expression is not FunctionLiteral"),
        };

        assert_eq!(
            function.args.len(),
            expected_params.len(),
            "wrong number of params"
        );

        for (i, ident) in expected_params.iter().enumerate() {
            test_identifier(&Expression::Identifier(function.args[i].clone()), ident);
        }
    }
}

#[test]
fn test_function_call_expression_parsing() {
    let input = "add(1, 2*3, 4+5);";
    let l = Lexer::new_lexer(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert!(p.check_parse_errors());

    assert_eq!(program.statements.len(), 1);

    let stmt = match &program.statements[0] {
        Statement::Expression(es) => es,
        _ => panic!("stmt is not ExpressionStatement"),
    };
    let ce = match &stmt.expression {
        Expression::CallExpression(call_expression) => call_expression,
        e => panic!("Expected CallExpression, found {}", e.string()),
    };

    test_identifier(&ce.function, "add");
    assert_eq!(ce.args.len(), 3);
    test_literal_expression(&ce.args[0], Value::Double(1f64));
    test_infix_expression(&ce.args[1], Value::Double(2f64), "*", Value::Double(3f64));
    test_infix_expression(&ce.args[2], Value::Double(4f64), "+", Value::Double(5f64));
}
#[test]
fn test_let_statements() {
    struct Test<'a> {
        input: &'a str,
        expected_identifier: &'a str,
        expected_value: Value,
    }

    let tests = [
        Test {
            input: "let x = 5;",
            expected_identifier: "x",
            expected_value: Value::Double(5.0),
        },
        Test {
            input: "let y = true;",
            expected_identifier: "y",
            expected_value: Value::True,
        },
        Test {
            input: "let foobar = y;",
            expected_identifier: "foobar",
            expected_value: Value::StringLiteral("y".to_string()),
        },
    ];

    for tt in tests {
        let l = Lexer::new_lexer(tt.input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert!(p.check_parse_errors());

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement"
        );

        let stmt = &program.statements[0];

        test_let_statement(stmt, tt.expected_identifier);

        let val = match stmt {
            Statement::Let(ls) => &ls.value,
            _ => panic!("stmt is not LetStatement"),
        };

        match &tt.expected_value {
            Value::StringLiteral(s) => {
                test_identifier(val, s);
            }
            _ => {
                test_literal_expression(val, tt.expected_value.clone());
            }
        }
    }
}
