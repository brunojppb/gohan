use crate::{ast::Node, lexer::Lexer, parser::Parser};

/// Renders an HTML string from the given AST
///
/// # Examples
///
/// ```
/// use md_parser::renderer;
/// let markdown = r"I'm a **paragraph**.";
/// let html = renderer::render_html(markdown);
/// assert_eq!(html, "<p>I'm a <strong>paragraph</strong>.</p>");
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
        Node::Header(_) | Node::Paragraph(_) => visit_block(buffer, node),
        node => visit_inline(buffer, node),
    }
}

fn visit_block(buffer: &mut String, node: &Node) {
    match node {
        Node::Header(header) => {
            buffer.push_str(&format!("<h{}>", header.level));
            visit_inline_nodes(buffer, &header.children);
            buffer.push_str(&format!("</h{}>", header.level));
        }
        Node::Paragraph(paragraph) => {
            buffer.push_str("<p>");
            for (idx, node) in paragraph.children.iter().enumerate() {
                // Within a paragraph, whenever we hit the last node
                // and it's a newline, we can just discard it as the
                // paragraph element behaves itself as a block.
                if idx >= paragraph.children.len() - 1 && node == &Node::LineBreak {
                    continue;
                }
                visit_inline(buffer, node);
            }
            buffer.push_str("</p>");
        }
        _ => panic!("Node {:#?} not supported as a block node type", node),
    }
}

fn visit_inline(buffer: &mut String, node: &Node) {
    match node {
        Node::Text(txt) => buffer.push_str(txt),
        Node::Strong(strong) => {
            buffer.push_str("<strong>");
            visit_inline_nodes(buffer, &strong.children);
            buffer.push_str("</strong>");
        }
        Node::Digit(d) => buffer.push_str(d),
        Node::LineBreak => buffer.push_str("<br>"),
        Node::Emphasis(italic) => {
            buffer.push_str("<em>");
            visit_inline_nodes(buffer, &italic.children);
            buffer.push_str("</em>");
        }
        Node::Link(link) => {
            buffer.push_str(r#"<a href=""#);
            visit_inline_nodes(buffer, &link.url);
            buffer.push_str(r#"">"#);
            visit_inline_nodes(buffer, &link.children);
            buffer.push_str("</a>");
        }
        _ => panic!("Node {:#?} not supported as a inline node type", node),
    }
}

fn visit_inline_nodes(buffer: &mut String, nodes: &[Node]) {
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

    #[test]
    fn render_plan() {
        let markdown = r"
## Title

I'm a **paragraph**.
";
        let html = render_html(markdown);
        assert_eq!(
            html,
            "<h2>Title</h2><p>I'm a <strong>paragraph</strong>.</p>"
        );
    }
}
