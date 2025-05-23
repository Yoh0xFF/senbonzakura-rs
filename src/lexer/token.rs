use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TokenType {
    // Whitespace and comments
    Whitespace,
    SingleLineComment,
    MultiLineComment,

    // Symbols
    StatementEnd,
    OpeningBrace,
    ClosingBrace,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    Comma,
    Dot,
    Colon,

    // Keywords,
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    DoKeyword,
    ForKeyword,
    DefKeyword,
    ReturnKeyword,
    ClassKeyword,
    ExtendsKeyword,
    ThisKeyword,
    SuperKeyword,
    NewKeyword,
    TypeKeyword,
    NumberTypeKeyword,
    StringTypeKeyword,
    BooleanTypeKeyword,
    VoidTypeKeyword,

    // Identifier
    Identifier,

    // Equality operators
    EqualityOperator,

    // Assignment operators
    SimpleAssignmentOperator,
    ComplexAssignmentOperator,

    // Math operators
    AdditiveOperator,
    FactorOperator,

    // Relational operators
    RelationalOperator,

    // Logical operators
    LogicalAndOperator,
    LogicalOrOperator,
    LogicalNotOperator,

    // Literals
    Boolean,
    Nil,
    Number,
    String,

    // End
    End,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub i: usize,
    pub j: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token ({}, {}, {})",
            self.token_type.to_string(),
            self.i,
            self.j,
        )
    }
}
