use crate::token::Token;

pub enum InlineNode {
    Span(String),
    Link { text: String, url: String },
}

pub enum BlockNode {
    Heading(InlineNode),
    Paragraph(Vec<InlineNode>),
}
