extern crate scraper;

use scraper::Html;

mod lib;

fn parse_html_to_ast(html: &str) -> Html {
    // Parse the HTML string into a DOM tree
    Html::parse_document(html)
}

fn main() {
    let html_strings = lib::HTML_STRINGS;

    html_strings
        .iter()
        .map(|html| parse_html_to_ast(html))
        .for_each(|dom| println!("{:#?}", dom));

    // `dom` now contains the root of the parsed HTML tree
    // You can traverse and manipulate the DOM tree as needed
}
