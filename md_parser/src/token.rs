use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Token<'a> {
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
    LeftSquareBracket,
    RightSquareBracket,
    Digit(&'a str),
    Text(&'a str),
    EndOfFile,
}

impl<'a> Display for Token<'a> {
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
            Self::LeftSquareBracket => f.write_str("`[`"),
            Self::RightSquareBracket => f.write_str("`]`"),
            Self::Tab => f.write_str("`\\t`"),
            Self::Space => f.write_str("` `"),
            Self::Newline => f.write_str("`\\n`"),
            Self::Underscore => f.write_str("`_`"),
            Self::Digit(number) => f.write_str(&format!("digit:'{}'", &number.to_string())),
            Self::Text(text) => f.write_str(&format!("text:'{}'", text)),
            Self::EndOfFile => f.write_str("`EOF`"),
        }
    }
}

impl<'a> Token<'a> {
    /// Literal string representation of a given token
    pub fn literal(&self) -> &'a str {
        match self {
            Self::Hash => "#",
            Self::Star => "*",
            Self::Bang => "!",
            Self::Dot => ".",
            Self::Dash => "-",
            Self::Underscore => "_",
            Self::Backslash => "\\",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftSquareBracket => "[",
            Self::RightSquareBracket => "]",
            Self::Tab => "\t",
            Self::Newline => "\n",
            Self::Space => " ",
            Self::Digit(d) => d,
            Self::Text(t) => t,
            Self::EndOfFile => "",
        }
    }

    pub fn is_block_level_token(&self) -> bool {
        matches!(self, Self::Hash)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " @ {}:{}", self.line, self.col)
    }
}
