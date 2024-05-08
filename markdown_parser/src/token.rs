use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub position: usize,
    pub kind: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(position: usize, kind: TokenType, line: usize) -> Self {
        Self {
            position,
            kind,
            line,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Hash,
    Star,
    Bang,
    Underscore,
    Newline,
    Tab,
    Space,
    Dot,
    Dash,
    Backslash,
    LeftParen,
    RightParen,
    LeftSquareBraket,
    RightSquareBraket,
    Number(usize),
    Text(String),
    EndOfFile,
}

impl TokenType {
    /// Lexeme representation. Mostly for debugging
    pub fn lexime(&self) -> String {
        match self {
            Self::Hash => "#".to_string(),
            Self::Star => "*".to_string(),
            Self::Bang => "!".to_string(),
            Self::Dot => ".".to_string(),
            Self::Dash => "-".to_string(),
            Self::Backslash => "\\".to_string(),
            Self::LeftParen => "(".to_string(),
            Self::RightParen => ")".to_string(),
            Self::LeftSquareBraket => "[".to_string(),
            Self::RightSquareBraket => "]".to_string(),
            Self::Tab => "\\t".to_string(),
            Self::Space => " ".to_string(),
            Self::Newline => "\\n".to_string(),
            Self::Underscore => "_".to_string(),
            Self::Number(number) => number.to_string(),
            Self::Text(text) => text.to_string(),
            Self::EndOfFile => "EOF".to_string(),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lexeme={}", self.lexime())
    }
}
