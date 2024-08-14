use html_ast::HTML_CONTENT;

#[derive(Clone, Copy)]
enum HtmlState {
    OutsideTag,
    InsideTag,
    InsideAttributeName,
    InsideAttributeValue,
    InsideTagContent,
}

struct Tag {
    tag: String,
    attributes: Vec<(String, String)>,
    content: String,
}

fn parse_html(html: &str) -> Vec<Tag> {
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
                parsed_data.push(Tag {
                    tag: current_tag.into(),
                    attributes: attributes.into(),
                    content: current_tag_content.into(),
                });
                current_tag = String::new();
                attributes = Vec::new();
                current_tag_content = String::new();
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

    parsed_data.into_iter().for_each(|target| {
        println!("Tag: {}", target.tag);
        target.attributes.into_iter().for_each(|(name, value)| {
            println!("  {} = {}", name, value);
        });
        println!("  Content: {}", target.content);
    });
}
