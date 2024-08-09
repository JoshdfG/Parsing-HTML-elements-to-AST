extern crate scraper;
use scraper::{Html, Selector};
mod lib;

fn main() {
    // Parse the HTML content
    let document = Html::parse_document(lib::HTML_CONTENT);

    // Create a selector for the paragraph tag
    let selector = Selector::parse("p,a,h1,ul,li,table,span").unwrap();

    // Find all elements that match the selector
    for element in document.select(&selector) {
        // This line will Extract and print the text content of the element
        println!("Text: {}", element.text().collect::<Vec<_>>().concat());
    }
}
