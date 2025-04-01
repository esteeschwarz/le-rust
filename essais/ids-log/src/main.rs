use actix_cors::Cors; // Import the CORS middleware
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use rusqlite::{params, Connection, Error};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use chrono::{NaiveDateTime, DateTime, FixedOffset, Utc};
use regex::Regex;
use rand::{distributions::Alphanumeric, Rng};
use rand::distributions::Uniform;



// Define the structure of a database entry
#[derive(Serialize, Deserialize)]
struct Entry {
    id: i32,
    field1: String,
    field2: String,
    field3: String,
    field4: String,
    field5: String,
    field6: String,
    field7: String,
    field8: String,
    field9: String,
    timestamp: String,
}

// Define the structure for form data
#[derive(Serialize, Deserialize)]
struct FormData {
    field1: String,
    field2: String,
    field3: String,
    field4: String,
    field5: String,
    field6: String,
    field7: String,
    field8: String,
    field9: String,
}
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    table_name: String,
    password: String,
    #[serde(default = "default_masterpassword")] // Set default value
    masterpassword: String,
}
#[derive(Serialize, Deserialize,Debug)]
struct masterData {
    request: String,
    masterpassword: String,
    }
// Function to provide the default value for masterpassword
fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();

    // Generate 10 random letters
    let letters: String = (0..10)
        .map(|_| rng.sample(Alphanumeric))
        .filter(|c| c.is_ascii_alphabetic()) // Ensure only letters
        .map(|c| c as char)
        .collect();

    // Generate 5 random numbers
    let numbers: String = (0..5)
        .map(|_| rng.sample(Uniform::new(0, 10)))
        .map(|n| n.to_string())
        .collect();

    // Combine letters and numbers
    format!("{}{}", letters, numbers)
}
fn default_masterpassword() -> String {
//    "default_master_password".to_string()
    generate_random_string().to_string()

}
#[derive(Serialize, Deserialize)]
struct SaveRequest {
    data: FormData,       // Nested struct for the "data" field
    table_name: String,   // Corresponds to "table_name" in JSON
    password: String,     // Corresponds to "password" in JSON
}

async fn login(
    login_data: web::Json<LoginRequest>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &login_data.table_name;
    let password = &login_data.password;
   // let masterpassword = &login_data.masterpassword;

    match check_credentials(&conn, table_name, password) {
        Ok(true) => HttpResponse::Ok().body("Login successful!"),
        Ok(false) => HttpResponse::Unauthorized().body("Invalid credentials. Create new table?"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}


async fn create_table_endpoint(
    login_data: web::Json<LoginRequest>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &login_data.table_name;
    let password = &login_data.password;
    let masterpassword = &login_data.masterpassword;
    
    match check_create_pwd(&conn, table_name, password,masterpassword) {
        Ok(_) => {
            //HttpResponse::Ok().body("masterpassword provided, create table...");
                match create_table(&conn, table_name, password) {
                Ok(_) => HttpResponse::Ok().body("Table created successfully!"),
                Err(_) => HttpResponse::InternalServerError().body("Failed to create table"),
            }//,
           // Err(_) => HttpResponse::InternalServerError().body("Failed to create table"),

        }
        Ok(false) => HttpResponse::Unauthorized().body("Invalid master password"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
       // Err(_) => HttpResponse::InternalServerError().body("Failed to create table"),
    }
    // match create_table(&conn, table_name, password) {
    //     Ok(_) => HttpResponse::Ok().body("Table created successfully!"),
    //     Err(_) => HttpResponse::InternalServerError().body("Failed to create table"),
    // }
//}
fn init_db_dep(conn: &Connection) -> rusqlite::Result<()> {
    // Create the meta table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS meta (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            table_name TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    // Create a default table (optional)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS default_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            field1 TEXT,
            field2 TEXT,
            field3 TEXT,
            field4 TEXT,
            field5 TEXT,
            field6 TEXT,
            field7 TEXT,
            field8 TEXT,
            field9 TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    Ok(())
}    fn check_credentials(conn: &Connection, table_name: &str, password: &str) -> rusqlite::Result<bool> {
        let mut stmt = conn.prepare("SELECT password FROM meta WHERE table_name = ?1")?;
        let mut rows = stmt.query(params![table_name])?;
    
        if let Some(row) = rows.next()? {
            let stored_password: String = row.get(0)?;
            Ok(stored_password == password)
        } else {
            Ok(false)
        }
    }
    fn check_create_pwd(conn: &Connection, table_name: &str, password: &str,masterpassword:&str) -> rusqlite::Result<bool> {
        let mut stmt = conn.prepare("SELECT * FROM meta WHERE `table_name` = 'createTable'")
        .unwrap();
    println!("fetchdata");
    let entries = stmt.query_map([], |row|{
        Ok(masterData{
            request: row.get(1)?,
            masterpassword: row.get(2)?,
        })
    })
        .unwrap()
        .collect::<Result<Vec<masterData>, _>>()
        .unwrap();
    println!("{:?}", entries); //wks.
    println!("{:?}", entries[0].masterpassword); //wks.
        let mut stored_password: &String = &generate_random_string().to_string();
        println!("create pwd random before fetch db {}",stored_password);
        stored_password = &entries[0].masterpassword;

        println!("create pwd in fetch meta {}",stored_password);
        println!("master pwd provided {}",masterpassword);
      
        
            Ok(stored_password == masterpassword)
        }
    // }
    fn create_table(conn: &Connection, table_name: &str, password: &str) -> rusqlite::Result<()> {
        // Add the new table to the meta table

        conn.execute(
            "INSERT INTO meta (table_name, password) VALUES (?1, ?2)",
            params![table_name, password],
        )?;
    
        // Create a new table for the database
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    field1 TEXT,
                    field2 TEXT,
                    field3 TEXT,
                    field4 TEXT,
                    field5 TEXT,
                    field6 TEXT,
                    field7 TEXT,
                    field8 TEXT,
                    field9 TEXT,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                table_name
            ),
            [],
        )?;
    
        Ok(())
    }
/////////////////////////////////
// Initialize the SQLite database
fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            field1 TEXT,
            field2 TEXT,
            field3 TEXT,
            field4 TEXT,
            field5 TEXT,
            field6 TEXT,
            field7 TEXT,
            field8 TEXT,
            field9 TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

// Test endpoint to check if the server is running
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Server is running!\n")
}

// Save data to the database
async fn save_data_dep(
    form_data: web::Json<FormData>,
    login_data: web::Json<LoginRequest>,
    db: web::Data<Mutex<Connection>>
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &login_data.table_name;
    //console.log("save data to:");
    //console.log(table_name);

    println!("main.rs.save::Fetching data for table: {}", table_name);
    let query = &format!("INSERT INTO {} (field1, field2, field3, field4, field5, field6, field7, field8, field9)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",table_name);
    //console.log(query);
    match conn.execute(
        query,
        params![
            form_data.field1,
            form_data.field2,
            form_data.field3,
            form_data.field4,
            form_data.field5,
            form_data.field6,
            form_data.field7,
            form_data.field8,
            form_data.field9,
        ],
    ) {
        Ok(_) => HttpResponse::Ok().json("Data saved successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn save_data(
    request: web::Json<SaveRequest>, // Deserialize the JSON body into SaveRequest
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &request.table_name; // Access table_name from the request
    let password = &request.password;    // Access password from the request
    let form_data = &request.data;       // Access the nested FormData struct

    // Check credentials
    // match check_credentials(&conn, table_name, password) {
    //     Ok(true) => {
            // Save data to the table
            eprintln!("rs save data to {}", table_name);

            let query = format!(
                "INSERT INTO {} (field1, field2, field3, field4, field5, field6, field7, field8, field9)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                table_name
            );
            match conn.execute(
                &query,
                params![
                    form_data.field1,
                    form_data.field2,
                    form_data.field3,
                    form_data.field4,
                    form_data.field5,
                    form_data.field6,
                    form_data.field7,
                    form_data.field8,
                    form_data.field9,
                ],
            ) {
                Ok(_) => HttpResponse::Ok().json("Data saved successfully"),
                Err(e) => {
                    eprintln!("Failed to save data: {}", e);
                    HttpResponse::InternalServerError().body("Failed to save data")
                }
            }
        }
        fn replace_hashtags(input: &str) -> String {
            let re = Regex::new(r"#(\w+)").unwrap();
            re.replace_all(input, r#"<span class="hash">$1</span>"#).to_string()
        }
// Fetch all data from the database
async fn fetch_data_login(
    db: web::Data<Mutex<Connection>>,
    login_data: web::Json<LoginRequest>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &login_data.table_name;
    println!("main.rs.fetch::Fetching data for table: {}", table_name);

    let mut stmt = conn
        .prepare(&format!("SELECT id, field1, field2, field3, field4, field5, field6, field7, field8, field9, timestamp FROM {} ORDER BY timestamp DESC",table_name))
        .unwrap();
   

    let entries = stmt
    .query_map([], |row| {

    let utc_timestamp: String = row.get(10)?;

            // Parse the UTC timestamp without timezone
            let naive_utc_time = NaiveDateTime::parse_from_str(&utc_timestamp, "%Y-%m-%d %H:%M:%S")
                .expect("Failed to parse UTC timestamp");

            // Convert NaiveDateTime to DateTime<Utc>
            let utc_time = DateTime::<Utc>::from_utc(naive_utc_time, Utc);

            // Convert to CET (UTC+1 or UTC+2 depending on DST)
            let cet_offset = FixedOffset::east(1 * 3600); // CET is UTC+1
            let cet_time = utc_time.with_timezone(&cet_offset);


            Ok(Entry {
                id: row.get(0)?,
                // field1: row.get(1)?,
                // field2: row.get(2)?,
                // field3: row.get(3)?,
                // field4: row.get(4)?,
                // field5: row.get(5)?,
                // field6: row.get(6)?,
                // field7: row.get(7)?,
                // field8: row.get(8)?,
                // field9: row.get(9)?,
                field1: replace_hashtags(&row.get::<_, String>(1)?),
                field2: replace_hashtags(&row.get::<_, String>(2)?),
                field3: replace_hashtags(&row.get::<_, String>(3)?),
                field4: replace_hashtags(&row.get::<_, String>(4)?),
                field5: replace_hashtags(&row.get::<_, String>(5)?),
                field6: replace_hashtags(&row.get::<_, String>(6)?),
                field7: replace_hashtags(&row.get::<_, String>(7)?),
                field8: replace_hashtags(&row.get::<_, String>(8)?),
                field9: replace_hashtags(&row.get::<_, String>(9)?),
                timestamp: cet_time.to_rfc3339(), // Store the CET timestamp as a string

                // timestamp: row.get(10)?,
            })
         
            })
       // })
        .unwrap()
        .collect::<Result<Vec<Entry>, _>>()
        .unwrap();
    HttpResponse::Ok().json(entries)
}

async fn fetch_data(db: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = db.lock().unwrap();
    println!("fetchdata fun");
    let mut stmt = conn
        .prepare("SELECT id, field1, field2, field3, field4, field5, field6, field7, field8, field9, timestamp FROM entries")
        .unwrap();
    let entries = stmt
        .query_map([], |row| {
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
                timestamp: row.get(10)?,
            })
          
            })
       // })
        .unwrap()
        .collect::<Result<Vec<Entry>, _>>()
        .unwrap();
    HttpResponse::Ok().json(entries)
}
#[post("/hello")]
// Test endpoint to check if the server is running
async fn test_he() -> impl Responder {
    HttpResponse::Ok().body("Server is running!\n")
}
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
                // let cet_time = utc_time.with_timezone(&cet_offset);
                // let cet_time = utc_timestamp
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
                    // timestamp: cet_time.to_rfc3339(),
                    timestamp: row.get(10)?,

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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize SQLite database
    let conn = Connection::open("../../../../idsdatabase.db").unwrap();
//    let conn = Connection::open("database.db").unwrap();
    init_db(&conn).unwrap();

    // Wrap the database connection in a Mutex for thread safety
    let db = web::Data::new(Mutex::new(conn));

    // // Start the Actix Web server
    // HttpServer::new(move || {
    //     // Configure CORS
    //     let cors = Cors::default()
    //         .allowed_origin("http://localhost:5000/save") // Allow requests from this origin
    //         .allowed_origin("http://localhost:5000/data") // Allow requests from this origin

    //         .allowed_methods(vec!["GET", "POST"]) // Allow specific HTTP methods
    //         .allowed_headers(vec!["Content-Type"]) // Allow specific headers
    //         .max_age(3600); // Cache preflight response for 1 hour

    //     App::new()
    //         .wrap(cors) // Apply CORS middleware
    //         .app_data(db.clone())
    //         .route("/test", web::get().to(test))
    //         .route("/save", web::post().to(save_data))
    //         .route("/data", web::get().to(fetch_data))
    // })
    // .bind("127.0.0.1:5000")?
    // .run()
    // .await
//}

  // Start the Actix Web server
  HttpServer::new(move || {
    // Configure CORS to allow all origins
    // let cors = Cors::permissive(); // Allow all origins, methods, and headers
    let cors = Cors::default()
    .allow_any_origin()
    .allow_any_method()
    .allow_any_header();
    ////
    // HttpServer::new(move || {
    //     let cors = Cors::default()
    //         .allow_any_origin()
    //         .allow_any_method()
    //         .allow_any_header();
        
    //     App::new()
    //         .wrap(cors)
    //         .app_data(db.clone())
    //         .service(hello)
    // })
    // .bind(("0.0.0.0", 4174))?
    // .run()
    // .await
    ////


    App::new()
        .wrap(cors) // Apply CORS middleware
        .app_data(db.clone())
        .route("/test", web::get().to(test))
        .route("/save", web::post().to(save_data))
        // .route("/data", web::get().to(fetch_data))
        .route("/data", web::post().to(fetch_data_login))
        .route("/login", web::post().to(login))
        .route("/create_table", web::post().to(create_table_endpoint))
        //.service(hello)
        .route("/search", web::post().to(regex_search))
})
.bind(("0.0.0.0", 4173))?
// .bind("127.0.0.1:4173")?
.run()
.await
}