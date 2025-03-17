use actix_cors::Cors; // Import the CORS middleware
use actix_web::{web, App, HttpResponse, HttpServer, Responder, http};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

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
    HttpResponse::Ok().body("Server is running!")
}

// Save data to the database
async fn save_data(
    form_data: web::Json<FormData>,
    db: web::Data<Mutex<Connection>>,
) -> impl Responder {
    let conn = db.lock().unwrap();
    let query = "INSERT INTO entries (field1, field2, field3, field4, field5, field6, field7, field8, field9)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)";
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
        })
        .unwrap()
        .collect::<Result<Vec<Entry>, _>>()
        .unwrap();
    HttpResponse::Ok().json(entries)
}



async fn handle_options() -> impl Responder {
    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "localhost")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Credentials", "true")
        .header("Vary", "Origin") // Disable caching
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("localhost")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            // .route("/test", web::get().to(|| HttpResponse::Ok().body("Server is running!")))
            .route("/test", web::get().to(test))
            .route("/save", web::post().to(save_data))
            .route("/data", web::get().to(fetch_data))
            // .route("/save", web::options().to(handle_options)) // Explicitly handle OPTIONS
            // .route("/data", web::options().to(handle_options)) // Explicitly handle OPTIONS
            // .route("/save", http::Method::to(handle_options))
            // .route("/data", http::Method::to(handle_options))

            // .route("/data", web::method(Method::OPTIONS).to(handle_options))
            // use           actix_web::http::Method

    })
    .bind("0.0.0.0:4173")?
    .run()
    .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Initialize SQLite database
//     let conn = Connection::open("database.db").unwrap();
//     init_db(&conn).unwrap();

//     // Wrap the database connection in a Mutex for thread safety
//     let db = web::Data::new(Mutex::new(conn));

//     // // Start the Actix Web server
//     // HttpServer::new(move || {
//     //     // Configure CORS
//     //     let cors = Cors::default()
//     //         .allowed_origin("http://localhost:5000/save") // Allow requests from this origin
//     //         .allowed_origin("http://localhost:5000/data") // Allow requests from this origin

//     //         .allowed_methods(vec!["GET", "POST"]) // Allow specific HTTP methods
//     //         .allowed_headers(vec!["Content-Type"]) // Allow specific headers
//     //         .max_age(3600); // Cache preflight response for 1 hour

//     //     App::new()
//     //         .wrap(cors) // Apply CORS middleware
//     //         .app_data(db.clone())
//     //         .route("/test", web::get().to(test))
//     //         .route("/save", web::post().to(save_data))
//     //         .route("/data", web::get().to(fetch_data))
//     // })
//     // .bind("127.0.0.1:5000")?
//     // .run()
//     // .await
// //}

//   // Start the Actix Web server
//   HttpServer::new(move || {
//     // Configure CORS to allow all origins
//     let cors = Cors::permissive(); // Allow all origins, methods, and headers
// // Configure CORS
//         // let cors = Cors::default()
//         // //     .allowed_origin(vec![
//         // //     "http://localhost:5000/save", // Allow requests from localhost
//         // //     "http://localhost:5000/data",
//         // //     "http://mini12:5000/save",
//         // //     "http://mini12:5000/data",
//         // //     "http://mini12:5000/test" // Allow requests from www.example.com
//         // // ])
//         //     .allowed_methods(vec!["GET", "POST", "OPTIONS"]) // Allow specific HTTP methods
//         //     // .allowed_headers(vec!["Content-Type"]) // Allow specific headers
//         //     .supports_credentials() // Allow credentials (if needed)
//         //     .max_age(3600) // Cache preflight response for 1 hour
//         //     .allowed_origin("http://localhost:5000/save") // Allow requests from this origin
//         //     .allowed_origin("http://localhost:5000/data") // Allow requests from this origin
//         //     .allowed_origin("http://mini12:5000/save") // Allow requests from this origin
//         //     .allowed_origin("http://mini12:5000/data") // Allow requests from this origin
//         //     .allowed_origin("http://mini12:5000/test"); // Allow requests from this origin

//     App::new()
//         .wrap(cors) // Apply CORS middleware
//         .app_data(db.clone())
//         .route("/test", web::get().to(test))
//         .route("/save", web::post().to(save_data))
//         .route("/data", web::get().to(fetch_data))
// })
// // .bind("localhost:5000")?
// .bind("mini12:5000")?
// .run()
// .await
// }