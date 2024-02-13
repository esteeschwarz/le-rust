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
      //  io::stdin().input_conv::read_char(&mut buffer)?;
       // io::read_to_string(&mut buffer)?;

        Ok(buffer)
    } else {
        println!("reading from pipe");
        // Stdin is redirected (piped input)
//        let mut buffer = String::new();
        let mut buffer = io::read_to_string(io::stdin())?;

  //      io::stdin().read_line(&mut buffer)?;
        Ok(buffer)
    }
}

fn main() -> io::Result<()> {
     let input = read_stdin()?;
//     // Process the input as needed (e.g., regex search/replace)
//     // ...

//     // Example: Print the modified content
//     println!("Modified content:\n{}", input);

//     Ok(())
// }


// ///


// fn main() -> io::Result<()> {
    // Read the input file from stdin (piped via bash cat command)
//    let mut input = String::new();
  //  let mut input = io::read_to_string(io::stdin())?;

    let empty = "sample mal katze";
//    let pipe = io::stdin();
   // io::stdin().read_line(&mut input)?;
    // if pipe.unwrap() =="" {
    //     input = empty.to_string();
    // }
    //     else {
    //     io::stdin().read_string(&mut input)?;
    //     }
    
    // Define your regex pattern
    let re_h1 = Regex::new(r"#(.+?)\n").unwrap();
    let re_h2 = Regex::new(r"##(.+?)\n").unwrap();
    let re_h3 = Regex::new(r"###(.+?)\n").unwrap();
    let re_h4 = Regex::new(r"####(.+?)\n").unwrap();
    let re_h5 = Regex::new(r"#####(.+?)\n").unwrap();
    let re_h6 = Regex::new(r"######(.+?)\n").unwrap();
    let re_p = Regex::new(r"^(^#){1,6}(.+?)\n").unwrap();

    // Perform the replacement
    let mut modified_content = re_p.replace_all(&input, "<p>$2</p>");
    // let mut modified_content = re_p.replace_all(&modified_content, "<p>$1</p>");

    let mut modified_content = re_h6.replace_all(&modified_content, "<h6>$1</h6>");
    let mut modified_content = re_h5.replace_all(&modified_content, "<h5>$1</h5>");
    let mut modified_content = re_h4.replace_all(&modified_content, "<h4>$1</h4>");
    let mut modified_content = re_h3.replace_all(&modified_content, "<h3>$1</h3>");
    let mut modified_content = re_h2.replace_all(&modified_content, "<h2>$1</h2>");
    let mut modified_content = re_h1.replace_all(&modified_content, "<h1>$1</h1>");
    let modified_content2 = "hund";
    //modified_content2 = re.replace_all("", "hund");
     if modified_content == "" {
        modified_content = std::borrow::Cow::Borrowed(modified_content2);
    }
   // modified_content = modified_content2;
    // // Write the modified content to a new output file
    // let output_file = env::args().nth(1).expect("Usage: my_regex_tool <output-file>");
    let output_file = "sample-out.txt";
//        if output_file =="" {
//         output_file = "sample_out.txt";
//         output_file = output_file.to_string();
// //        output_file = to_string("sample_out.txt");
//     }
  //  fs::write(output_file, modified_content)?;
println!("{}",modified_content);
//println!("{}",modified_content2);

    Ok(())
}
