use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
    
    let mermaid = md_list_to_mermaid(&buffer);
    println!("{}", mermaid);
}

struct ListItem {
    text: String,
    indent_level: usize,
    node_id: String,
}

fn md_list_to_mermaid(markdown: &str) -> String {
    let mut items = Vec::new();
    let mut node_counter = HashMap::new();
    
    for line in markdown.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let indent_level = line.chars().take_while(|c| c.is_whitespace()).count() / 2;
        let trimmed = line.trim();
        
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let entry = trimmed[2..].trim();
            
            if entry.is_empty() {
                continue;
            }
            
            let base_id = entry
                .chars()
                .filter(|c| c.is_alphabetic())
                .take(2)
                .collect::<String>()
                .to_uppercase();
            
            let count = node_counter.entry(base_id.clone()).or_insert(0);
            *count += 1;
            
            let node_id = if *count == 1 {
                base_id
            } else {
                format!("{}{}", base_id, count)
            };
            
            items.push(ListItem {
                text: entry.to_string(),
                indent_level,
                node_id,
            });
        }
    }
    
    let mut graph_lines = Vec::new();
    let mut parent_stack: Vec<&ListItem> = Vec::new();
    
    for item in &items {
        while parent_stack.len() > item.indent_level {
            parent_stack.pop();
        }
        
        graph_lines.push(format!("    {}[{}]", item.node_id, item.text));
        
        if let Some(parent) = parent_stack.last() {
            graph_lines.push(format!("    {} --> {}", parent.node_id, item.node_id));
        }
        
        parent_stack.push(item);
    }
    
    // let mut result = String::from("```mermaid\ngraph TD\n");
    // result.push_str(&graph_lines.join("\n"));
    // result.push_str("\n```");
    let mut result = String::from("graph TD\n");
    result.push_str(&graph_lines.join("\n"));
    result.push_str("\n");
    
    result
}