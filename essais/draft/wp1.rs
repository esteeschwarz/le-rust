use pulldown_cmark::{Parser, html};
use reqwest::blocking::Client;
use serde_xml_rs::from_str;
use std::fs;
use std::env;

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn xml_rpc_call(url: &str, method_name: &str, params: Vec<String>) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let request_body = format!(
        r#"<?xml version="1.0"?>
        <methodCall>
            <methodName>{}</methodName>
            <params>
                {}
            </params>
        </methodCall>"#,
        method_name,
        params.iter().map(|p| format!("<param><value><string>{}</string></value></param>", p)).collect::<String>()
    );

    let response = client
        .post(url)
        .header("Content-Type", "text/xml")
        .body(request_body)
        .send()?
        .text()?;

    Ok(response)
}

fn edit_post(url: &str, blog_id: &str, username: &str, password: &str, post_id: &str, content: &str) -> Result<String, reqwest::Error> {
    let params = vec![
        blog_id.to_string(),
        username.to_string(),
        password.to_string(),
        post_id.to_string(),
        content.to_string(),
    ];
    xml_rpc_call(url, "mt.editPost", params)
}

fn main() {
    // Fetch command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 7 {
        eprintln!("Usage: {} <postid> <method> <username> <password> <blogurl> <markdownfile>", args[0]);
        std::process::exit(1);
    }

    // Parse arguments
    let post_id = &args[1];
    let method = &args[2];
    let username = &args[3];
    let password = &args[4];
    let blog_url = &args[5];
    let markdown_file = &args[6];

    // Read the Markdown file
    let markdown_content = fs::read_to_string(markdown_file).expect("Failed to read Markdown file");

    // Convert Markdown to HTML
    let html_content = markdown_to_html(&markdown_content);

    // Update the post with the generated HTML
    match edit_post(blog_url, "1", username, password, post_id, &html_content) {
        Ok(response) => println!("Post updated successfully! Response: {}", response),
        Err(e) => eprintln!("Failed to update post: {}", e),
    }
}