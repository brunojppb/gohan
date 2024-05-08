use std::fmt::{Debug, Display};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TokenType {
    Hash,
    Star,
    Bang,
    Underscore,
    Newline,
    Tab,
    Space,
    Text(String),
    EndOfFile,
}

impl TokenType {
    pub fn lexime(&self) -> String {
        match self {
            Self::Hash => "#".to_string(),
            Self::Star => "*".to_string(),
            Self::Bang => "!".to_string(),
            Self::Tab => "\\t".to_string(),
            Self::Space => " ".to_string(),
            Self::Newline => "\\n".to_string(),
            Self::Underscore => "_".to_string(),
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
