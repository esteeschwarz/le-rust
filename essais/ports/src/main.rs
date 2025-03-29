use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use chrono::{NaiveDateTime, Utc, TimeZone};
use regex::Regex;

#[derive(Deserialize)]
struct UserRequest {
    name: String,
    password: Option<String>,
    master_password: Option<String>,
    new_password: Option<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
    request_password: bool,
    allow_creation: bool,
}

struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Failed to open database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS meta (
                user TEXT PRIMARY KEY,
                pwkey TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create table");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

#[post("/hello")]
async fn hello(req: web::Json<UserRequest>, db: web::Data<Arc<Database>>) -> impl Responder {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT pwkey FROM meta WHERE user = ?1").unwrap();
    let mut rows = stmt.query(params![req.name.clone()]).unwrap();
    
    if let Some(row) = rows.next().unwrap() {
        let stored_pw: String = row.get(0).unwrap();
        if let Some(provided_pw) = &req.password {
            if provided_pw == &stored_pw {
                return HttpResponse::Ok().json(Response {
                    message: "Welcome!".to_string(),
                    request_password: false,
                    allow_creation: false,
                });
            }
        }
        return HttpResponse::Ok().json(Response {
            message: "Incorrect password".to_string(),
            request_password: true,
            allow_creation: false,
        });
    }
    
    HttpResponse::Ok().json(Response {
        message: "User not found".to_string(),
        request_password: false,
        allow_creation: true,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(Database::new("../../../../idsdb.db"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(hello)
    })
    .bind(("0.0.0.0", 4174))?
    .run()
    .await
}
