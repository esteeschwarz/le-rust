use clap::Parser;
use pulldown_cmark::{Parser as MdParser, html, Event, Tag};
use reqwest::blocking::Client;
use serde_xml_rs::from_str;
use std::fs;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path to the TOML configuration file
    #[clap(short = 'c', long, default_value = "~/wpconfig.toml")]
    config: String,

    /// XML-RPC method to call (e.g., mt.editPost or mt.newPost)
    #[clap(short, long, default_value = "mt.getRecentPostTitles")]
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


    /// post ID
    #[clap(short = 'i', long, default_value = "3000")]
    postid: Option<String>
}

#[derive(Debug, Deserialize)]
struct Config {
    wordpress: WordPressConfig,
}

#[derive(Debug, Deserialize)]
struct WordPressConfig {
    username: String,
    password: String,
    blogurl: String,
    // postid: Option<String>
    postid: String
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = MdParser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
/// Extracts the first level 1 heading (`#`) from the Markdown content.
fn extract_first_heading(markdown: &str) -> Option<String> {
    for line in markdown.lines() {
        if line.starts_with("# ") {
            return Some(line.trim_start_matches("# ").trim().to_string());
        }
    }
    None
}
/// Removes the first level 1 heading from the Markdown content.
fn remove_first_h1(markdown: &str) -> String {
    let mut lines = markdown.lines();
    let mut result = String::new();

    // Skip the first line if it's a level 1 heading
    if let Some(first_line) = lines.next() {
        if !first_line.starts_with("# ") {
            result.push_str(first_line);
            result.push('\n');
        }
    }

    // Append the remaining lines
    for line in lines {
        result.push_str(line);
        result.push('\n');
    }

    result
}

fn extract_first_heading_dep(markdown: &str) -> Option<String> {
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
        params.iter().map(|p| format!("<param>{}</param>", p)).collect::<String>()
    );
    // original XMLRPC: <string><![CDATA[samplestring]]></string>
    
    println!("body");
    println!("{}",request_body);

    let response = client
        .post(url)
        .header("Content-Type", "text/xml")
        .header("User-Agent", "Rust XML-RPC Client/1.0")
        .body(request_body)
        .send()?
        .text()?;

    Ok(response)
}

fn edit_post(url: &str, post_id: &str, username: &str, password: &str,  post_title: &str, content: &str, publish: bool) -> Result<String, reqwest::Error> {
    let params = vec![
       // format!("<value><string><![CDATA[{}]]></string></value>",blog_id.to_string()),
        format!("<value><string><![CDATA[{}]]></string></value>",post_id.to_string()),
        format!("<value><string><![CDATA[{}]]></string></value>",username.to_string()),
        format!("<value><string><![CDATA[{}]]></string></value>",password.to_string()),
        format!("<struct><member><name>description</name><value><string><![CDATA[{}]]></string></value></member><member><name>title</name><value><string><![CDATA[{}]]></string></value></member></struct>",content.to_string(),post_title.to_string()),
        format!("<value><boolean>{}</boolean></value>", if publish { 1 } else { 0 })
  //     blog_id.to_string(),
    //     username.to_string(),
    //     password.to_string(),
    //     post_id.to_string(),
    //     content.to_string(),
    ];
    xml_rpc_call(url, "metaWeblog.editPost", params)
}

fn new_post(url: &str, blog_id: i32, username: &str, password: &str, post_title: &str, content: &str, publish: bool ) -> Result<String, reqwest::Error> {
   // let blog_id = 1;
    let params = vec![
        //"1".to_string(),
        format!("<value><string><![CDATA[{}]]></string></value>",blog_id.to_string()),
        format!("<value><string><![CDATA[{}]]></string></value>",username.to_string()),
        format!("<value><string><![CDATA[{}]]></string></value>",password.to_string()),
        format!("<struct><member><name>description</name><value><string><![CDATA[{}]]></string></value></member><member><name>title</name><value><string><![CDATA[{}]]></string></value></member></struct>",content.to_string(),post_title.to_string()),
        format!("<value><boolean>{}</boolean></value>", if publish { 1 } else { 0 }),


    ];
    xml_rpc_call(url, "metaWeblog.newPost", params)
}

fn read_config(file_path: &str) -> Option<Config> {
    // Read the TOML file
    let toml_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return None, // Return None if file is missing
    };

    // Parse the TOML content into the Config struct
    toml::from_str(&toml_content).ok()
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Read configuration from TOML file
    let config = read_config(&args.config);
    println!("1");

    // Use command-line arguments if provided, otherwise fall back to TOML config or defaults
    let username = args.username
        .or_else(|| config.as_ref().map(|c| c.wordpress.username.clone()))
        .unwrap_or_else(|| "admin".to_string());
    println!("2");   

    let password = args.password
        .or_else(|| config.as_ref().map(|c| c.wordpress.password.clone()))
        .unwrap_or_else(|| "password".to_string());
    println!("3");
    let blogurl = args.blogurl
        .or_else(|| config.as_ref().map(|c| c.wordpress.blogurl.clone()))
        .unwrap_or_else(|| "http://mini12/ap/wpost/xmlrpc.php".to_string());
    let postid = args.postid
        .or_else(|| config.as_ref().map(|c| c.wordpress.postid.clone()))
        .unwrap_or_else(|| "2000".to_string());
    println!("4");
    // Read the Markdown file
    let markdown_content = fs::read_to_string(&args.markdownfile).expect("Failed to read Markdown file");
    println!("5");

    // Remove the first level 1 heading from the Markdown content
    let markdown_without_h1 = remove_first_h1(&markdown_content);

    // Convert the modified Markdown content to HTML
    let html_content = markdown_to_html(&markdown_without_h1);

    // Convert Markdown to HTML
    // let html_content = markdown_to_html(&markdown_content);
    println!("6");
    // Handle different methods
    match args.method.as_str() {
        "editPost" => {
            let postid = config.and_then(|c| Some(c.wordpress.postid))
                .unwrap_or_else(|| "2000".to_string());
            let post_title = extract_first_heading(&markdown_content)
                .unwrap_or_else(|| "untitled-x".to_string());

            match edit_post(&blogurl,&postid, &username, &password, &post_title, &html_content,true) {
                Ok(response) => println!("Post updated successfully! Response: {}", response),
                Err(e) => eprintln!("Failed to update post: {}", e),
            }
        }
        "newPost" => {
            let post_title = extract_first_heading(&markdown_content)
                .unwrap_or_else(|| "15121.silver.snc.test".to_string());

            match new_post(&blogurl, 1, &username, &password, &post_title, &html_content, true) {
                Ok(response) => println!("Post created successfully! Response: {}", response),
                Err(e) => eprintln!("Failed to create post: {}", e),
            }
        }
        _ => eprintln!("Unsupported method: {}", args.method),
    }
}
