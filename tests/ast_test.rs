use aumlang::{
    parser::{
        Node,
        ast::{Expression, Identifier, LetStatement, Program, Statement},
    },
    token::Keyword,
};

#[test]
fn test_string() {
    let program = Program::new(vec![Statement::Let(LetStatement::new(
        Keyword::LET,
        Identifier {
            value: "myVar".to_string(),
        },
        Expression::Identifier(Identifier {
            value: "anotherVar".to_string(),
        }),
    ))]);

    let result = program.string();

    assert_eq!(result, "let myVar = anotherVar;\n");
}
