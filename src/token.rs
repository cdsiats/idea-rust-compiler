#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    Plugin,
    Prop,
    Use,
    Type,
    Model,

    // Identifiers
    Identifier(String),

    // Literals
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),

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