use pulldown_cmark::{Parser, Options, html};
use std::fs;

fn main() {
    // Read the Markdown file into a string
    let markdown_input = fs::read_to_string("sample.md").expect("Failed to read file");

    // Extract the first level 1 heading as the title
    let title = extract_first_h1(&markdown_input).unwrap_or_else(|| String::from("Untitled"));

    // Convert the Markdown content to HTML
    let html_content = markdown_to_html(&markdown_input);

    // Print the title and HTML content
    print_title_and_content(&title, &html_content);
}

/// Extracts the first level 1 heading (`#`) from the Markdown content.
fn extract_first_h1(markdown: &str) -> Option<String> {
    for line in markdown.lines() {
        if line.starts_with("# ") {
            return Some(line.trim_start_matches("# ").trim().to_string());
        }
    }
    None
}

/// Converts Markdown content to HTML using `pulldown-cmark`.
fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Prints the title and HTML content.
fn print_title_and_content(title: &str, content: &str) {
    println!("Title: {}", title);
    println!("HTML Content:\n{}", content);
}