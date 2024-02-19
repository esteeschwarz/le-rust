fn sam() {
    println!("Hello, world!");
}

// fn lines(){
// let path = "sample.md";
//     let string = path.read_lines_into_string().unwrap();

// }
fn hello() {
    println!("Hello, world!");
}

// main.rs

use std::env;
use std::fs;
use std::io::{self, Write};
use regex::Regex;
use input_conv::read_string;
use input_conv::read_char;
//use std::io::{self, Read};
use atty::Stream;
use read_lines_into::traits::*;
use std::path::Path;
use read_lines_into::traits::*;
 
// Choose any existing text file

///////////////////////////
/// /// pipe check

// main.rs


fn read_stdin() -> io::Result<String> {
    let modified_content = "nostrings";
    if atty::is(Stream::Stdin) {
        // Stdin is connected to a terminal (interactive input)
        println!("Please enter your input:");
       let mut buffer = String::new();
     // let mut buffer = io::read_to_string(io::stdin())?;
        io::stdin().read_line(&mut buffer)?;
        path = Path::new(io::stdin().read_line(&mut buffer)?;);
        buffer = path.read_into_string().unwrap();
        Ok(buffer)
    } else {
        println!("reading from pipe");
        // Stdin is redirected (piped input)
       // let mut buffer = io::read_to_string(io::stdin())?;
       let pathfix = "../pipe-md/sample.md";
       let path = if let Some(path) = std::env::args().nth(1) {
        path
    }   
    else {
         pathfix.to_string()    
    };

       let mut buffer = Path::new(&path).read_lines_into_string().unwrap();

  //      io::stdin().read_line(&mut buffer)?;
        Ok(buffer)
    }
}

fn main() -> io::Result<()> {
     let input = read_stdin()?;
    let re_h1 = Regex::new(r"# ?(.+?)<").unwrap();
    let re_h2 = Regex::new(r"## (.+?)<").unwrap();
    let re_h3 = Regex::new(r"### (.+?)<").unwrap();
    let re_h4 = Regex::new(r"#### (.+?)<").unwrap();
    let re_h5 = Regex::new(r"##### (.+?)<").unwrap();
    let re_h6 = Regex::new(r"###### (.+?)<").unwrap();
    let re_p = Regex::new(r"\n(.+?)\n|\n(.+?)$").unwrap();
    // Perform the replacement
    let mut modified_content = re_p.replace_all(&input, "<p>$1$2</p>");
    let mut modified_content = re_h6.replace_all(&modified_content, "<h6>$1</h6><");
    let mut modified_content = re_h5.replace_all(&modified_content, "<h5>$1</h5><");
    let mut modified_content = re_h4.replace_all(&modified_content, "<h4>$1</h4><");
    let mut modified_content = re_h3.replace_all(&modified_content, "<h3>$1</h3><");
    let mut modified_content = re_h2.replace_all(&modified_content, "<h2>$1</h2><");
    let mut modified_content = re_h1.replace_all(&modified_content, "<h1>$1</h1><");
    let modified_content2 = "hund";
     if modified_content == "" {
        modified_content = std::borrow::Cow::Borrowed(modified_content2);
    }
    let output_file = "sample-out.txt";
let header = "<!DOCTYPE html><html><head><meta name='viewport' content='width=device-width, initial-scale=1.0'><meta charset='utf-8' /><link rel='stylesheet' type='text/css' href='https://ada-sub.dh-index.org/school/css/style.css' /><title>14074.le-rust</title></head><body>";
let finbody = "</body></html>";
println!("{}{}{}",header,modified_content,finbody);
    Ok(())
}
