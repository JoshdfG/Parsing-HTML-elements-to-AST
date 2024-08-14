use html_ast::HTML_CONTENT;

#[derive(Clone, Copy)]
enum HtmlState {
    OutsideTag,
    InsideTag,
    InsideAttributeName,
    InsideAttributeValue,
    InsideTagContent,
}

fn parse_html(html: &str) -> Vec<String> {
    let mut state = HtmlState::OutsideTag;
    let mut current_tag_content = String::new();
    let mut parsed_data = Vec::new();

    for ch in html.chars() {
        match (state, ch) {
            (HtmlState::OutsideTag, '<') => {
                state = HtmlState::InsideTag;
            }
            (HtmlState::InsideTag, '>') => {
                state = HtmlState::InsideTagContent;
            }
            (HtmlState::InsideTagContent, '<') => {
                if !current_tag_content.is_empty() {
                    parsed_data.push(current_tag_content.clone());
                    current_tag_content.clear();
                }
                state = HtmlState::InsideTag;
            }
            _ => match state {
                HtmlState::InsideTagContent => current_tag_content.push(ch),
                _ => {}
            },
        }
    }

    if !current_tag_content.is_empty() {
        parsed_data.push(current_tag_content);
    }

    parsed_data
}

fn main() {
    let parsed_data = parse_html(HTML_CONTENT);
    for content in parsed_data {
        println!("{}", content);
    }
}
