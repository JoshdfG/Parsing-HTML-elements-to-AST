use html_ast::HTML_CONTENT;

#[derive(Clone, Copy)]
enum HtmlState {
    OutsideTag,
    InsideTag,
    InsideAttributeName,
    InsideAttributeValue,
    InsideTagContent,
}

fn parse_html(html: &str) -> Vec<(String, Vec<(String, String)>, String)> {
    let mut state = HtmlState::OutsideTag;
    let mut current_tag = String::new();
    let mut current_attribute_name = String::new();
    let mut current_attribute_value = String::new();
    let mut attributes = Vec::new();
    let mut current_tag_content = String::new();
    let mut parsed_data = Vec::new();

    for ch in html.chars() {
        match (state, ch) {
            (HtmlState::OutsideTag, '<') => {
                state = HtmlState::InsideTag;
                current_tag.clear();
                attributes.clear();
                current_tag_content.clear();
            }
            (HtmlState::InsideTag, ' ') => {
                state = HtmlState::InsideAttributeName;
            }
            (HtmlState::InsideAttributeName, '=') => {
                state = HtmlState::InsideAttributeValue;
            }
            (HtmlState::InsideAttributeValue, '\"') => {
                attributes.push((
                    current_attribute_name.clone(),
                    current_attribute_value.clone(),
                ));
                current_attribute_name.clear();
                current_attribute_value.clear();
                state = HtmlState::InsideAttributeName;
            }
            (HtmlState::InsideTag, '>') => {
                state = HtmlState::InsideTagContent;
            }
            (HtmlState::InsideTagContent, '<') => {
                parsed_data.push((
                    current_tag.clone(),
                    attributes.clone(),
                    current_tag_content.clone(),
                ));
                current_tag.clear();
                attributes.clear();
                current_tag_content.clear();
                state = HtmlState::InsideTag;
            }
            _ => match state {
                HtmlState::InsideTag => current_tag.push(ch),
                HtmlState::InsideAttributeName => current_attribute_name.push(ch),
                HtmlState::InsideAttributeValue => current_attribute_value.push(ch),
                HtmlState::InsideTagContent => current_tag_content.push(ch),
                HtmlState::OutsideTag => {}
            },
        }
    }

    parsed_data
}

fn main() {
    let parsed_data = parse_html(HTML_CONTENT);
    for (tag, attributes, content) in parsed_data {
        println!("Tag: {}", tag);
        for (name, value) in attributes {
            println!("  {} = {}", name, value);
        }
        println!("  Content: {}", content);
    }
}
