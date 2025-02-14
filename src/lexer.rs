use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let source = input.chars().collect::<Vec<_>>();
        let current_char = source.get(0).copied();
        Self { 
            source, 
            position: 0, 
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.source.get(self.position).copied();
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }

    pub fn next_token(&mut self) -> Token {
        // skip whitespace
        while self.current_char.map_or(false, |c: char| c.is_whitespace()) {
            self.advance();
        }

        let start = self.position;
        let token = match self.current_char {
            Some('(') => Token { token_type: TokenType::OpenParen, start, end: start + 1, raw: "(".to_string() },
            Some(')') => Token { token_type: TokenType::CloseParen, start, end: start + 1, raw: ")".to_string() },
            Some('{') => Token { token_type: TokenType::OpenBrace, start, end: start + 1, raw: "{".to_string() },
            Some('}') => Token { token_type: TokenType::CloseBrace, start, end: start + 1, raw: "}".to_string() },
            Some('[') => Token { token_type: TokenType::OpenSquare, start, end: start + 1, raw: "[".to_string() },
            Some(']') => Token { token_type: TokenType::CloseSquare, start, end: start + 1, raw: "]".to_string() },
            Some('?') => Token { token_type: TokenType::Optional, start, end: start + 1, raw: "?".to_string() },
            Some('!') => Token { token_type: TokenType::Final, start, end: start + 1, raw: "!".to_string() },
            None => Token { token_type: TokenType::EOF, start, end: start, raw: "".to_string() },
            _ => panic!("Unexpected Token: {}", self.current_char.unwrap()),
        };
        self.advance();
        token
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn should_tokenize_symbols() {
        let input = "{} [] () ! ?";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token { token_type: TokenType::OpenBrace, start: 0, end: 1, raw: "{".to_string() },
            Token { token_type: TokenType::CloseBrace, start: 1, end: 2, raw: "}".to_string() },
            Token { token_type: TokenType::OpenSquare, start: 3, end: 4, raw: "[".to_string() },
            Token { token_type: TokenType::CloseSquare, start: 4, end: 5, raw: "]".to_string() },
            Token { token_type: TokenType::OpenParen, start: 6, end: 7, raw: "(".to_string() },
            Token { token_type: TokenType::CloseParen, start: 7, end: 8, raw: ")".to_string() },
            Token { token_type: TokenType::Final, start: 9, end: 10, raw: "!".to_string() },
            Token { token_type: TokenType::Optional, start: 11, end: 12, raw: "?".to_string() },
            Token { token_type: TokenType::EOF, start: 12, end: 12, raw: "".to_string() },
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.start, expected_token.start);
            assert_eq!(token.end, expected_token.end);
            assert_eq!(token.raw, expected_token.raw);
        }
    }
}