use super::statement_parse_class_declaration::parse_class_declaration;
use crate::ast::{Statement, StatementList, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::statement_parse_conditional::parse_if_statement;
use crate::parser::parsers::statement_parse_empty_and_expression::{
    parse_empty_statement, parse_expression_statement,
};
use crate::parser::parsers::statement_parse_function_declaration::{
    parse_function_declaration_statement, parse_return_statement,
};
use crate::parser::parsers::statement_parse_loop::{
    parse_do_while_statement, parse_for_statement, parse_while_statement,
};
use crate::parser::parsers::statement_parse_variable_declaration::parse_variable_declaration_statement;
use crate::parser::{Parser, ParserResult};

///
/// Main entry point
/// Program
///  : StatementList
///  ;
///
pub(super) fn parse_program_statement(parser: &mut Parser) -> ParserResult<StatementRef> {
    let statement_list = parse_statement_list(parser, None)?;
    Ok(Box::new(Statement::Program {
        body: statement_list,
    }))
}

///
/// BlockStatement
///  : '{' OptStatementList '}'
///  ;
///
pub(super) fn parse_block_statement(parser: &mut Parser) -> ParserResult<StatementRef> {
    parser.eat_token(TokenType::OpeningBrace)?;

    let block = if !parser.is_next_token_of_type(TokenType::ClosingBrace) {
        parse_statement_list(parser, Some(TokenType::ClosingBrace))?
    } else {
        vec![]
    };

    parser.eat_token(TokenType::ClosingBrace)?;

    Ok(Box::new(Statement::Block { body: block }))
}

///
/// StatementList
///  : Statement
///  | StatementList Statement
///  ;
///
pub(super) fn parse_statement_list(
    parser: &mut Parser,
    stop_token_type: Option<TokenType>,
) -> ParserResult<StatementList> {
    let mut statement_list: Vec<Statement> = vec![];

    while !parser.is_next_token_of_type(TokenType::End)
        && !parser.is_next_token_of_type(stop_token_type.unwrap_or(TokenType::End))
    {
        let statement = parse_statement(parser)?;
        statement_list.push(*statement);
    }

    Ok(statement_list)
}

///
/// Statement
///  : ExpressionStatement
///  | BlockStatement
///  | EmptyStatement
///  | VariableStatement
///  | ConditionalStatement
///  | IterationStatement
///  | FunctionDeclarationStatement
///  | ReturnStatement
///  | ClassDeclaration
///  ;
///
pub(super) fn parse_statement(parser: &mut Parser) -> ParserResult<StatementRef> {
    match parser.lookahead.token_type {
        TokenType::StatementEnd => parse_empty_statement(parser),
        TokenType::OpeningBrace => parse_block_statement(parser),
        TokenType::LetKeyword => parse_variable_declaration_statement(parser, true),
        TokenType::IfKeyword => parse_if_statement(parser),
        TokenType::WhileKeyword => parse_while_statement(parser),
        TokenType::DoKeyword => parse_do_while_statement(parser),
        TokenType::ForKeyword => parse_for_statement(parser),
        TokenType::DefKeyword => parse_function_declaration_statement(parser),
        TokenType::ReturnKeyword => parse_return_statement(parser),
        TokenType::ClassKeyword => parse_class_declaration(parser),
        _ => parse_expression_statement(parser, true),
    }
}
