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

    // let lstmt = match s {
    //     Statement::Let(stmt) => stmt,
    //     // _ => return false,
    // };
    let Statement::Let(lstmt) = s;
    if lstmt.name.value != name {
        return false;
    }

    if lstmt.name.token_literal() != name {
        return false;
    }

    true
}
