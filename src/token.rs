#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    Plugin,
    Prop,
    Use,
    Type,
    Model,
    Enum,

    // Identifiers
    Identifier,
    AttributeIdentifier,

    // Literals
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,

    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenSquare,
    CloseSquare,
    // '!' symbol
    Final,
    // '?' symbol
    Optional,

    EOF,    
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub raw: String,
}