use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::{Parser, ParserError, ParserResult};

///
/// Literal
///  : BooleanLiteral
///  : NilLiteral
///  : NumericLiteral
///  | StringLiteral
///  ;
///
pub(super) fn parse_literal_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    match parser.lookahead.token_type {
        TokenType::BooleanTrue => parse_boolean_literal_expression(parser),
        TokenType::BooleanFalse => parse_boolean_literal_expression(parser),
        TokenType::Nil => parse_nil_literal_expression(parser),
        TokenType::Number => parse_numeric_literal_expression(parser),
        TokenType::String => parse_string_literal_expression(parser),
        _ => Err(ParserError::SemanticError {
            message: String::from("Literal: unexpected literal production"),
        }),
    }
}

///
/// BooleanLiteral
///  : BOOLEAN
///  ;
///
pub(super) fn parse_boolean_literal_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    let token = parser.eat_any_of_token(&[TokenType::BooleanTrue, TokenType::BooleanFalse])?;
    let bool_value = token.token_type == TokenType::BooleanTrue;

    Ok(Box::new(Expression::BooleanLiteral { value: bool_value }))
}

///
/// NilLiteral
///  : NIL
///  ;
///
pub(super) fn parse_nil_literal_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parser.eat_token(TokenType::Nil)?;

    Ok(Box::new(Expression::NilLiteral))
}

///
/// NumericLiteral
///  : NUMBER
///  ;
///
pub(super) fn parse_numeric_literal_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    let token = parser.eat_token(TokenType::Number)?;
    let token_value = token.text(parser.source);
    let token_value = token_value.trim().parse().unwrap();

    Ok(Box::new(Expression::NumericLiteral { value: token_value }))
}

///
/// StringLiteral
///  : STRING
///  ;
///
pub(super) fn parse_string_literal_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    let token = parser.eat_token(TokenType::String)?;
    let token_value = &parser.source[token.start.offset + 1..token.end.offset - 1];

    Ok(Box::new(Expression::StringLiteral {
        value: String::from(token_value),
    }))
}
