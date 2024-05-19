use crate::token::{Span, Token};

const SYMBOLS: &str = "#*!_[]().- \n\t\\";

/// Tokenizes Markdown input
pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<(Token<'a>, Span)>,
    start: usize,
    current: usize,
    col: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            col: 0,
        }
    }

    pub fn scan(&mut self) -> &Vec<(Token<'a>, Span)> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(Token::EndOfFile);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let Some(c) = self.advance() else {
            panic!("Could not scan the next token. Line {}", &self.line);
        };

        match c {
            '#' => self.add_token(Token::Hash),
            '*' => self.add_token(Token::Star),
            '!' => self.add_token(Token::Bang),
            ' ' => self.add_token(Token::Space),
            '_' => self.add_token(Token::Underscore),
            '-' => self.add_token(Token::Dash),
            '.' => self.add_token(Token::Dot),
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '[' => self.add_token(Token::LeftSquareBracket),
            ']' => self.add_token(Token::RightSquareBracket),
            '\\' => self.add_token(Token::Backslash),
            '\t' => self.add_token(Token::Tab),
            '\n' => {
                self.line += 1;
                self.col = 0;
                self.add_token(Token::Newline);
            }
            c if c.is_ascii_digit() => {
                self.add_token(Token::Digit(&self.source[self.current - 1..self.current]))
            }
            _ => self.handle_string(),
        }
    }

    fn is_token(&self, c: Option<char>) -> bool {
        c.filter(|c| c.is_ascii_digit() || SYMBOLS.contains(*c))
            .is_some()
    }

    fn handle_string(&mut self) {
        while !self.is_at_end() && !self.is_token(self.peek()) {
            self.advance();
        }

        let sub_str_offset = self.start + (self.current - self.start);
        let value = &self.source[self.start..sub_str_offset];

        self.add_token(Token::Text(value));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token: Token<'a>) {
        // let token = Token::new(self.start, token_type, self.line);
        let span = Span {
            line: self.line,
            col: self.col,
        };
        self.tokens.push((token, span));
    }

    /// Look-up the next character, but do not consume it
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.source.chars().nth(self.current)
    }

    /// Consume the next character and advance the needle
    /// to point to a potential next character
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        self.col += 1;
        c
    }

    // Look-up one character after the next, but do not consume it
    // fn peek_next(&self) -> Option<char> {
    //     self.input.chars().nth(self.current + 1)
    // }
    //
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn tokenize_markdown() {
        insta::glob!("snapshot_inputs/*.md", |path| {
            let markdown = fs::read_to_string(path).unwrap();
            let mut lexer = Lexer::new(&markdown);
            insta::assert_json_snapshot!(lexer.scan());
        });
    }
}
