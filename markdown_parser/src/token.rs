use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token<'a> {
    pub position: usize,
    #[serde(borrow)]
    pub kind: TokenType<'a>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(position: usize, kind: TokenType<'a>, line: usize) -> Self {
        Self {
            position,
            kind,
            line,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType<'a> {
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
    Text(&'a str),
    EndOfFile,
}

impl<'a> Display for TokenType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hash => f.write_str("`#`"),
            Self::Star => f.write_str("`*`"),
            Self::Bang => f.write_str("`!`"),
            Self::Dot => f.write_str("`.`"),
            Self::Dash => f.write_str("`-`"),
            Self::Backslash => f.write_str("`\\`"),
            Self::LeftParen => f.write_str("`(`"),
            Self::RightParen => f.write_str("`)`"),
            Self::LeftSquareBraket => f.write_str("`[`"),
            Self::RightSquareBraket => f.write_str("`]`"),
            Self::Tab => f.write_str("`\\t`"),
            Self::Space => f.write_str("` `"),
            Self::Newline => f.write_str("`\\n`"),
            Self::Underscore => f.write_str("`_`"),
            Self::Number(number) => f.write_str(&number.to_string()),
            Self::Text(text) => f.write_str(text),
            Self::EndOfFile => f.write_str("`EOF`"),
        }
    }
}
