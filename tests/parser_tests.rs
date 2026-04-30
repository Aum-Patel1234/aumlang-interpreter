use aumlang::{
    lexer::Lexer,
    parser::{
        // Node,
        ast::{Expression, Statement},
        parser_logic::Parser,
    },
};

// #[test]
// fn test_let_parser() {
//     let input = r#"
//         let x = 5;
//         let y = 10;
//         let foobar = 838383;
//     "#;
//
//     let l = Lexer::new_lexer(input);
//     let mut p = Parser::new(l);
//
//     let program = p.parse_program();
//     p.check_parse_errors();
//     let tests = ["x", "y", "foobar"];
//
//     assert_eq!(
//         program.statements.len(),
//         tests.len(),
//         "Mismatch: number of parsed statements does not match number of tests"
//     );
//     for (i, test) in tests.iter().enumerate() {
//         let stmt = program.statements.get(i);
//         if let Some(s) = stmt
//             && !test_let_statement(s, test)
//         {
//             return;
//         }
//     }
// }
// #[test]
// fn test_parser_error() {
//     let input = r#"
//         let foobar = 838383;
//         let x  5;
//         let y - 10;
//     "#;
//
//     let l = Lexer::new_lexer(input);
//     let mut p = Parser::new(l);
//
//     let program = p.parse_program();
//     assert!(
//         !p.get_errors().is_empty(),
//         "Expected parser errors but got none"
//     );
//
//     // let tests = ["x", "y", "foobar"];
//     assert_eq!(
//         program.statements.len(),
//         1,
//         "Expected 1 statement, got {}",
//         program.statements.len()
//     );
//     // for (i, test) in tests.iter().enumerate() {
//     //     let stmt = program.statements.get(i);
//     //     if let Some(s) = stmt
//     //         && !test_let_statement(s, test)
//     //     {
//     //         return;
//     //     }
//     // }
// }

// fn test_let_statement(s: &Statement, name: &str) -> bool {
//     if s.token_literal() != "let" {
//         return false;
//     }
//
//     let lstmt = match s {
//         Statement::Let(stmt) => stmt,
//         _ => return false,
//     };
//     if lstmt.name.value != name {
//         return false;
//     }
//
//     if lstmt.name.token_literal() != name {
//         return false;
//     }
//
//     true
// }

// #[test]
// fn test_return_statements() {
//     let input = r#"
//         return 5;
//         return 10;
//         return 993322;
//     "#;
//
//     let l = Lexer::new_lexer(input);
//     let mut p = Parser::new(l);
//
//     let program = p.parse_program();
//     assert!(p.check_parse_errors());
//
//     assert_eq!(
//         program.statements.len(),
//         3,
//         "program.statements does not contain 3 statements. got={}",
//         program.statements.len()
//     );
//
//     for stmt in program.statements.iter() {
//         match stmt {
//             Statement::Return(return_stmt) => {
//                 assert_eq!(
//                     return_stmt.token_literal(),
//                     "return",
//                     "returnStmt.token_literal not 'return'"
//                 );
//             }
//             _ => {
//                 panic!("stmt not ReturnStatement. got={:?}", stmt);
//             }
//         }
//     }
// }

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

                // let ident = match expr {
                //     Expression::Identifier(ident) => ident,
                // };
                let Expression::Identifier(ident) = expr;
                assert_eq!(ident.value, *name);
            }

            (Statement::Let(let_stmt), "let") => {
                // check variable name
                assert_eq!(
                    let_stmt.name.value, *name,
                    "Let name mismatch at index {}",
                    i
                );

                // check assigned value
                let Expression::Identifier(ident) = &let_stmt.value;

                assert_eq!(ident.value, *val, "Let value mismatch at index {}", i);
            }

            (Statement::Return(ret_stmt), "return") => {
                let Expression::Identifier(ident) = &ret_stmt.return_val;
                assert_eq!(ident.value, *name, "Return value mismatch at index {}", i);
            }

            _ => panic!("Unexpected statement at index {}", i),
        }
    }
}
