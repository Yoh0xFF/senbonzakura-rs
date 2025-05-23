use crate::{
    ast::{ExpressionRef, Statement, StatementRef},
    lexer::TokenType,
    parser::parsers::{
        expression_parse_primary::parse_identifier_expression,
        statement_parse_block::parse_block_statement,
    },
    Parser,
};

///
/// ClassDeclaration
///  : class IdentifierExpression [ClassExtendsExpression] BlockStatement
///  ;
///
pub(super) fn parse_class_declaration(parser: &mut Parser) -> StatementRef {
    parser.eat_token(TokenType::ClassKeyword);

    let name = parse_identifier_expression(parser);

    let super_class = if parser.is_next_token_of_type(TokenType::ExtendsKeyword) {
        Some(parse_class_extends_expression(parser))
    } else {
        None
    };

    let body = parse_block_statement(parser);

    Box::new(Statement::ClassDeclaration {
        name,
        super_class,
        body,
    })
}

///
/// ClassExtendsExpression
///  : extends IdentifierExpression
///  ;
///
fn parse_class_extends_expression(parser: &mut Parser) -> ExpressionRef {
    parser.eat_token(TokenType::ExtendsKeyword);
    parse_identifier_expression(parser)
}
