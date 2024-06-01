use crate::ast::{BlockNode, InlineNode, Node};
use crate::token::{Span, Token};

use std::cmp::max;

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

// The given parent of an element when recursing
// on inner elements. Helpful for controlling whether
// we allow some types of elements to have chidren elements
#[derive(PartialEq, Eq)]
enum Parent {
    Block,
    Inline,
}

/// Recursive Descent Parser for transforming
/// the given list of tokens a DOM AST
pub struct Parser<'source> {
    current: usize,
    tokens: &'source Vec<(Token<'source>, Span)>,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: &'source Vec<(Token<'source>, Span)>) -> Self {
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
            while let Some(inline) = self.inline(Parent::Block) {
                if inline == InlineNode::LineBreak {
                    break;
                }
                inline_elements.push(inline)
            }
            return Some(Node::Block(BlockNode::Heading(
                heading_level,
                inline_elements,
            )));
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

        while let Some(inline) = self.inline(Parent::Block) {
            inline_elements.push(inline);
        }

        if inline_elements.is_empty() {
            return None;
        }

        Some(Node::Block(BlockNode::Paragraph(inline_elements)))
    }

    fn inline(&mut self, parent: Parent) -> Option<InlineNode<'source>> {
        if self.is_at_end() {
            return None;
        }

        if let Some((token, _)) = self.peek() {
            let node = match token {
                // Two consecutive newlines should break off from any inline elements
                // and give it a chance to a new block or inline element to be constructed
                Token::Newline if self.check_next(Token::Newline) => {
                    return None;
                }
                Token::Newline => InlineNode::LineBreak,
                Token::Star => return self.maybe_bold(),
                Token::LeftSquareBracket if parent == Parent::Block => return self.maybe_link(),
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
                | Token::LeftSquareBracket
                | Token::RightSquareBracket
                | Token::Backslash => InlineNode::Text(token.literal()),
                t if t.is_block_level_token() => return None,
                t => todo!("unhandled token: {}", t),
            };
            self.advance();
            return Some(node);
        }

        None
    }

    fn maybe_link(&mut self) -> Option<InlineNode<'source>> {
        let mut markers: [u8; 4] = [0, 0, 0, 0];
        let rewind_position = self.current;
        // Any inline element can partially show-up and should be represented as text,
        // but if we find the right token makers that can complete a link, we should
        // rewind and structure it as a Link inline node instead.
        'outer: while markers != [1, 1, 1, 1] && !self.is_at_end() {
            while let Some((next, _)) = self.advance() {
                match next {
                    Token::LeftSquareBracket if markers == [0, 0, 0, 0] => markers[0] = 1,
                    // The closing text of a link must be followed by "]("
                    Token::RightSquareBracket if markers == [1, 0, 0, 0] => {
                        if self.peek_token().is_some_and(|t| t == &Token::LeftParen) {
                            markers[1] = 1;
                            markers[2] = 1;
                        }
                    }
                    Token::RightParen if markers == [1, 1, 1, 0] => markers[3] = 1,
                    token if token == &Token::Newline => {
                        if let Some(&(Token::Newline, _)) = self.peek() {
                            break 'outer;
                        }
                    }
                    _ => {}
                };
            }
        }

        self.rewind(rewind_position);

        // We are guaranteed to have a well-structured link here
        // lets force-consume all the special tokens
        if markers == [1, 1, 1, 1] {
            self.consume(&Token::LeftSquareBracket);

            let mut link_text = Vec::new();

            while !self.check(&Token::RightSquareBracket) && !self.is_at_end() {
                if let Some(inline) = self.inline(Parent::Inline) {
                    link_text.push(inline);
                } else {
                    break;
                }
            }

            self.consume(&Token::RightSquareBracket);
            self.consume(&Token::LeftParen);

            let mut url = Vec::new();
            while !self.check(&Token::RightParen) && !self.is_at_end() {
                if let Some(inline) = self.inline(Parent::Inline) {
                    url.push(inline);
                } else {
                    break;
                }
            }

            self.consume(&Token::RightParen);

            return Some(InlineNode::Link(link_text, url));
        }

        // Otherwise we bail, rewind and let the next loop handle
        // each token as as normal text or other inline elements
        self.consume(&Token::LeftSquareBracket);
        Some(InlineNode::Text(Token::LeftSquareBracket.literal()))
    }

    fn maybe_bold(&mut self) -> Option<InlineNode<'source>> {
        let mut markers: [u8; 2] = [0, 0];
        let rewind_position = self.current;
        'outer: while markers != [1, 1] && !self.is_at_end() {
            while let Some((next, _)) = self.advance() {
                match next {
                    Token::Star => {
                        if self.peek_token().is_some_and(|t| t == &Token::Star) {
                            if rewind_position == self.current - 1 {
                                markers[0] = 1;
                            } else {
                                markers[1] = 1;
                            }
                        }
                    }
                    // Two consecutive newlines should break out from the inline element
                    token if token == &Token::Newline => {
                        if self.peek_token().is_some_and(|t| t == &Token::Newline) {
                            break 'outer;
                        }
                    }
                    _ => {}
                };
            }
        }

        self.rewind(rewind_position);

        if markers == [1, 1] {
            self.consume(&Token::Star);
            self.consume(&Token::Star);
            let mut inner = Vec::new();
            while !self.check(&Token::Star) && !self.is_at_end() {
                if let Some(inline) = self.inline(Parent::Inline) {
                    inner.push(inline);
                } else {
                    panic!("Invalid inline node for link URL component");
                }
            }
            // Consume the wrapping "**" around bold tokens
            self.consume(&Token::Star);
            self.consume(&Token::Star);
            return Some(InlineNode::Bold(inner));
        }

        // Otherwise we bail, rewind and let the next loop handle each token
        // be handled as normal text or other inline elements
        self.rewind(rewind_position);
        self.consume(&Token::Star);
        Some(InlineNode::Text(Token::Star.literal()))
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
        self.tokens
            .get(self.current)
            .filter(|t| t.0 == Token::EndOfFile)
            .is_some()
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
