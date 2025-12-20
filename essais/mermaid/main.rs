use std::fs;

fn main() {
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
    println!("{}", mermaid);
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

    // Calcule le niveau d'indentation (espaces avant le -)
    let indent_level = line.chars().take_while(|c| c.is_whitespace()).count() / 2;
    let trimmed = line.trim();

    if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        let entry = trimmed[2..].trim();

        if entry.is_empty() {
            continue;
        }

        // Génère un ID unique basé sur les 2 premières lettres
        let base_id = entry
            .chars()
            .filter(|c| c.is_alphabetic())
            .take(2)
            .collect::<String>()
            .to_uppercase();

        // Gère les doublons en ajoutant un numéro
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

// Génère le graphe Mermaid
let mut graph_lines = Vec::new();
let mut parent_stack: Vec<&ListItem> = Vec::new();

for item in &items {
    // Ajuste la pile des parents selon le niveau d'indentation
    while parent_stack.len() > item.indent_level {
        parent_stack.pop();
    }

    // Crée le nœud
    graph_lines.push(format!("    {}[{}]", item.node_id, item.text));

    // Si on a un parent, crée la connexion
    if let Some(parent) = parent_stack.last() {
        graph_lines.push(format!("    {} --> {}", parent.node_id, item.node_id));
    }

    // Ajoute cet item à la pile pour ses enfants potentiels
    parent_stack.push(item);
}

// Assemble le code Mermaid
let mut result = String::from("```mermaid\ngraph TD\n");
result.push_str(&graph_lines.join("\n"));
result.push_str("\n```");

result


}

// Version qui lit depuis un fichier
fn md_file_to_mermaid(filepath: &str) -> Result<String, std::io::Error> {
let content = fs::read_to_string(filepath)?;
Ok(md_list_to_mermaid_hierarchical(&content))
}

// Version alternative : style LR (Left to Right) au lieu de TD
fn md_list_to_mermaid_lr(markdown: &str) -> String {
let result = md_list_to_mermaid_hierarchical(markdown);
result.replace("graph TD", "graph LR")
}

#[cfg(test)]
mod tests {
use super::*;


#[test]
fn test_hierarchical_conversion() {
    let input = r#"
```

- Python
  - NumPy
  - Pandas
- Rust
  "#;
  let output = md_list_to_mermaid_hierarchical(input);

  
    assert!(output.contains("PY[Python]"));
    assert!(output.contains("NU[NumPy]"));
    assert!(output.contains("PA[Pandas]"));
    assert!(output.contains("PY --> NU"));
    assert!(output.contains("PY --> PA"));
  

  }

  #[test]
  fn test_deep_nesting() {
  let input = r#"
- A
  - B
    - C
      - D
        "#;
        let output = md_list_to_mermaid_hierarchical(input);

        assert!(output.contains("A –> B"));
        assert!(output.contains("B –> C"));
        assert!(output.contains("C –> D"));
        }

    #[test]
    fn test_duplicate_names() {
    let input = r#"
- Python
  - Python Tools
    "#;
    let output = md_list_to_mermaid_hierarchical(input);

    
    // Devrait avoir PY et PY2 pour éviter les doublons
    assert!(output.contains("PY[Python]"));
    assert!(output.contains("PY2[Python Tools]"));
    

    }
    }