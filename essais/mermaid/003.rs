use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Exemple 1: Depuis une string
    let input = r#"
- Python
  - NumPy
  - Pandas
- JavaScript
  - TypeScript
  - React
    - Next.js
- Rust
  - Tokio
"#;
    
    let mermaid = md_list_to_mermaid_hierarchical(input);
    println!("=== Exemple depuis string ===\n{}\n", mermaid);
    
    // Exemple 2: Depuis un fichier
    match process_md_file("input.md", "output.md") {
        Ok(output_path) => println!("✅ Fichier sauvegardé: {}", output_path),
        Err(e) => eprintln!("❌ Erreur: {}", e),
    }
    
    // Exemple 3: Ajouter au fichier existant
    match append_mermaid_to_md("notes.md") {
        Ok(_) => println!("✅ Diagramme Mermaid ajouté à notes.md"),
        Err(e) => eprintln!("❌ Erreur: {}", e),
    }
}

#[derive(Debug)]
struct ListItem {
    text: String,
    indent_level: usize,
    node_id: String,
}

fn md_list_to_mermaid_hierarchical(markdown: &str) -> String {
    let mut items = Vec::new();
    let mut node_counter = std::collections::HashMap::new();
    
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
    
    let mut result = String::from("```mermaid\ngraph TD\n");
    result.push_str(&graph_lines.join("\n"));
    result.push_str("\n```");
    
    result
}

/// Lit un fichier .md, extrait les listes, génère Mermaid, et sauvegarde dans un nouveau fichier
fn process_md_file(input_path: &str, output_path: &str) -> Result<String, std::io::Error> {
    // Lit le fichier source
    let content = fs::read_to_string(input_path)?;
    
    // Génère le code Mermaid
    let mermaid = md_list_to_mermaid_hierarchical(&content);
    
    // Crée le contenu du fichier de sortie
    let output_content = format!(
        "# Diagramme généré depuis {}\n\n{}\n\n## Source originale\n\n{}",
        input_path, mermaid, content
    );
    
    // Écrit dans le fichier de sortie
    fs::write(output_path, output_content)?;
    
    Ok(output_path.to_string())
}

/// Ajoute le diagramme Mermaid à la fin du fichier existant
fn append_mermaid_to_md(filepath: &str) -> Result<(), std::io::Error> {
    // Lit le contenu existant
    let content = fs::read_to_string(filepath)?;
    
    // Génère le Mermaid
    let mermaid = md_list_to_mermaid_hierarchical(&content);
    
    // Ouvre le fichier en mode append
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(filepath)?;
    
    // Ajoute le diagramme
    writeln!(file, "\n\n## Diagramme hiérarchique\n\n{}", mermaid)?;
    
    Ok(())
}

/// Remplace les listes dans le fichier par le diagramme Mermaid
fn replace_lists_with_mermaid(filepath: &str) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(filepath)?;
    let mermaid = md_list_to_mermaid_hierarchical(&content);
    
    // Supprime les listes originales et les remplace par le diagramme
    let lines: Vec<&str> = content.lines().collect();
    let mut new_content = String::new();
    let mut in_list = false;
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            if !in_list {
                // Première ligne de liste - insère le Mermaid
                new_content.push_str(&mermaid);
                new_content.push('\n');
                in_list = true;
            }
            // Saute les lignes de liste
        } else {
            in_list = false;
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    
    fs::write(filepath, new_content)?;
    Ok(())
}

/// Crée un fichier .mmd (Mermaid) pur
fn save_as_mermaid_file(input_path: &str, output_path: &str) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(input_path)?;
    let mermaid = md_list_to_mermaid_hierarchical(&content);
    
    // Enlève les backticks pour un fichier .mmd pur
    let mermaid_pure = mermaid
        .replace("```mermaid\n", "")
        .replace("\n```", "");
    
    fs::write(output_path, mermaid_pure)?;
    Ok(())
}

/// Interface CLI simple
fn cli_usage() {
    println!("Usage:");
    println!("  md-to-mermaid <input.md> <output.md>    # Génère un nouveau fichier");
    println!("  md-to-mermaid --append <file.md>        # Ajoute au fichier existant");
    println!("  md-to-mermaid --replace <file.md>       # Remplace les listes");
    println!("  md-to-mermaid --mmd <input.md> <out.mmd> # Crée un fichier .mmd");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_file_processing() {
        // Crée un fichier de test
        let test_input = "test_input.md";
        let test_output = "test_output.md";
        
        fs::write(test_input, "- Item A\n  - Item B\n- Item C").unwrap();
        
        let result = process_md_file(test_input, test_output);
        assert!(result.is_ok());
        
        let output_content = fs::read_to_string(test_output).unwrap();
        assert!(output_content.contains("```mermaid"));
        assert!(output_content.contains("IT[Item A]"));
        
        // Nettoyage
        fs::remove_file(test_input).ok();
        fs::remove_file(test_output).ok();
    }
}