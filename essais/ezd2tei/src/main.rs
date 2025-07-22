use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn convert_txt_to_xml(input_path: &str, output_path: &str) -> io::Result<()> {
    // Read input file
    let content = fs::read_to_string(input_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Prepare output file
    let mut output = fs::File::create(output_path)?;
    
    // Write XML header
    writeln!(output, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(output, r#"<TEI xml:id="insertID" xml:lang="ger" xmlns="http://www.tei-c.org/ns/1.0">"#)?;
    writeln!(output, r#"  <teiHeader>"#)?;
    writeln!(output, r#"    <fileDesc>"#)?;
    writeln!(output, r#"      <titleStmt>"#)?;
    
    // Process metadata
    let mut title = "".to_string();
    let mut subtitle = "".to_string();
    let mut author = "".to_string();
    let mut cast_list = Vec::new();
    
    let mut in_body = false;
    let mut current_act = None;
    let mut current_scene = None;
    
    for line in lines {
        if line.starts_with("@title ") {
            title = line[7..].trim().to_string();
            writeln!(output, r#"        <title type="main">{}</title>"#, title)?;
        } else if line.starts_with("@subtitle ") {
            subtitle = line[10..].trim().to_string();
            writeln!(output, r#"        <title type="sub">{}</title>"#, subtitle)?;
        } else if line.starts_with("@author ") {
            author = line[8..].trim().to_string();
            writeln!(output, r#"        <author>{}</author>"#, author)?;
        } else if line.starts_with("^") {
            // Cast list
            if line == "^Personen." {
                cast_list.clear();
            } else if line.starts_with("^") && line.len() > 1 {
                cast_list.push(line[1..].trim().to_string());
            }
        } else if line.starts_with("#") {
            // Act
            if !in_body {
                // Close header and start body
                writeln!(output, r#"      </titleStmt>"#)?;
                writeln!(output, r#"      <publicationStmt>"#)?;
                writeln!(output, r#"        <publisher xml:id="dracor">DraCor</publisher>"#)?;
                writeln!(output, r#"        <idno type="URL">https://dracor.org</idno>"#)?;
                writeln!(output, r#"        <availability>"#)?;
                writeln!(output, r#"          <licence>"#)?;
                writeln!(output, r#"            <ab>CC0 1.0</ab>"#)?;
                writeln!(output, r#"            <ref target="https://creativecommons.org/publicdomain/zero/1.0/">"#)?;
                writeln!(output, r#"              Licence</ref>"#)?;
                writeln!(output, r#"          </licence>"#)?;
                writeln!(output, r#"        </availability>"#)?;
                writeln!(output, r#"      </publicationStmt>"#)?;
                writeln!(output, r#"      <sourceDesc>"#)?;
                writeln!(output, r#"        <bibl type="digitalSource">"#)?;
                writeln!(output, r#"          <name>ENTER SOURCE NAME HERE</name>"#)?;
                writeln!(output, r#"          <idno type="URL">ENTER SOURCE URL HERE</idno>"#)?;
                writeln!(output, r#"          <availability status="free">"#)?;
                writeln!(output, r#"            <p>In the public domain.</p>"#)?;
                writeln!(output, r#"          </availability>"#)?;
                writeln!(output, r#"        </bibl>"#)?;
                writeln!(output, r#"      </sourceDesc>"#)?;
                writeln!(output, r#"    </fileDesc>"#)?;
                writeln!(output, r#"    <profileDesc>"#)?;
                writeln!(output, r#"      <particDesc>"#)?;
                writeln!(output, r#"        <listPerson>"#)?;
                
                // Write cast list as persons
                for person in &cast_list {
                    let sex = if person == "Eva" || person == "Zilla" {
                        "FEMALE"
                    } else {
                        "MALE"
                    };
                    let id = person.to_lowercase().replace(" ", "_");
                    writeln!(output, r#"          <person sex="{}" xml:id="{}">"#, sex, id)?;
                    writeln!(output, r#"            <persName>{}</persName>"#, person)?;
                    writeln!(output, r#"          </person>"#)?;
                }
                
                writeln!(output, r#"        </listPerson>"#)?;
                writeln!(output, r#"      </particDesc>"#)?;
                writeln!(output, r#"    </profileDesc>"#)?;
                writeln!(output, r#"    <revisionDesc>"#)?;
                writeln!(output, r#"      <listChange>"#)?;
                writeln!(output, r#"        <change when="2025-03-21">DESCRIBE CHANGE</change>"#)?;
                writeln!(output, r#"      </listChange>"#)?;
                writeln!(output, r#"    </revisionDesc>"#)?;
                writeln!(output, r#"  </teiHeader>"#)?;
                writeln!(output, r#"  <standOff>"#)?;
                writeln!(output, r#"    <listEvent>"#)?;
                writeln!(output, r#"      <event type="print" when="2025">"#)?;
                writeln!(output, r#"        <desc/>"#)?;
                writeln!(output, r#"      </event>"#)?;
                writeln!(output, r#"      <event type="premiere" when="2025">"#)?;
                writeln!(output, r#"        <desc/>"#)?;
                writeln!(output, r#"      </event>"#)?;
                writeln!(output, r#"      <event type="written" when="2025">"#)?;
                writeln!(output, r#"        <desc/>"#)?;
                writeln!(output, r#"      </event>"#)?;
                writeln!(output, r#"    </listEvent>"#)?;
                writeln!(output, r#"    <listRelation>"#)?;
                writeln!(output, r#"      <relation active="INSERT" name="wikidata" passive="INSERT"/>"#)?;
                writeln!(output, r#"    </listRelation>"#)?;
                writeln!(output, r#"  </standOff>"#)?;
                writeln!(output, r#"  <text>"#)?;
                writeln!(output, r#"    <front>"#)?;
                writeln!(output, r#"      <castList>"#)?;
                writeln!(output, r#"        <head>Personen.</head>"#)?;
                
                // Write cast list items
                for person in &cast_list {
                    writeln!(output, r#"        <castItem>{}</castItem>"#, person)?;
                }
                
                writeln!(output, r#"        <castItem>"#)?;
                writeln!(output, r#"        </castItem>"#)?;
                writeln!(output, r#"      </castList>"#)?;
                writeln!(output, r#"    </front>"#)?;
                writeln!(output, r#"    <body>"#)?;
                in_body = true;
            }
            
            // Close previous scene and act if they exist
            if let Some(scene) = current_scene.take() {
                writeln!(output, "{}", scene)?;
                writeln!(output, r#"        </div>"#)?;
            }
            if let Some(act) = current_act.take() {
                writeln!(output, "{}", act)?;
                writeln!(output, r#"      </div>"#)?;
            }
            
            // Start new act
            let act_title = line[1..].trim();
            writeln!(output, r#"      <div type="act">"#)?;
            writeln!(output, r#"        <head>{}</head>"#, act_title)?;
            current_act = Some(format!(r#"      <div type="act">"#));
        } else if line.starts_with("##") {
            // Scene
            if let Some(scene) = current_scene.take() {
                writeln!(output, "{}", scene)?;
                writeln!(output, r#"        </div>"#)?;
            }
            
            let scene_title = line[2..].trim();
            writeln!(output, r#"        <div type="scene">"#)?;
            writeln!(output, r#"          <head>{}</head>"#, scene_title)?;
            current_scene = Some(format!(r#"        <div type="scene">"#));
        } else if line.starts_with("$") {
            // Stage direction
            let stage = line[1..].trim();
            if !stage.is_empty() {
                writeln!(output, r#"          <stage>{}</stage>"#, stage)?;
            }
        } else if line.starts_with("@") {
            // Speaker
            let speaker_parts: Vec<&str> = line[1..].splitn(2, '.').collect();
            if speaker_parts.len() == 2 {
                let speaker = speaker_parts[0].trim();
                let speech = speaker_parts[1].trim();
                
                // Handle page numbers (::)
                let speech_with_pb = speech.replace("::", "</p><pb n=\"");
                let speech_with_pb = speech_with_pb.replace(" ", "\"/>\n            <p>");
                
                writeln!(output, r##" <sp who="{}">"##, speaker.to_lowercase())?;
                writeln!(output, r#"            <speaker>{}</speaker>"#, speaker)?;
                writeln!(output, r#"            <p>{}</p>"#, speech_with_pb)?;
                writeln!(output, r#"          </sp>"#)?;
            }
        } else if line.starts_with("(") && line.ends_with(")") {
            // Inline stage direction
            let stage = &line[1..line.len()-1];
            writeln!(output, r#"            <stage>{}</stage>"#, stage)?;
        } else if !line.trim().is_empty() {
            // Regular text line (continuation of speech)
            let line_with_pb = line.replace(":", "<pb n=\"dumm\"/>");
//            let line_with_pb = line_with_pb.replace(" ", "\"/>            <p>");
            let line_p = line;
            writeln!(output, r#"            <p>{}</p>"#, line_with_pb)?;
          //  writeln!(output, r#"            <p>{}</p>"#, line_p)?;
        }
    }
    
    // Close any open tags
    if let Some(scene) = current_scene.take() {
        writeln!(output, "{}", scene)?;
        writeln!(output, r#"        </div>"#)?;
    }
    if let Some(act) = current_act.take() {
        writeln!(output, "{}", act)?;
        writeln!(output, r#"      </div>"#)?;
    }
    
    // Close body and document
    writeln!(output, r#"    </body>"#)?;
    writeln!(output, r#"  </text>"#)?;
    writeln!(output, r#"</TEI>"#)?;
    
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.txt> <output.xml>", args[0]);
        std::process::exit(1);
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    
    if !Path::new(input_path).exists() {
        eprintln!("Input file does not exist: {}", input_path);
        std::process::exit(1);
    }
    
    match convert_txt_to_xml(input_path, output_path) {
        Ok(_) => println!("Successfully converted {} to {}", input_path, output_path),
        Err(e) => eprintln!("Error: {}", e),
    }
}