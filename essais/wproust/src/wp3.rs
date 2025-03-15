use clap::Parser;
use pulldown_cmark::{Parser as MdParser, html, Event, Tag};
use reqwest::blocking::Client;
use serde_xml_rs::from_str;
use std::fs;
use std::collections::HashMap;
use toml::Value;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path to the TOML configuration file
    #[clap(short = 'c', long, default_value = "~/wpconfig.toml")]
    config: String,

    /// XML-RPC method to call (e.g., mt.editPost or mt.newPost)
    #[clap(short, long, default_value = "mt.supportedMethods")]
    method: String,

    /// WordPress username (overrides TOML config)
    #[clap(short = 'u', long)]
    username: Option<String>,

    /// WordPress password (overrides TOML config)
    #[clap(short = 's', long)]
    password: Option<String>,

    /// WordPress XML-RPC URL (overrides TOML config)
    #[clap(short = 'b', long)]
    blogurl: Option<String>,

    /// Path to the Markdown file
    #[clap(short = 'f', long, default_value = "sample.md")]
    markdownfile: String,
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = MdParser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn extract_first_heading(markdown: &str) -> Option<String> {
    let parser = MdParser::new(markdown);
    for event in parser {
        if let Event::Start(Tag::Heading(1)) = event {
            if let Some(Event::Text(text)) = parser.next() {
                return Some(text.to_string());
            }
        }
    }
    None
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
        .header("User-Agent", "Rust XML-RPC Client/1.0")
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
    xml_rpc_call(url, "blogger.editPost", params)
}

fn new_post(url: &str, blog_id: &str, username: &str, password: &str, post_title: &str, content: &str) -> Result<String, reqwest::Error> {
    let params = vec![
        blog_id.to_string(),
        username.to_string(),
        password.to_string(),
        post_title.to_string(),
        content.to_string(),
    ];
    xml_rpc_call(url, "blogger.newPost", params)
}

fn read_config(file_path: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();

    // Read the TOML file
    let toml_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return config, // Return empty config if file is missing
    };

    // Parse the TOML content
    let parsed_toml: Value = toml::from_str(&toml_content).unwrap_or(Value::Table(HashMap::new()));

    // Extract WordPress configuration
    if let Some(wordpress) = parsed_toml.get("wordpress") {
        if let Some(username) = wordpress.get("username").and_then(|v| v.as_str()) {
            config.insert("username".to_string(), username.to_string());
        }
        if let Some(password) = wordpress.get("password").and_then(|v| v.as_str()) {
            config.insert("password".to_string(), password.to_string());
        }
        if let Some(blogurl) = wordpress.get("blogurl").and_then(|v| v.as_str()) {
            config.insert("blogurl".to_string(), blogurl.to_string());
        }
        if let Some(postid) = wordpress.get("postid").and_then(|v| v.as_str()) {
            config.insert("postid".to_string(), postid.to_string());
        }
    }

    config
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Read configuration from TOML file
    let config = read_config(&args.config);

    // Use command-line arguments if provided, otherwise fall back to TOML config or defaults
    let username = args.username
        .or_else(|| config.get("username").cloned())
        .unwrap_or_else(|| "admin".to_string());

    let password = args.password
        .or_else(|| config.get("password").cloned())
        .unwrap_or_else(|| "password".to_string());

    let blogurl = args.blogurl
        .or_else(|| config.get("blogurl").cloned())
        .unwrap_or_else(|| "https://example.com/xmlrpc.php".to_string());

    // Read the Markdown file
    let markdown_content = fs::read_to_string(&args.markdownfile).expect("Failed to read Markdown file");

    // Convert Markdown to HTML
    let html_content = markdown_to_html(&markdown_content);

    // Handle different methods
    match args.method.as_str() {
        "editPost" => {
            let postid = config.get("postid")
                .cloned()
                .unwrap_or_else(|| "1".to_string());

            match edit_post(&blogurl, "1", &username, &password, &postid, &html_content) {
                Ok(response) => println!("Post updated successfully! Response: {}", response),
                Err(e) => eprintln!("Failed to update post: {}", e),
            }
        }
        "mt.newPost" => {
            let post_title = extract_first_heading(&markdown_content)
                .unwrap_or_else(|| "15121.snc.test".to_string());

            match new_post(&blogurl, "1", &username, &password, &post_title, &html_content) {
                Ok(response) => println!("Post created successfully! Response: {}", response),
                Err(e) => eprintln!("Failed to create post: {}", e),
            }
        }
        _ => eprintln!("Unsupported method: {}", args.method),
    }
}
