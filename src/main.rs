mod logic;
use html_ast::HTML_CONTENT;

fn initializer() {
    let parsed_data = logic::parse_html(HTML_CONTENT);

    for parsed in parsed_data {
        println!(
            "ParsedHtml {{\n\tTag name:{}\n\tContent: {}\n\tAttributes: [{}]\n}}",
            parsed.tag.name,
            parsed.content.trim(),
            parsed.attributes.trim()
        );
    }
}

fn main() {
    initializer();
}
