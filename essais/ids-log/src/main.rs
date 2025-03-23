use actix_cors::Cors; // Import the CORS middleware
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use chrono::{Local, DateTime, FixedOffset};

/// wt login
//use actix_web::{web, HttpResponse, Responder};
//use rusqlite::{params, Connection};
//use serde::{Deserialize, Serialize};
//use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    table_name: String,
    password: String,
}

async fn login(
    login_data: web::Json<LoginRequest>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &login_data.table_name;
    let password = &login_data.password;

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

    match create_table(&conn, table_name, password) {
        Ok(_) => HttpResponse::Ok().body("Table created successfully!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create table"),
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let conn = Connection::open("main.db").unwrap();
//     init_db(&conn).unwrap();

//     let db = web::Data::new(Mutex::new(conn));

//     HttpServer::new(move || {
//         App::new()
//             .app_data(db.clone())
//             .route("/login", web::post().to(login))
//             .route("/create_db", web::post().to(create_db))
//     })
//     .bind("127.0.0.1:5000")?
//     .run()
//     .await
// }
/// 

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

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
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
fn init_db_dep(conn: &Connection) -> rusqlite::Result<()> {
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
async fn save_data(
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

// Fetch all data from the database
async fn fetch_data_login(
    db: web::Data<Mutex<Connection>>,
    login_data: web::Json<LoginRequest>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let table_name = &login_data.table_name;
    println!("main.rs.fetch::Fetching data for table: {}", table_name);

    let mut stmt = conn
        .prepare(&format!("SELECT id, field1, field2, field3, field4, field5, field6, field7, field8, field9, timestamp FROM {}",table_name))
        .unwrap();
    // let mut stmt = conn
    //     .prepare("SELECT id, field1, field2, field3, field4, field5, field6, field7, field8, field9, timestamp FROM entries")
    //     .unwrap();
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
           // .query_map([], |row| {
                // Fetch the UTC timestamp from the database
                // let utc_timestamp: String = row.get(10)?;
    
                // // Parse the UTC timestamp
                // let utc_time = DateTime::parse_from_rfc3339(&utc_timestamp).unwrap();
    
                // // Convert to CET (UTC+1 or UTC+2 depending on DST)
                // let cet_offset = FixedOffset::east(1 * 3600); // CET is UTC+1
                // let cet_time = utc_time.with_timezone(&cet_offset);
    
                // // Create the Entry struct with the CET timestamp
                // Ok(Entry {
                //     id: row.get(0)?,
                //     field1: row.get(1)?,
                //     field2: row.get(2)?,
                //     field3: row.get(3)?,
                //     field4: row.get(4)?,
                //     field5: row.get(5)?,
                //     field6: row.get(6)?,
                //     field7: row.get(7)?,
                //     field8: row.get(8)?,
                //     field9: row.get(9)?,
                //     timestamp: cet_time.to_rfc3339(), // Store the CET timestamp as a string
                // })
            })
       // })
        .unwrap()
        .collect::<Result<Vec<Entry>, _>>()
        .unwrap();
    HttpResponse::Ok().json(entries)
}

async fn fetch_data(db: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = db.lock().unwrap();
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
           // .query_map([], |row| {
                // Fetch the UTC timestamp from the database
                // let utc_timestamp: String = row.get(10)?;
    
                // // Parse the UTC timestamp
                // let utc_time = DateTime::parse_from_rfc3339(&utc_timestamp).unwrap();
    
                // // Convert to CET (UTC+1 or UTC+2 depending on DST)
                // let cet_offset = FixedOffset::east(1 * 3600); // CET is UTC+1
                // let cet_time = utc_time.with_timezone(&cet_offset);
    
                // // Create the Entry struct with the CET timestamp
                // Ok(Entry {
                //     id: row.get(0)?,
                //     field1: row.get(1)?,
                //     field2: row.get(2)?,
                //     field3: row.get(3)?,
                //     field4: row.get(4)?,
                //     field5: row.get(5)?,
                //     field6: row.get(6)?,
                //     field7: row.get(7)?,
                //     field8: row.get(8)?,
                //     field9: row.get(9)?,
                //     timestamp: cet_time.to_rfc3339(), // Store the CET timestamp as a string
                // })
            })
       // })
        .unwrap()
        .collect::<Result<Vec<Entry>, _>>()
        .unwrap();
    HttpResponse::Ok().json(entries)
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
    let cors = Cors::permissive(); // Allow all origins, methods, and headers

    App::new()
        .wrap(cors) // Apply CORS middleware
        .app_data(db.clone())
        .route("/test", web::get().to(test))
        .route("/save", web::post().to(save_data))
        // .route("/data", web::get().to(fetch_data))
        .route("/data", web::post().to(fetch_data_login))
        .route("/login", web::post().to(login))
        .route("/create_table", web::post().to(create_table_endpoint))
})
.bind("127.0.0.1:4173")?
.run()
.await
}