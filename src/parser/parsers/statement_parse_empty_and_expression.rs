use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::parse_root_expression;
use crate::parser::Parser;

///
/// EmptyStatement
///  : ';'
///  ;
///
pub(super) fn parse_empty_statement(parser: &mut Parser) -> StatementRef {
    parser.eat_token(TokenType::StatementEnd);

    Box::new(Statement::Empty)
}

///
/// ExpressionStatement
///  : Expression ';'
///  ;
///
pub(super) fn parse_expression_statement(
    parser: &mut Parser,
    consume_statement_end: bool,
) -> StatementRef {
    let expression = parse_root_expression(parser);

    if consume_statement_end {
        parser.eat_token(TokenType::StatementEnd);
    }

    Box::new(Statement::Expression { expression })
}
