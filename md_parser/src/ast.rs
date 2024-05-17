use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Node<'s> {
    #[serde(borrow)]
    Block(BlockNode<'s>),
    Inline(InlineNode<'s>),
}

/// Block level elements
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockNode<'s> {
    Heading(u8, Vec<InlineNode<'s>>), // (heading level, elements)
    #[serde(borrow)]
    Paragraph(Vec<InlineNode<'s>>),
}

/// inline level elements
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum InlineNode<'s> {
    Bold(Vec<InlineNode<'s>>),
    Italic(Vec<InlineNode<'s>>),
    Link(Vec<InlineNode<'s>>, Vec<InlineNode<'s>>), // (elements, url)
    Digit(usize),
    Text(&'s str),
    LineBreak,
}
