use crate::token::{Span, Token};

const SYMBOLS: &str = "#*!_[]().- \n\t\\";

/// Tokenizes Markdown input
pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<(Token<'a>, Span)>,
    start_byte_offset: usize,
    current_byte_offset: usize,
    col: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            tokens: Vec::new(),
            start_byte_offset: 0,
            current_byte_offset: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn scan(&mut self) -> &Vec<(Token<'a>, Span)> {
        while !self.is_at_end() {
            self.start_byte_offset = self.current_byte_offset;
            self.scan_token();
        }

        self.add_token(Token::EndOfFile);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let Some(c) = self.advance() else {
            panic!(
                "Could not scan the next token. line={} byte_offset={}",
                &self.line, self.current_byte_offset
            );
        };

        match c {
            b'#' => self.add_token(Token::Hash),
            b'*' => self.add_token(Token::Star),
            b'!' => self.add_token(Token::Bang),
            b' ' => self.add_token(Token::Space),
            b'_' => self.add_token(Token::Underscore),
            b'-' => self.add_token(Token::Dash),
            b'.' => self.add_token(Token::Dot),
            b'(' => self.add_token(Token::LeftParen),
            b')' => self.add_token(Token::RightParen),
            b'[' => self.add_token(Token::LeftSquareBracket),
            b']' => self.add_token(Token::RightSquareBracket),
            b'\\' => self.add_token(Token::Backslash),
            b'\t' => self.add_token(Token::Tab),
            b'\n' => self.add_token(Token::Newline),
            c if c.is_ascii_digit() => self.add_token(Token::Digit(
                &self.source[self.current_byte_offset - 1..self.current_byte_offset],
            )),
            _ => self.handle_string(),
        }
    }

    fn is_token(&self, c: Option<u8>) -> bool {
        match c {
            Some(c) => {
                if c.is_ascii() {
                    c.is_ascii_digit() || SYMBOLS.contains(c as char)
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn handle_string(&mut self) {
        let start_offset = self.current_byte_offset - 1;
        let mut end_byte_offset = start_offset;
        while !self.is_at_end() && !self.is_token(self.peek()) {
            self.advance();
            end_byte_offset += 1;
        }

        let value = &self.source[start_offset..end_byte_offset + 1];

        self.add_token(Token::Text(value));
    }

    fn is_at_end(&self) -> bool {
        self.current_byte_offset >= self.source.bytes().len()
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
    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            return None;
        }
        self.source
            .as_bytes()
            .get(self.current_byte_offset)
            .copied()
    }

    /// Consume the next byte and advance the needle
    /// to point to a potential next character.
    /// byte continution of multi-byte characters
    /// should be handled by the caller.
    fn advance(&mut self) -> Option<u8> {
        if let Some(c) = self
            .source
            .as_bytes()
            .get(self.current_byte_offset)
            .copied()
        {
            if c == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }

            self.current_byte_offset += 1;
            return Some(c);
        }

        self.current_byte_offset += 1;
        None
    }
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

    #[test]
    fn accept_multi_byte_chars() {
        let markdown = r"
        ## This is a title
        
        This should include emojis and **bold text**.
        🤡😜🎉 text 🐙👪.
        ";
        let mut lexer = Lexer::new(markdown);
        let result = lexer.scan();
        assert_eq!(result.len(), 80);
    }
}
