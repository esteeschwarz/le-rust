use std::path::Path;
use std::fs;

fn main() {
    let path = Path::new(".");
    match list_files_in_directory(path) {
        Ok(files_string) => println!("{}", files_string),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn list_files_in_directory(path: &Path) -> Result<String, std::io::Error> {
    let mut files_string = String::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
           files_string.push_str(&format!("{}{}{}{}{}{}\n", "<a href='",path.display(),"'>","",path.display(),"</a>"));
        }
    }
    Ok(files_string)
}
