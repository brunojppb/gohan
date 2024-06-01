use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Node<'s> {
    Header(Header<'s>),
    Paragraph(Paragraph<'s>),
    Link(Link<'s>),
    Bold(Bold<'s>),
    Italic(Italic<'s>),
    Digit(&'s str),
    Text(&'s str),
    LineBreak,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Header<'s> {
    pub level: u8,
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Paragraph<'s> {
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Link<'s> {
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
    /// List of Text nodes
    pub url: Vec<Node<'s>>,
    // TODO: Support title for tooltips
    // title: Option<&'s str>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bold<'s> {
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Italic<'s> {
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
}
