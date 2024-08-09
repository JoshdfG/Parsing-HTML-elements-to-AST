extern crate scraper;

use scraper::Html;

mod lib;

fn parse_html_to_ast(html: &str) -> Html {
    Html::parse_document(html)
}

fn main() {
    let html_strings = lib::HTML_STRINGS;

    html_strings
        .iter()
        .map(|html| parse_html_to_ast(html))
        .for_each(|dom| println!("{:#?}", dom));
}
