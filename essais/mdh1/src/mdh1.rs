use pulldown_cmark::{Parser as MdParser, html, Event, Tag};
use std::fs;


fn extract_first_heading(markdown: &str) -> Option<String> {
    let mut parser = MdParser::new(markdown);
    let title = "15121.2.silver.snc";
    for event in parser {
        if let Event::Start(Tag::Heading(1)) = event {
           // while let Some(event) = parser.next() {
           // while let Some(Event::Text(text)) = parser.next() {
           //     return Some(text.to_string());
           //panic!("breakout1");
           dbg!(event);
           return Some(title.to_string());
           //dbg!(event.to_string());
  //         panic!("{}",event);
//           return event.to_string();
           // }
        }
    }
    None
    //Some(title.to_string())
}

let markdown_content = fs::read_to_string(&args.markdownfile).expect("Failed to read Markdown file");
println!("5");
println!(markdown_content);
let post_title = extract_first_heading(markdown_content);
println!(post_title);
// Convert Markdown to HTML
let html_content = markdown_to_html(&markdown_content);
println!(html_content);
println!("6");
