use aumlang::{
    lexer::Lexer,
    parser::{Node, ast::Statement, parser_logic::Parser},
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

    assert!(program.statements.len() == tests.len());
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
    p.check_parse_errors();
    let tests = ["x", "y", "foobar"];

    assert!(program.statements.len() == 1);
    for (i, test) in tests.iter().enumerate() {
        let stmt = program.statements.get(i);
        if let Some(s) = stmt
            && !test_let_statement(s, test)
        {
            return;
        }
    }
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
