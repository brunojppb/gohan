use crate::ast::{Bold, Header, Link, Node, Paragraph};
use crate::token::{Span, Token};

use std::cmp::max;
use std::ops::Range;

// Markdown Grammar
// (* A document is a series of blocks *)
// document = { block } ;

// (* A block can be a paragraph, header, blockquote, list, code block, or horizontal rule *)
// block = paragraph | header | blockquote | list | code_block | horizontal_rule ;

// (* Headers *)
// header = ( "#" | "##" | "###" | "####" | "#####" | "######" ), " ", text ;

// (* Paragraphs are simply text separated by one or more blank lines *)
// paragraph = text, { newline, newline, text } ;

// (* Blockquotes *)
// blockquote = ">", { ">", text } ;

// (* Lists can be unordered or ordered *)
// list = unordered_list | ordered_list ;
// unordered_list = ( "*", " " ), text, { newline, ( "*", " " ), text } ;
// ordered_list = digit, ".", " ", text, { newline, digit, ".", " ", text } ;

// (* Code blocks *)
// code_block = "```", newline*, { text }, newline*, "```" ;

// (* Horizontal rules *)
// horizontal_rule = ( "---" | "***" | "___" ) ;

// (* Inline elements can be within other blocks like paragraphs and headers *)
// text = { inline_element | chars } ;
// inline_element = code | emph | strong | link | image ;

// (* Inline code *)
// code = "`", chars, "`" ;

// (* Emphasis can be italics or bold, using either asterisks or underscores *)
// emph = ( "*" | "_" ), text, ( "*" | "_" ) ;
// strong = ( "**" | "__" ), text, ( "**" | "__" ) ;

// (* Links *)
// link = "[", text, "]", "(", url, ")" ;

// (* Images *)
// image = "!", "[", alt_text, "]", "(", url, ")" ;

// (* Helpers *)
// newline = "\n" | "\r\n" ;
// digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
// chars = ? all visible characters excluding control characters ? ;
// language = ? any string that represents a programming language name ? ;
// url = ? any valid URL ? ;
// title = ? any string ? ;
// alt_text = ? any string ? ;

/// Recursive Descent Parser for transforming
/// the given list of tokens a DOM AST
pub struct Parser<'source> {
    current: usize,
    tokens: &'source [(Token<'source>, Span)],
}

impl<'source> Parser<'source> {
    pub fn new(tokens: &'source [(Token<'source>, Span)]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Node<'source>> {
        let mut nodes: Vec<Node<'source>> = Vec::new();
        while !self.is_at_end() {
            if let Some(node) = self.block() {
                nodes.push(node);
            }
        }

        nodes
    }

    /// Parser step for nested inline elements only.
    /// Helpful for cases where we want to restrict parsing
    /// for within a specific range of tokens within another inline element.
    /// e.g. Links containing bold text and other allowed inline elements
    fn parse_inline(&mut self) -> Vec<Node<'source>> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            if let Some(node) = self.inline() {
                nodes.push(node);
            }
        }

        nodes
    }

    fn block(&mut self) -> Option<Node<'source>> {
        while self.check(&Token::Newline) {
            self.consume(&Token::Newline);
        }

        // Headings can only start as the very first token in a line
        if let Some(&(Token::Hash, span)) = self.peek() {
            if span.col == 1 {
                return self.maybe_heading();
            }
        }

        self.maybe_paragraph()
    }

    fn maybe_heading(&mut self) -> Option<Node<'source>> {
        let mut heading_level: u8 = 0;
        while self.match_token(Token::Hash) {
            heading_level += 1;
        }

        if heading_level > 0 && heading_level <= 6 && self.match_token(Token::Space) {
            let mut inline_elements = Vec::new();
            while let Some(inline) = self.inline() {
                if inline == Node::LineBreak {
                    break;
                }
                inline_elements.push(inline)
            }
            return Some(Node::Header(Header {
                level: heading_level,
                children: inline_elements,
            }));
        }

        // in case of detected hashes, at this point,
        // we know they are not valid header levels
        // so let's rewind and let them be handled as normal text
        if heading_level > 0 {
            self.step_back(heading_level as usize);
        }

        self.maybe_paragraph()
    }

    fn maybe_paragraph(&mut self) -> Option<Node<'source>> {
        // A paragraph might or might not start with a newline
        // @TODO: Add newlines before paragraphs as linebreak nodes?
        // So we just consume newlines outside of a paragraph and discard them.
        // I might need to revisit this and add Linebreak as a inline node?
        while self.check(&Token::Newline) && !self.is_at_end() {
            self.consume(&Token::Newline);
        }

        let mut inline_elements = Vec::new();

        while let Some(inline) = self.inline() {
            inline_elements.push(inline);
        }

        if inline_elements.is_empty() {
            return None;
        }

        Some(Node::Paragraph(Paragraph {
            children: inline_elements,
        }))
    }

    fn inline(&mut self) -> Option<Node<'source>> {
        if self.is_at_end() {
            return None;
        }

        if let Some((token, _)) = self.peek() {
            let node = match token {
                // Hitting end of the file, just advance and halt
                Token::EndOfFile => {
                    self.advance();
                    return None;
                }
                // Two consecutive newlines should break off from any inline elements
                // and give it a chance to a new block or inline element to be constructed
                Token::Newline if self.check_next(Token::Newline) => {
                    return None;
                }
                Token::Newline => Node::LineBreak,
                Token::Star => return self.maybe_bold(),
                Token::LeftSquareBracket => return self.maybe_link(),
                Token::Text(_)
                | Token::Digit(_)
                | Token::Space
                | Token::Dash
                | Token::Dot
                | Token::Underscore
                | Token::Bang
                | Token::Hash
                | Token::LeftParen
                | Token::RightParen
                | Token::RightSquareBracket
                | Token::Backslash => Node::Text(token.literal()),
                // block-level tokens should be interpreted outside of the inline loop
                // to give them a chance of being interpreted as block-level elements
                t if t.is_block_level_token() => return None,
                t => todo!("Token not handled yet: {}", t),
            };
            self.advance();
            return Some(node);
        }

        None
    }

    fn maybe_link(&mut self) -> Option<Node<'source>> {
        let mut marker = LinkMarker::new();
        let rewind_position = self.current;
        let mut steps = 0;
        // Any inline element can partially show-up and should be represented as text,
        // but if we find the right token makers that can complete a link, we should
        // rewind and structure it as a Link inline node instead.
        while !marker.is_link() && !self.is_at_end() {
            if let Some((next, _)) = self.advance() {
                steps += 1;
                match next {
                    Token::LeftSquareBracket if marker.is_empty() => {
                        marker.set_start_text(self.current)
                    }
                    // The closing text of a link must be followed by "]("
                    Token::RightSquareBracket if marker.has_open_text() => {
                        if self.peek_token().is_some_and(|t| t == &Token::LeftParen) {
                            marker.set_end_text(self.current - 1);
                            marker.set_start_url(self.current + 1);
                        }
                    }
                    Token::RightParen if marker.has_open_url() => {
                        marker.set_end_url(self.current - 1)
                    }
                    token if token == &Token::Newline => {
                        if let Some(&(Token::Newline, _)) = self.peek() {
                            break;
                        }
                    }
                    _ => {}
                };
            }
        }

        self.rewind(rewind_position);

        // We are guaranteed to have a well-structured link here
        // lets force-consume all the special tokens
        if let Some((text_range, url_range)) = marker.ranges() {
            let mut text_parser = Self::new(&self.tokens[text_range]);
            let text_nodes = text_parser.parse_inline();

            let mut url_parser = Self::new(&self.tokens[url_range]);
            let url_nodes = url_parser.parse_inline();
            self.current += steps;

            let link = Node::Link(Link {
                children: text_nodes,
                url: url_nodes,
            });

            return Some(link);
        }

        // Otherwise we bail, rewind and let the next loop handle
        // each token as as normal text or other inline elements
        self.consume(&Token::LeftSquareBracket);
        Some(Node::Text(Token::LeftSquareBracket.literal()))
    }

    fn maybe_bold(&mut self) -> Option<Node<'source>> {
        let rewind_position = self.current;
        let mut marker = InlineMarker::new();
        let mut steps = 0;

        while !marker.is_closed() && !self.is_at_end() {
            steps += 1;
            if let Some((next, _)) = self.advance() {
                match next {
                    Token::Star => {
                        if self.check(&Token::Star) {
                            if marker.is_empty()
                                && !self.peek_next_token().is_some_and(|t| {
                                    t == &Token::Space
                                        || t == &Token::Newline
                                        || t == &Token::EndOfFile
                                })
                            {
                                marker.open(self.current + 1);
                            } else if marker.is_open()
                                && !self.tokens.get(self.current - 2).is_some_and(|(t, _)| {
                                    t == &Token::Space
                                        || t == &Token::Newline
                                        || t == &Token::EndOfFile
                                })
                            {
                                marker.close(self.current - 1);
                                steps += 1;
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                    // Two consecutive newlines should break out from the inline element
                    Token::Newline => {
                        if self.check_next(Token::Newline) {
                            break;
                        }
                    }

                    // If we enter the potential inner elements of bold element
                    // and they are not following a `**`, this is not a bold element.
                    _t if marker.is_empty() => break,

                    // Any other token should move along as they can be nested within
                    // the bold text as just text or inner inline elements
                    _t => {}
                };
            }
        }

        self.rewind(rewind_position);

        // At this point, we are sure we have a bold element.
        if let Some(bold_text_range) = marker.range() {
            let t = &self.tokens[bold_text_range];
            let mut text_parser = Self::new(t);
            let text_nodes = text_parser.parse_inline();

            self.current += steps;

            let bold = Node::Bold(Bold {
                children: text_nodes,
            });

            return Some(bold);
        }

        // Otherwise we bail, rewind and let the next loop handle each token
        // be handled as normal text or other inline elements
        self.consume(&Token::Star);
        Some(Node::Text(Token::Star.literal()))
    }

    fn consume(&mut self, kind: &Token) -> &Token {
        if let Some(token) = self.advance() {
            if token.0 == *kind {
                return &token.0;
            }

            panic!(
                "Invalid next token to consume. expected={:#?} found={:#?} span={:#?}",
                kind, token.0, token.1
            );
        }

        panic!("Could not consume next token kind={}", kind)
    }

    fn advance(&mut self) -> Option<&(Token<'source>, Span)> {
        if self.is_at_end() {
            return None;
        }

        self.current += 1;
        return self.previous();
    }

    /// Walk back the given number of steps,
    /// but never move to a negative position
    fn step_back(&mut self, num_steps: usize) -> Option<&(Token<'source>, Span)> {
        self.current = max(0, self.current - num_steps);
        return self.peek();
    }

    /// Jump straight to an specific position
    /// with no bounds validation
    fn rewind(&mut self, to_position: usize) {
        self.current = to_position;
    }

    fn previous(&self) -> Option<&(Token<'source>, Span)> {
        self.tokens.get(self.current - 1)
    }

    fn peek(&self) -> Option<&(Token<'source>, Span)> {
        self.tokens.get(self.current)
    }

    fn peek_token(&self) -> Option<&Token> {
        match self.peek() {
            Some((token, _)) => Some(token),
            None => None,
        }
    }

    fn peek_next_token(&self) -> Option<&Token> {
        match self.peek_next() {
            Some((token, _)) => Some(token),
            None => None,
        }
    }

    /// Get the next token in line, but do not consume it
    fn peek_next(&self) -> Option<&(Token<'source>, Span)> {
        self.tokens.get(self.current + 1)
    }

    /// Compare the current token, but do not consume it.
    fn check(&self, token: &Token) -> bool {
        self.peek().is_some_and(|t| t.0 == *token)
    }

    /// Compare the given token to the next one in line
    /// but do not consume it.
    fn check_next(&self, token: Token) -> bool {
        self.peek_next().is_some_and(|t| t.0 == token)
    }

    /// Compare the given token to the next one in line
    /// and consume it
    fn match_token(&mut self, expected: Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.check(&expected) {
            self.advance();
            return true;
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[derive(Debug)]
struct LinkMarker {
    start_text: Option<usize>,
    end_text: Option<usize>,
    start_url: Option<usize>,
    end_url: Option<usize>,
}

/// helful for holding the boundaries of a Link element during parsing
impl LinkMarker {
    fn new() -> Self {
        Self {
            start_text: None,
            end_text: None,
            start_url: None,
            end_url: None,
        }
    }

    fn set_start_text(&mut self, index: usize) {
        self.start_text = Some(index);
    }

    fn set_end_text(&mut self, index: usize) {
        self.end_text = Some(index);
    }

    fn set_start_url(&mut self, index: usize) {
        self.start_url = Some(index);
    }

    fn set_end_url(&mut self, index: usize) {
        self.end_url = Some(index);
    }

    fn is_link(&self) -> bool {
        self.start_text.is_some()
            && self.end_text.is_some()
            && self.start_url.is_some()
            && self.end_url.is_some()
    }

    fn is_empty(&self) -> bool {
        self.start_text.is_none()
            && self.end_text.is_none()
            && self.start_url.is_none()
            && self.end_url.is_none()
    }

    fn has_open_text(&self) -> bool {
        self.start_text.is_some()
            && self.end_text.is_none()
            && self.start_url.is_none()
            && self.end_url.is_none()
    }

    fn has_open_url(&self) -> bool {
        self.start_text.is_some()
            && self.end_text.is_some()
            && self.start_url.is_some()
            && self.end_url.is_none()
    }

    /// given a complete link, extract the ranges of its inner components
    fn ranges(&self) -> Option<(Range<usize>, Range<usize>)> {
        match (self.start_text, self.end_text, self.start_url, self.end_url) {
            (Some(text_start), Some(text_end), Some(url_start), Some(url_end)) => {
                Some((text_start..text_end, url_start..url_end))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct InlineMarker {
    start: Option<usize>,
    end: Option<usize>,
}

impl InlineMarker {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    fn is_open(&self) -> bool {
        self.start.is_some() && self.end.is_none()
    }

    fn is_closed(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    fn open(&mut self, index: usize) {
        self.start = Some(index);
    }

    fn close(&mut self, index: usize) {
        self.end = Some(index);
    }

    fn range(&self) -> Option<Range<usize>> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(start..end),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_markdown() {
        insta::glob!("snapshot_inputs/*.md", |path| {
            let markdown = fs::read_to_string(path).unwrap();
            let mut lexer = Lexer::new(&markdown);
            let tokens = lexer.scan();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();
            insta::assert_json_snapshot!(ast);
        });
    }
}
