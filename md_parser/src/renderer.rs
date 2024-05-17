use crate::{
    ast::{BlockNode, InlineNode, Node},
    lexer::Lexer,
    parser::Parser,
};

/// Renders an HTML string from the given AST
///
/// # Examples
///
/// ```
/// use md_parser::renderer;
/// let markdown = r#"
/// ## Title
///
/// I'm a **paragraph**.
/// "#;
/// let html = renderer::render_html(markdown);
/// println!("{}", html);
/// // <h2>Title</h2><p>I'm a <strong>paragraph</strong>.</p>
/// ```
pub fn render_html(markdown: &str) -> String {
    let mut lexer = Lexer::new(markdown);
    let mut parser = Parser::new(lexer.scan());
    let ast = parser.parse();
    render(ast)
}

fn render(ast: Vec<Node>) -> String {
    let mut text = String::from("");
    for node in ast.iter() {
        visit(&mut text, node);
    }
    text
}

fn visit(buffer: &mut String, node: &Node) {
    match node {
        Node::Block(block) => visit_block(buffer, block),
        Node::Inline(inline) => visit_inline(buffer, inline),
    }
}

fn visit_block(buffer: &mut String, node: &BlockNode) {
    match node {
        BlockNode::Heading(level, inline_nodes) => {
            buffer.push_str(&format!("<h{}>", level));
            visit_inline_nodes(buffer, inline_nodes);
            buffer.push_str(&format!("</h{}>", level));
        }
        BlockNode::Paragraph(inline_nodes) => {
            buffer.push_str("<p>");
            visit_inline_nodes(buffer, inline_nodes);
            buffer.push_str("</p>");
        }
    }
}

fn visit_inline(buffer: &mut String, node: &InlineNode) {
    match node {
        InlineNode::Text(txt) => buffer.push_str(txt),
        InlineNode::Bold(inline_nodes) => {
            buffer.push_str("<strong>");
            visit_inline_nodes(buffer, inline_nodes);
            buffer.push_str("</strong>");
        }
        InlineNode::Digit(d) => buffer.push_str(&d.to_string()),
        InlineNode::LineBreak => buffer.push_str("<br>"),
        InlineNode::Italic(inline_nodes) => {
            buffer.push_str("<em>");
            visit_inline_nodes(buffer, inline_nodes);
            buffer.push_str("</em>");
        }
        InlineNode::Link(text_nodes, link_nodes) => {
            buffer.push_str(r#"<a href=""#);
            visit_inline_nodes(buffer, link_nodes);
            buffer.push_str(r#"">"#);
            visit_inline_nodes(buffer, text_nodes);
            buffer.push_str("</a>");
        }
    }
}

fn visit_inline_nodes(buffer: &mut String, nodes: &[InlineNode]) {
    for inline in nodes.iter() {
        visit_inline(buffer, inline);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::renderer::render_html;

    #[test]
    fn render_html_string() {
        insta::glob!("snapshot_inputs/*.md", |path| {
            let markdown = fs::read_to_string(path).unwrap();
            let result = render_html(&markdown);
            insta::assert_json_snapshot!(result);
        });
    }
}
