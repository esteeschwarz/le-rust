use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use csv::Reader;
use regex::Regex;

fn main() -> io::Result<()> {
    // Create sample data if it doesn't exist
    if !Path::new("sitedb.csv").exists() {
        create_sample_data()?;
    }

    // Read the templates
    let page_template = fs::read_to_string("page-template.html")?;
    let index_template = fs::read_to_string("index-template.html")?;

    // Read the CSV data
    let mut rdr = Reader::from_path("sitedb.csv")?;
    let headers = rdr.headers()?.clone();
    let mut records = vec![];

    // Process each record
    for result in rdr.deserialize() {
        let record: SiteRecord = result?;
        records.push(record);
    }

    // Create individual pages
    for (id, record) in records.iter().enumerate() {
        let id = id + 1;
        // Get content and link columns
        let content_links = get_content_links(&headers, record);
        let filled_template = fill_template(&page_template, record, &content_links);
        let output_filename = format!("page-{}.html", id);
        fs::write(&output_filename, filled_template)?;
    }

    // Create index page
    create_index_page(&records, &index_template)?;

    Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct SiteRecord {
    id: u32,
    title_site: String,
    h1_top: String,
    h4_navigate_title: String,
    p_class: String,
    p_tutor_term: String,
    h1_title_content: String,
    h4_date: String,
    p_abstract: String,
    td_paper_title: String,
    td_paper_href: String,
    // Added sample content/link fields
    content_1: String,
    link_1: String,
    content_2: String,
    link_2: String,
    content_about: String,
    link_about: String,
}

fn get_content_links(headers: &csv::StringRecord, record: &SiteRecord) -> Vec<(String, String)> {
    let mut content_links = Vec::new();
    
    // Find all columns starting with "content" or "link"
    for (i, header) in headers.iter().enumerate() {
        if header.starts_with("content") {
            // Try to find matching link column
            let link_header = header.replace("content", "link");
            if let Some(link_idx) = headers.iter().position(|h| h == link_header) {
                // Get the values using reflection (simplified approach)
                let content_value = match header {
                    "content_1" => &record.content_1,
                    "content_2" => &record.content_2,
                    "content_3" => &record.content_3,
                    _ => "",
                };
                
                let link_value = match link_header.as_str() {
                    "link_1" => &record.link_1,
                    "link_2" => &record.link_2,
                    "link_3" => &record.link_3,
                    _ => "",
                };
                
                if !content_value.is_empty() && !link_value.is_empty() {
                    content_links.push((content_value.to_string(), link_value.to_string()));
                }
            }
        }
    }
    
    content_links
}

fn fill_template(template: &str, record: &SiteRecord, content_links: &[(String, String)]) -> String {
    let mut result = template.to_string();
    
    // Replace standard fields
    let re = Regex::new(r"\{\%(\w+)\%\}").unwrap();
    
    for cap in re.captures_iter(template) {
        let field = &cap[1];
        let value = match field {
            "id" => record.id.to_string(),
            "title.site" => record.title_site.clone(),
            "h1.top" => record.h1_top.clone(),
            "h4.navigate.title" => record.h4_navigate_title.clone(),
            "p.class" => record.p_class.clone(),
            "p.tutor.term" => record.p_tutor_term.clone(),
            "h1.title.content" => record.h1_title_content.clone(),
            "h4.date" => record.h4_date.clone(),
            "p.abstract" => record.p_abstract.clone(),
            "td.paper.title" => record.td_paper_title.clone(),
            "td.paper.href" => record.td_paper_href.clone(),
            _ => "".to_string(),
        };
        
        let pattern = format!("{{%{}%}}", field);
        result = result.replace(&pattern, &value);
    }
    
    // Generate content links list if template has the placeholder
    if template.contains("{%content.links%}") {
        let mut links_html = String::new();
        for (content, link) in content_links {
            links_html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>\n",
                link, content
            ));
        }
        result = result.replace("{%content.links%}", &links_html);
    }
    
    result
}

fn create_index_page(records: &[SiteRecord], index_template: &str) -> io::Result<()> {
    let mut list_items = String::new();
    
    for (id, record) in records.iter().enumerate() {
        let id = id + 1;
        list_items.push_str(&format!(
            "<li><a href=\"page-{}.html\">{}</a></li>\n",
            id, record.title_site
        ));
    }
    
    let filled_template = index_template.replace("{%page.list%}", &list_items);
    fs::write("index.html", filled_template)?;
    Ok(())
}

fn create_sample_data() -> io::Result<()> {
    let mut wtr = csv::Writer::from_writer(File::create("sitedb.csv")?);
    
    // Write header with content/link fields
    wtr.write_record(&[
        "id", "title_site", "h1_top", "h4_navigate_title", "p_class", 
        "p_tutor_term", "h1_title_content", "h4_date", "p_abstract", 
        "td_paper_title", "td_paper_href", "content_1", "link_1",
        "content_2", "link_2", "content_about", "link_about"
    ])?;
    
    // Write sample records with content/link data
    for i in 1..=10 { // Reduced to 10 for example
        wtr.serialize(SiteRecord {
            id: i,
            title_site: format!("Site Title {}", i),
            h1_top: format!("Main Heading {}", i),
            h4_navigate_title: format!("Navigation {}", i),
            p_class: format!("Class {}", i),
            p_tutor_term: format!("Term {}", i),
            h1_title_content: format!("Content Title {}", i),
            h4_date: format!("2023-{:02}-{:02}", i%12 + 1, i%28 + 1),
            p_abstract: format!("Abstract text for item {}", i),
            td_paper_title: format!("Paper {}", i),
            td_paper_href: format!("paper_{}.pdf", i),
            content_1: format!("Introduction {}", i),
            link_1: format!("page-{}.html#intro", i),
            content_2: format!("Details {}", i),
            link_2: format!("page-{}.html#details", i),
            content_about: format!("About {}", i),
            link_about: format!("page-{}.html#about", i),
        })?;
    }
    
    wtr.flush()?;
    Ok(())
}