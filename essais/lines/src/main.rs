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
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
//use regex::Regex;
use fancy_regex::Regex;

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
    let modified_content = "[testlink123](https://faz.net)";
    //if atty::is(Stream::Stdin) {
        // Stdin is connected to a terminal (interactive input)
      //  println!("Please enter your input:");
       //let mut buffer = String::new();
     // let mut buffer = io::read_to_string(io::stdin())?;
        //io::stdin().read_line(&mut buffer)?;
       // path = Path::new(io::stdin().read_line(&mut buffer)?;);
        //buffer = path.read_into_string().unwrap();
        //Ok(buffer)
   // } else {
        println!("reading from file");
        // Stdin is redirected (piped input)
       // let mut buffer = io::read_to_string(io::stdin())?;
       let pathfix = "13123.ada.pinghook.md";
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
//}
// fn append_to_csv(filename: &str, data:&str){
//     let mut file = OpenOptions::new()
//         .write(true)
//         .append(true)
//        // .create(true)
//         .open(filename)?;

//     io::Write(file, "{}", data)?;
//     Ok(file)
// }
fn main() -> io::Result<()> {
     let input = read_stdin()?;
     //let modified_content = "[testlink](https://faz.net)";
    let re_h1 = Regex::new(r"# (.+)").unwrap();
    let re_h2 = Regex::new(r"## (.+?)<").unwrap();
    let re_h3 = Regex::new(r"### (.+?)<").unwrap();
    let re_h4 = Regex::new(r"#### (.+)").unwrap();
    let re_h5 = Regex::new(r"##### (.+)").unwrap();
    let re_h6 = Regex::new(r"###### (.+?)<").unwrap();
    let re_p = Regex::new(r"\n(.+?)\n|\n(.+?)$").unwrap();
    let re_t = Regex::new(r"\[(.+?)\]\((.+?)\)").unwrap();
    let re_sub1 = Regex::new(r"- (.+?):[^/]").unwrap();
    let re_subt = Regex::new(r"\t- (.+?):[^/]").unwrap();
    let re_tx = Regex::new(r"\[(.+)\]\((http.?://.+)\)(.+)?").unwrap();
    let re_first = Regex::new(r"#nomatch#").unwrap();
    let re_note = Regex::new(r"\[(.+)\]\((http.?://.+)\)(.+(^#note:))? (#note: (.+)#)?").
    unwrap();
    let re_note_com = Regex::new(r"\[(.+)\]\((http.?://.+)\) (.+[^#note:])? ?(#note:.+)#?").unwrap();

    //let re_h1 = Regex::new(r"- (.+?):[^/]").unwrap();
    //let re_h1 = Regex::new(r"- (.+?):[^/]").unwrap();

   
    let mut modified_content = re_first.replace_all(&input, ";;;;;;$1;$2");
    let mut modified_content = re_sub1.replace_all(&modified_content, ";;;;$1;;;\n");

    let mut modified_content = re_subt.replace_all(&modified_content, ";;;;$1;;;");
    let mut modified_content = re_tx.replace_all(&modified_content, ";;;;;;$1;$2;$3");
    let mut modified_content = re_note_com.replace_all(&modified_content, ";;;;;;$1;$2;$3;$4");
   // let mut modified_content = re_note.replace_all(&modified_content, ";;;;;;$1;$2;$3;$4");

    
    let mut modified_content = re_h5.replace_all(&modified_content, ";;;$1;;;;");
    let mut modified_content = re_h4.replace_all(&modified_content, ";;$1;;;;;");
  //  let mut modified_content = re_tx.replace_all(&modified_content, ";;;;;;$1;$2;$3");

    let mut modified_content = re_h1.replace_all(&modified_content, ";$1;;;;;;");
    let output_file = "pinghook.csv";

   // append_to_csv(output_file, &modified_content);

   // let modified_content2 = "hund";
     //if modified_content == "" {
       // modified_content = std::borrow::Cow::Borrowed//////(modified_content2);
    //}
let header = "id;h1;h2;h3;h4;h5;text;link;description;note\n"; //9
//println!("{}{}{}",header,modified_content,finbody);
println!("{}",modified_content);
let mut file = fs::File::create(output_file).unwrap();

writeln!(file,"{}{}",header,modified_content);
//let mut modified_content = [&header,&modified_content].concat();
//let mut file = fs::File::create(output_file).unwrap();
//file.write_all(modified_content.as_bytes()).unwrap();
//fs::write(output_file, modified_content.println!())?;
    Ok(())
}
