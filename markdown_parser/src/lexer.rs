use crate::token::{Token, TokenType};

const SPECIAL_TOKENS: &str = "#*!_[]().- \n\t\\";

pub struct Lexer<'a> {
    input: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
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
            '!' => self.add_token(TokenType::Bang),
            ' ' => self.add_token(TokenType::Space),
            '_' => self.add_token(TokenType::Underscore),
            '-' => self.add_token(TokenType::Dash),
            '.' => self.add_token(TokenType::Dot),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftSquareBraket),
            ']' => self.add_token(TokenType::RightSquareBraket),
            '\\' => self.add_token(TokenType::Backslash),
            '\t' => self.add_token(TokenType::Tab),
            '\n' => {
                self.line += 1;
                self.add_token(TokenType::Newline);
            }
            c if c.is_ascii_digit() => {
                self.add_token(TokenType::Number(c.to_digit(10).unwrap() as usize))
            }
            _ => self.handle_string(),
        }
    }

    fn is_token(&self, c: Option<char>) -> bool {
        let r = c
            .filter(|c| c.is_ascii_digit() || SPECIAL_TOKENS.contains(*c))
            .is_some();
        println!("c={:?} r={}", c, r);
        r
    }

    fn handle_string(&mut self) {
        while !self.is_at_end() && !self.is_token(self.peek()) {
            self.advance();
        }

        let value: String = self
            .input
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        println!(
            "start={} current={} value='{}'",
            self.start, self.current, value
        );

        self.add_token(TokenType::Text(value));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token::new(self.start, token_type, self.line);

        self.tokens.push(token);
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
        self.current += 1;
        c
    }

    // Look-up one character after the next, but do not consume it
    // fn peek_next(&self) -> Option<char> {
    //     self.input.chars().nth(self.current + 1)
    // }
    //
    // fn match_next(&mut self, expected: char) -> bool {
    //     if self.is_at_end() {
    //         return false;
    //     }

    //     if self.peek() == Some(expected) {
    //         self.advance();
    //         return true;
    //     }

    //     false
    // }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_headers() {
        insta::glob!("snapshot_inputs/*.md", |path| {
            let markdown = fs::read_to_string(path).unwrap();
            let mut lexer = Lexer::new(&markdown);
            insta::assert_json_snapshot!(lexer.scan());
        });
    }
}
