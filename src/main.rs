use html_ast::HTML_CONTENT;

#[derive(Debug, Clone, Copy)]
enum HtmlState {
    OutsideTag,
    InsideTag,
    InsideAttributeName,
    InsideAttributeValue,
    InsideTagContent,
}

#[derive(Debug)]
struct Tag {
    name: String,
}

#[derive(Debug)]
struct ParsedHtml {
    tag: Tag,
    content: String,
    attributes: String,
}

fn parse_html(html: &str) -> Vec<ParsedHtml> {
    let mut state = HtmlState::OutsideTag;
    let mut current_tag_name = String::new();
    let mut current_attribute_name = String::new();
    let mut current_attribute_value = String::new();
    let mut attributes = String::new();
    let mut current_tag_content = String::new();
    let mut parsed_data = Vec::new();
    let mut is_self_closing = false;
    let mut tag_stack: Vec<String> = Vec::new(); // Explicit type annotation for tag_stack
    let mut inside_quotes = false;

    for ch in html.chars() {
        match (state, ch) {
            (HtmlState::OutsideTag, '<') => {
                state = HtmlState::InsideTag;
                current_tag_name.clear();
                attributes.clear();
                current_tag_content.clear();
                is_self_closing = false;
            }
            (HtmlState::InsideTag, ' ') => {
                state = HtmlState::InsideAttributeName;
            }
            (HtmlState::InsideTag, '>') => {
                if is_self_closing {
                    parsed_data.push(ParsedHtml {
                        tag: Tag {
                            name: current_tag_name.clone(),
                        },
                        content: String::new(),
                        attributes: attributes.clone(),
                    });
                    state = HtmlState::OutsideTag;
                } else {
                    state = HtmlState::InsideTagContent;
                }
            }
            (HtmlState::InsideTag, '/') => {
                is_self_closing = true;
            }
            (HtmlState::InsideTag, ch) => {
                current_tag_name.push(ch);
            }
            (HtmlState::InsideAttributeName, '=') => {
                state = HtmlState::InsideAttributeValue;
                attributes.push_str(&format!("{}=", current_attribute_name));
                current_attribute_name.clear();
                inside_quotes = false;
            }
            (HtmlState::InsideAttributeName, ' ') => {
                // Handle cases where attribute names are separated by spaces
                if !current_attribute_name.is_empty() {
                    attributes.push_str(&current_attribute_name);
                    current_attribute_name.clear();
                }
            }
            (HtmlState::InsideAttributeName, '>') => {
                // Handle case where there's a '>' directly after attribute name without value
                if !current_attribute_name.is_empty() {
                    attributes.push_str(&current_attribute_name);
                    current_attribute_name.clear();
                }
                state = HtmlState::InsideTagContent;
            }
            (HtmlState::InsideAttributeName, ch) => {
                current_attribute_name.push(ch);
            }
            (HtmlState::InsideAttributeValue, '\"') => {
                inside_quotes = !inside_quotes;
                if !inside_quotes {
                    attributes.push_str(&format!("\"{}\"", current_attribute_value));
                    current_attribute_value.clear();
                    state = HtmlState::InsideAttributeName;
                }
            }
            (HtmlState::InsideAttributeValue, '>') if !inside_quotes => {
                attributes.push_str(&current_attribute_value);
                state = HtmlState::InsideTagContent;
            }
            (HtmlState::InsideAttributeValue, ch) => {
                current_attribute_value.push(ch);
            }
            (HtmlState::InsideTagContent, '<') => {
                if tag_stack.is_empty() {
                    parsed_data.push(ParsedHtml {
                        tag: Tag {
                            name: current_tag_name.clone(),
                        },
                        content: current_tag_content.clone(),
                        attributes: attributes.clone(),
                    });
                    current_tag_name.clear();
                    attributes.clear();
                    current_tag_content.clear();
                    state = HtmlState::OutsideTag;
                } else {
                    state = HtmlState::InsideTag;
                    current_tag_name.clear();
                    attributes.clear();
                    is_self_closing = false;
                }
            }
            (HtmlState::InsideTagContent, ch) => {
                current_tag_content.push(ch);
            }

            _ => match state {
                HtmlState::InsideTag => {
                    current_tag_name.push(ch);
                }
                HtmlState::InsideAttributeName => current_attribute_name.push(ch),
                HtmlState::InsideAttributeValue => current_attribute_value.push(ch),
                HtmlState::InsideTagContent => current_tag_content.push(ch),
                HtmlState::OutsideTag => {}
            },
        }
    }

    if !current_tag_name.is_empty() {
        parsed_data.push(ParsedHtml {
            tag: Tag {
                name: current_tag_name,
            },
            content: current_tag_content,
            attributes,
        });
    }

    parsed_data
}

fn main() {
    let parsed_data = parse_html(HTML_CONTENT);

    for parsed in parsed_data {
        println!(
            "ParsedHtml {{\n\tTag: name: {}\n\tContent: {}\n\tAttributes: [{}]\n}}",
            parsed.tag.name,
            parsed.content.trim(),
            parsed.attributes.trim()
        );
    }
}
