use std::rc::Rc;

use crate::{ast::Expression, Statement};

use super::parser_tests::execute;

#[test]
fn test_string_literal() {
    execute(
        r#""Hello";"#,
        Rc::new(Statement::Program {
            body: Rc::new(vec![Rc::new(Statement::Expression {
                expression: Rc::new(Expression::StringLiteral("Hello".to_string())),
            })]),
        }),
    )
}

#[test]
fn test_number_literal() {
    execute(
        "12;",
        Rc::new(Statement::Program {
            body: Rc::new(vec![Rc::new(Statement::Expression {
                expression: Rc::new(Expression::NumericLiteral(12)),
            })]),
        }),
    )
}

#[test]
fn test_multiple_literal() {
    execute(
        r#"
        "Hello";

        17;
        "#,
        Rc::new(Statement::Program {
            body: Rc::new(vec![
                Rc::new(Statement::Expression {
                    expression: Rc::new(Expression::StringLiteral("Hello".to_string())),
                }),
                Rc::new(Statement::Expression {
                    expression: Rc::new(Expression::NumericLiteral(17)),
                }),
            ]),
        }),
    )
}
