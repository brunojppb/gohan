use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Node<'s> {
    Header(Header<'s>),
    Paragraph(Paragraph<'s>),
    Link(Link<'s>),
    Strong(Strong<'s>),
    Emphasis(Emphasis<'s>),
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
pub struct Strong<'s> {
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Emphasis<'s> {
    #[serde(borrow)]
    pub children: Vec<Node<'s>>,
}
