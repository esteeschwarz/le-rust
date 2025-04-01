use regex::Regex;
use rusqlite::{Connection, Error};

// Add this struct to hold search request data
#[derive(Serialize, Deserialize)]
struct SearchRequest {
    table_name: String,
    password: String,
    regex_pattern: String,
}

// Add this endpoint function
async fn regex_search(
    search_data: web::Json<SearchRequest>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &search_data.table_name;
    let password = &search_data.password;
    let regex_pattern = &search_data.regex_pattern;

    // First verify credentials
    match check_credentials(&conn, table_name, password) {
        Ok(true) => {
            // Compile the regex pattern
            let regex = match Regex::new(regex_pattern) {
                Ok(re) => re,
                Err(e) => return HttpResponse::BadRequest().body(format!("Invalid regex pattern: {}", e)),
            };

            // Build the query to fetch all data from the table
            let query = format!(
                "SELECT id, field1, field2, field3, field4, field5, field6, field7, field8, field9, timestamp FROM {}",
                table_name
            );

            let mut stmt = match conn.prepare(&query) {
                Ok(stmt) => stmt,
                Err(e) => {
                    eprintln!("Failed to prepare query: {}", e);
                    return HttpResponse::InternalServerError().body("Database error");
                }
            };

            let entries_result = stmt.query_map([], |row| {
                let utc_timestamp: String = row.get(10)?;
                let naive_utc_time = NaiveDateTime::parse_from_str(&utc_timestamp, "%Y-%m-%d %H:%M:%S")
                    .expect("Failed to parse UTC timestamp");
                let utc_time = DateTime::<Utc>::from_utc(naive_utc_time, Utc);
                let cet_offset = FixedOffset::east(1 * 3600);
                let cet_time = utc_time.with_timezone(&cet_offset);

                Ok(Entry {
                    id: row.get(0)?,
                    field1: row.get(1)?,
                    field2: row.get(2)?,
                    field3: row.get(3)?,
                    field4: row.get(4)?,
                    field5: row.get(5)?,
                    field6: row.get(6)?,
                    field7: row.get(7)?,
                    field8: row.get(8)?,
                    field9: row.get(9)?,
                    timestamp: cet_time.to_rfc3339(),
                })
            });

            let entries = match entries_result {
                Ok(iter) => iter.collect::<Result<Vec<Entry>, _>>(),
                Err(e) => {
                    eprintln!("Failed to execute query: {}", e);
                    return HttpResponse::InternalServerError().body("Database error");
                }
            };

            let entries = match entries {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Failed to collect entries: {}", e);
                    return HttpResponse::InternalServerError().body("Database error");
                }
            };

            // Filter entries where any field matches the regex
            let filtered_entries: Vec<Entry> = entries.into_iter()
                .filter(|entry| {
                    let fields = [
                        &entry.field1, &entry.field2, &entry.field3, 
                        &entry.field4, &entry.field5, &entry.field6,
                        &entry.field7, &entry.field8, &entry.field9
                    ];
                    
                    fields.iter().any(|field| regex.is_match(field))
                })
                .map(|mut entry| {
                    // Apply hashtag replacement to all fields
                    entry.field1 = replace_hashtags(&entry.field1);
                    entry.field2 = replace_hashtags(&entry.field2);
                    entry.field3 = replace_hashtags(&entry.field3);
                    entry.field4 = replace_hashtags(&entry.field4);
                    entry.field5 = replace_hashtags(&entry.field5);
                    entry.field6 = replace_hashtags(&entry.field6);
                    entry.field7 = replace_hashtags(&entry.field7);
                    entry.field8 = replace_hashtags(&entry.field8);
                    entry.field9 = replace_hashtags(&entry.field9);
                    entry
                })
                .collect();

            HttpResponse::Ok().json(filtered_entries)
        },
        Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}