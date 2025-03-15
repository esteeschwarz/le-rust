use clap::Parser;
   use pulldown_cmark::{Parser as MdParser, html};
   use reqwest::blocking::Client;
   use serde_xml_rs::from_str;
   use std::fs;

   #[derive(Parser, Debug)]
   #[clap(about, version, author)]
   struct Args {
       /// ID of the post to edit
       #[clap(short, long)]
       postid: String,

       /// XML-RPC method to call (e.g., mt.editPost)
       #[clap(short, long)]
       method: String,

       /// WordPress username
       #[clap(short, long)]
       username: String,

       /// WordPress password
       #[clap(short, long)]
       password: String,

       /// WordPress XML-RPC URL
       #[clap(short, long)]
       blogurl: String,

       /// Path to the Markdown file
       #[clap(short, long)]
       markdownfile: String,
   }

   fn markdown_to_html(markdown: &str) -> String {
       let parser = MdParser::new(markdown);
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
       // Parse command-line arguments
       let args = Args::parse();

       // Read the Markdown file
       let markdown_content = fs::read_to_string(&args.markdownfile).expect("Failed to read Markdown file");

       // Convert Markdown to HTML
       let html_content = markdown_to_html(&markdown_content);

       // Update the post with the generated HTML
       match edit_post(&args.blogurl, "1", &args.username, &args.password, &args.postid, &html_content) {
           Ok(response) => println!("Post updated successfully! Response: {}", response),
           Err(e) => eprintln!("Failed to update post: {}", e),
       }
   }