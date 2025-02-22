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
            Some(c) if c.is_alphabetic() => self.read_identifier(),
            Some('@') if self.peek().unwrap().is_alphabetic() => self.read_attribute(),
            Some('"') => self.read_string(),
            Some(c) if c.is_digit(10) => self.read_number(),
            // If the char is a negative sign and the next char is a number 
            Some('-') if self.peek().map_or(false, |c: char| c.is_digit(10))  => self.read_number(),
            None => Token { token_type: TokenType::EOF, start, end: start, raw: "".to_string() },
            _ => panic!("Unexpected Token: {}", self.current_char.unwrap()),
        };
        self.advance();
        token
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        let mut raw = String::new();

        while self.current_char.map_or(false, |c: char| c.is_alphanumeric()){
            raw.push(self.current_char.unwrap());
            self.advance();
        }

        let token_type = match raw.as_str() {
            "plugin" => TokenType::Plugin,
            "prop" => TokenType::Prop,
            "enum" => TokenType::Enum,
            "model" => TokenType::Model,
            "type" => TokenType::Type,
            "use" => TokenType::Use,
            _ => TokenType::Identifier,
        };

        Token { token_type, start, end: self.position, raw }
    }

    fn read_attribute(&mut self) -> Token {
        let start = self.position;
        // Skips '@' symbol
        self.advance();
        let mut raw = String::new();

        while self.current_char.map_or(false, |c: char| c.is_alphabetic() || c == '.') {
            raw.push(self.current_char.unwrap());
            self.advance();
        }

        Token { token_type: TokenType::AttributeIdentifier, start, end: self.position, raw }
    }

    fn read_string(&mut self) -> Token {
        let start = self.position;
        self.advance();

        let mut raw = String::new();
        while let Some(c) = self.current_char {
            if c == '"' { break; }
            raw.push(c);
            self.advance();
        }

        Token { token_type: TokenType::StringLiteral, start, end: self.position, raw }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        let mut raw = String::new();

        if self.current_char == Some('-') {
            raw.push(self.current_char.unwrap());
            self.advance();
        }

        while self.current_char.map_or(false, |c: char| c.is_digit(10)) {
            raw.push(self.current_char.unwrap());
            self.advance();
        }

        if self.current_char == Some('.') {
            raw.push(self.current_char.unwrap());
            self.advance();

            while self.current_char.map_or(false, |c: char| c.is_digit(10)) {
                raw.push(self.current_char.unwrap());
                self.advance();
            }
            return Token {token_type: TokenType::FloatLiteral, start, end: self.position, raw}
        }

        Token { token_type: TokenType::IntegerLiteral, start, end: self.position, raw }
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

    #[test]
    fn should_tokenize_empty_input() {
        let mut lexer = Lexer::new("");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::EOF);
    }

    #[test]
    fn should_tokenize_lower_identifier() {
        let mut lexer = Lexer::new("somevariable");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "somevariable");
    }

    #[test]
    fn should_tokenize_camel_case_identifier() {
        let mut lexer = Lexer::new("someVariable");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "someVariable");
    }

    #[test]
    fn should_tokenize_pascal_case_identifier() {
        let mut lexer = Lexer::new("SomeVariable");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "SomeVariable");
    }

    #[test]
    fn should_tokenize_uppercase_identifier() {
        let mut lexer = Lexer::new("SOMEVARIABLE");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.raw, "SOMEVARIABLE");
    }

    #[test]
    fn should_tokenize_attribute() {
        let mut lexer = Lexer::new("@field.text");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::AttributeIdentifier);
        assert_eq!(token.raw, "field.text");
    }

    #[test]
    fn should_tokenize_string_literals() {
        let mut lexer = Lexer::new("\"Hello\"");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.raw, "Hello");
    }

    #[test]
    fn should_tokenize_integers() {
        let mut lexer = Lexer::new("12345");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::IntegerLiteral);
        assert_eq!(token.raw, "12345");
    }

    #[test]
    fn should_tokenize_negative_integers() {
        let mut lexer = Lexer::new("-12345");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::IntegerLiteral);
        assert_eq!(token.raw, "-12345");
    }

    #[test]
    fn should_tokenize_floats() {
        let mut lexer = Lexer::new("123.45");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::FloatLiteral);
        assert_eq!(token.raw, "123.45");
    }

    #[test]
    fn should_tokenize_negative_floats() {
        let mut lexer = Lexer::new("-123.45");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::FloatLiteral);
        assert_eq!(token.raw, "-123.45");
    }

    #[test]
    fn should_tokenize_plugin() {
        let mut lexer = Lexer::new("plugin");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Plugin);
        assert_eq!(token.raw, "plugin");
    }

    #[test]
    fn should_tokenize_prop() {
        let mut lexer = Lexer::new("prop");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Prop);
        assert_eq!(token.raw, "prop");
    }

    #[test]
    fn should_tokenize_use() {
        let mut lexer = Lexer::new("use");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Use);
        assert_eq!(token.raw, "use");
    }

    #[test]
    fn should_tokenize_type() {
        let mut lexer = Lexer::new("type");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Type);
        assert_eq!(token.raw, "type");
    }

    #[test]
    fn should_tokenize_model() {
        let mut lexer = Lexer::new("model");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Model);
        assert_eq!(token.raw, "model");
    }

    #[test]
    fn should_tokenize_enum() {
        let mut lexer = Lexer::new("enum");
        let token = lexer.next_token();

        assert_eq!(token.token_type, TokenType::Enum);
        assert_eq!(token.raw, "enum");
    }
}