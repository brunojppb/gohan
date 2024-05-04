use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        return Self {
            input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EndOfFile);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let Some(c) = self.advance() else {
            panic!("Could not scan the next token. Line {}", &self.line);
        };

        match c {
            '#' => self.add_token(TokenType::Hash),
            '*' => self.add_token(TokenType::Star),
            '_' => self.add_token(TokenType::Underscore),
            '\t' => self.add_token(TokenType::Tab),
            '\n' => {
                self.line = self.line + 1;
                self.add_token(TokenType::Newline);
            }
            _ => self.handle_string(),
        }
    }

    fn is_string(&self, c: Option<char>) -> bool {
        dbg!("char={}", c);
        c.filter(|c| c.is_digit(10) || c == &'.' || c == &' ')
            .is_some()
    }

    fn handle_string(&mut self) {
        while self.is_string(self.peek()) {
            dbg!("loop");
            self.advance();
        }

        dbg!("start={} current={}", self.start, self.current);

        let value: String = self
            .input
            .chars()
            .skip(self.start)
            .take(self.current - 1)
            .collect();

        self.add_token(TokenType::Text(value));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() == Some(expected) {
            self.advance();
            return true;
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token::new(self.start, token_type, self.line);

        self.tokens.push(token);
    }

    // Look-up one character after the next, but do not consume it
    fn peek_next(&self) -> Option<char> {
        self.input.chars().nth(self.current + 1)
    }

    /// Look-up the next character, but do not consume it
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.input.chars().nth(self.current)
    }

    /// Consume the next character and advance the needle
    /// to point to a potential next character
    fn advance(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.current);
        self.current = self.current + 1;
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_headers() {
        let mut lexer = Lexer::new("### hi there!");
        let tokens = lexer.scan();
        println!("Tokens: {:#?}", tokens);
        assert_eq!(tokens.len(), 3);
    }
}
